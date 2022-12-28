use rayon::prelude::*;
use std::collections::VecDeque;
use std::marker::Sync;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, available_parallelism};

// https://exercism.org/tracks/rust/exercises/sublist
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

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

#[allow(dead_code)]
pub fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    //! Sequentially compare the windows of a to b.

    a.windows(b.len()).any(|w| w == b)
}

#[allow(dead_code)]
pub fn is_superlist_rayon<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Parallelize the window comparisons using rayon.

    a.par_windows(b.len()).any(|w| w == b)
}

pub fn is_superlist_threads<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Parallelize the window comparisons using std-only tools.

    let b = Arc::new(b);
    let num_threads = match available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 4 as usize,
    };

    let tasks = Arc::new(Mutex::new(VecDeque::new()));
    let found_match = Arc::new(AtomicBool::new(false));

    // create the tasks and put them in one queue
    for window in a.windows(b.len()) {
        let b = b.clone();
        let found_match = Arc::clone(&found_match);

        tasks.lock().unwrap().push_back(move || {
            if *b == window {
                found_match.store(true, Ordering::Relaxed);
            };
        });
    }

    // spawn the threads and give them the tasks
    thread::scope(|s| {
        for _ in 0..num_threads {
            let tasks = Arc::clone(&tasks);
            let found_match = Arc::clone(&found_match);

            s.spawn(move || loop {
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
    });

    found_match.load(Ordering::Relaxed)
}
