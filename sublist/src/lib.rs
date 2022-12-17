use rayon::prelude::*;
use std::marker::Sync;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

#[allow(dead_code)]
fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|w| w == b)
}

#[allow(dead_code)]
fn is_superlist_rayon<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Parallelize the window comparisons using rayon.

    a.par_windows(b.len()).any(|w| w == b)
}

fn is_superlist_threads<T: PartialEq + Sync + Send>(a: &[T], b: &[T]) -> bool {
    //! Strateforwardly send each window comparison to a new thread (no pooling).

    let a = Arc::new(a);
    let b = Arc::new(b);

    // scoped threads as in https://stackoverflow.com/a/32751956/9076659
    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();

        for window in a.windows(b.len()) {
            // create a new transmitter for each thread
            let tx = tx.clone();

            let b = b.clone();

            s.spawn(move || {
                let are_equal = *b == window;
                tx.send(are_equal);
            });
        }

        for received in rx {
            if received {
                // TODO: kill all other threads?
                return true;
            }
        }

        false
    }) // no semicolon to return result
}

pub fn sublist<T: PartialEq + Sync + Send>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal;
    }

    if a.len() == 0 {
        return Comparison::Sublist;
    }

    if b.len() == 0 {
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
