use rayon::prelude::*;
use std::collections::VecDeque;
use std::marker::Sync;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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

type Job = Box<dyn FnOnce() + Send>;

pub fn is_superlist_threads<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Create threads and task queue to parallelize the window comparisons.

    let b = Arc::new(b);
    let num_threads = match available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 4 as usize,
    };

    // FIXME: lifetime may not live long enough
    let tasks: Arc<Mutex<VecDeque<Job>>> = Arc::new(Mutex::new(VecDeque::new()));
    let found_match = Arc::new(AtomicBool::new(false));

    for window in a.windows(b.len()) {
        let b = b.clone();
        let found_match = Arc::clone(&found_match);

        tasks.lock().unwrap().push_back(Box::new(move || {
            if *b == window {
                found_match.store(true, Ordering::Relaxed);
            };
        }));
    }

    for _ in 0..num_threads {
        let tasks = Arc::clone(&tasks);
        let found_match = Arc::clone(&found_match);

        thread::spawn(move || loop {
            let task = tasks.lock().unwrap().pop_front();

            match task {
                Some(task) => {
                    task();
                    if found_match.load(Ordering::Relaxed) {
                        break;
                    }
                }
                None => break,
            }
        });
    }

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

    let superlist = is_superlist_threads(a, b);
    let sublist = is_superlist_threads(b, a);

    match (sublist, superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
