use rayon::prelude::*;
use std::marker::Sync;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, available_parallelism};

pub fn sublist<T: PartialEq + Sync>(a: &[T], b: &[T], method: Method) -> Comparison {
    if a == b {
        return Comparison::Equal;
    }

    if a.is_empty() {
        return Comparison::Sublist;
    }

    if b.is_empty() {
        return Comparison::Superlist;
    }

    let f = match method {
        Method::Sequential => is_superlist,
        Method::Rayon => is_superlist_rayon,
        Method::Threads => is_superlist_threads,
    };

    let superlist = f(a, b);
    let sublist = f(b, a);

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

#[derive(Debug, Copy, Clone)]
pub enum Method {
    Sequential,
    Rayon,
    Threads,
}

fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    //! Sequentially compare the windows of a to b.

    a.windows(b.len()).any(|w| w == b)
}

fn is_superlist_rayon<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Parallelize the window comparisons using rayon.

    a.par_windows(b.len()).any(|w| w == b)
}

fn is_superlist_threads<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Parallelize the window comparisons using std-only tools.

    let num_threads = match available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 4,
    };

    let found_match = Arc::new(AtomicBool::new(false));
    let a_len = a.len();
    let b_len = b.len();

    thread::scope(|s| {
        for thread_i in 0..num_threads {
            let found_match = Arc::clone(&found_match);

            s.spawn(move || {
                let mut iteration = 0;
                loop {
                    let start = iteration * num_threads + thread_i;
                    let end = start + b_len;
                    if end > a_len {
                        break;
                    }
                    let window = &a[start..end];

                    if b == window {
                        found_match.store(true, Ordering::Relaxed);
                        break;
                    };

                    iteration += 1;
                }
            });
        }
    });

    found_match.load(Ordering::Relaxed)
}
