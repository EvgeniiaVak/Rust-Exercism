use std::marker::Sync;
use std::sync::mpsc;
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
    let (tx, rx) = mpsc::channel();

    let windows = a.windows(b.len()).collect::<Vec<&[T]>>();

    for window in windows {
        // create a new transmitter for each thread
        let tx = tx.clone();

        // TODO: is this clone necessary?
        let window = window.clone();
        let b = b.clone();

        // FIXME: error[E0521]: borrowed data escapes outside of function
        thread::spawn(move || {
            let are_equal = window == b;
            tx.send(are_equal).unwrap();
        });
    }

    for received in rx {
        if received {
            // TODO: kill all other threads?
            return true;
        }
    }

    false
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
