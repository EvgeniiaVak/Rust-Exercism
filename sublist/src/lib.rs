use rayon::prelude::*;
use std::marker::Sync;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, available_parallelism};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

#[allow(dead_code)]
pub fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|w| w == b)
}

#[allow(dead_code)]
pub fn is_superlist_rayon<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Parallelize the window comparisons using rayon.

    a.par_windows(b.len()).any(|w| w == b)
}

#[allow(dead_code)]
pub fn is_superlist_threads<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Strateforwardly send each window comparison to a new thread (no pooling).

    let a = Arc::new(a);
    let b = Arc::new(b);

    // scoped threads as in https://stackoverflow.com/a/32751956/9076659
    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();
        let mut thread_counter = 0;

        for window in a.windows(b.len()) {
            thread_counter += 1;
            // create a new transmitter for each thread
            let tx = tx.clone();

            let b = b.clone();

            s.spawn(move || {
                // TODO: kill the thread if we already found a match
                let are_equal = *b == window;
                tx.send(are_equal).unwrap();
            });
        }

        for _ in 0..thread_counter {
            if rx.recv() == Ok(true) {
                return true;
            }
        }

        false
    }) // no semicolon to return result
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker<'s> {
    id: usize,
    thread: thread::ScopedJoinHandle<'s, ()>,
}

impl<'s> Worker<'s> {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
        scope: &'s thread::Scope<'s, '_>,
    ) -> Worker<'s> {
        let thread = scope.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}

pub struct ScopedThreadPool<'s> {
    max_threads: usize,
    workers: Vec<Worker<'s>>,
    sender: mpsc::Sender<Job>,
}

impl<'s> ScopedThreadPool<'s> {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize, scope: &'s thread::Scope<'s, '_>) -> ScopedThreadPool<'s> {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver), scope));
        }

        ScopedThreadPool {
            max_threads: size,
            workers,
            sender,
        }
    }

    fn execute<F: FnOnce() + Send + 'static>(&mut self, f: F) {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

fn is_superlist_threadpool<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Execute window comparisons within a thread pool.
    //!
    //! Based on https://doc.rust-lang.org/book/ch20-02-multithreaded.html#improving-throughput-with-a-thread-pool

    let a = Arc::new(a);
    let b = Arc::new(b);

    let found_match = Arc::new(AtomicBool::new(false));

    let num_threads = match available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 4 as usize,
    };

    thread::scope(|scope| {
        let mut pool = ScopedThreadPool::new(num_threads, scope);

        for window in a.windows(b.len()) {
            // exit early if we've already found a match
            if found_match.load(Ordering::Relaxed) {
                return;
            };

            let found_match = Arc::clone(&found_match);
            let b = Arc::clone(&b);

            // FIXME: borrowed data escapes outside of function
            pool.execute(move || {
                // exit early if we've already found a match
                if found_match.load(Ordering::Relaxed) {
                    return;
                };
                if *b == window {
                    found_match.store(true, Ordering::Relaxed);
                }
            });
        }

        // TODO: wait for all jobs to finish and shut down the thread pool
    });

    found_match.load(Ordering::Relaxed)
}

pub fn sublist<T: PartialEq + Sync>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal;
    }

    if a.is_empty() {
        return Comparison::Sublist;
    }

    if b.is_empty() {
        return Comparison::Superlist;
    }

    let superlist = is_superlist_threadpool(a, b);
    let sublist = is_superlist_threadpool(b, a);

    match (sublist, superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
