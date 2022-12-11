#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
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

    let is_superlist = a.windows(b.len()).any(|w| w == b);
    let is_sublist = b.windows(a.len()).any(|w| w == a);
    match (is_sublist, is_superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
