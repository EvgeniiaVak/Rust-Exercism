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

fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|w| w == b)
}

fn is_superlist_threads<T: PartialEq + Sync>(a: &[T], b: &[T]) -> bool {
    //! Strateforwardly send each window comparison to a new thread (no pooling).

    let a = Arc::new(a);
    let b = Arc::new(b);

    // scoped threads as in https://stackoverflow.com/a/32751956/9076659
    thread::scope(|_| {
        let (tx, rx) = mpsc::channel();

        for window in a.windows(b.len()) {
            // create a new transmitter for each thread
            let tx = tx.clone();

            let b = *b.clone();
            let window = window.clone();

            // FIXME: error[E0521]: borrowed data escapes outside of function
            // maybe relevant https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541
            thread::spawn(move || {
                let are_equal = window == b;
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

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal;
    }

    if a.len() == 0 {
        return Comparison::Sublist;
    }

    if b.len() == 0 {
        return Comparison::Superlist;
    }

    let superlist = is_superlist(a, b);
    let sublist = is_superlist(b, a);

    match (sublist, superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
