use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sublist::{sublist, Comparison, Method};

fn test_sublist<T: PartialEq + Sync>(expected_comparison: Comparison, a: &[T], b: &[T]) {
    assert_eq!(expected_comparison, sublist(a, b, Method::Sequential));
    assert_eq!(expected_comparison, sublist(a, b, Method::Rayon));
    assert_eq!(expected_comparison, sublist(a, b, Method::Threads));
}

fn test_large_sublist(method: Method) {
    let i = 100_000_000;

    // random chars
    let rand = |i| {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(i)
            .map(char::from)
            .collect::<Vec<char>>()
    };

    let a = rand(i);
    let b = rand(i / 4);

    sublist(&a, &b, method);
}

#[test]
#[ignore]
fn large_threads() {
    test_large_sublist(Method::Threads);
}

#[test]
#[ignore]
fn large_rayon() {
    test_large_sublist(Method::Rayon);
}

#[test]
#[ignore]
fn large_sequential() {
    test_large_sublist(Method::Sequential);
}

#[test]
fn empty_equals_empty() {
    let v: &[u32] = &[];

    test_sublist(Comparison::Equal, v, v);
}

#[test]
fn test_empty_is_a_sublist_of_anything() {
    test_sublist(Comparison::Sublist, &[], &['a', 's', 'd', 'f']);
}

#[test]
fn test_anything_is_a_superlist_of_empty() {
    test_sublist(Comparison::Superlist, &['a', 's', 'd', 'f'], &[]);
}

#[test]
fn test_1_is_not_2() {
    test_sublist(Comparison::Unequal, &[1], &[2]);
}

#[test]
fn test_compare_larger_equal_lists() {
    use std::iter::repeat;

    let v: Vec<char> = repeat('x').take(1000).collect();

    test_sublist(Comparison::Equal, &v, &v);
}

#[test]
fn test_sublist_at_start() {
    test_sublist(Comparison::Sublist, &[1, 2, 3], &[1, 2, 3, 4, 5]);
}

#[test]
fn sublist_in_middle() {
    test_sublist(Comparison::Sublist, &[4, 3, 2], &[5, 4, 3, 2, 1]);
}

#[test]
fn sublist_at_end() {
    test_sublist(Comparison::Sublist, &[3, 4, 5], &[1, 2, 3, 4, 5]);
}

#[test]
fn partially_matching_sublist_at_start() {
    test_sublist(Comparison::Sublist, &[1, 1, 2], &[1, 1, 1, 2]);
}

#[test]
fn sublist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();

    test_sublist(Comparison::Sublist, &[3, 4, 5], &huge);
}

#[test]
fn huge_sublist_not_in_huge_list() {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    test_sublist(Comparison::Unequal, &v1, &v2);
}

#[test]
fn superlist_at_start() {
    test_sublist(Comparison::Superlist, &[1, 2, 3, 4, 5], &[1, 2, 3]);
}

#[test]
fn superlist_in_middle() {
    test_sublist(Comparison::Superlist, &[5, 4, 3, 2, 1], &[4, 3, 2]);
}

#[test]
fn superlist_at_end() {
    test_sublist(Comparison::Superlist, &[1, 2, 3, 4, 5], &[3, 4, 5]);
}

#[test]
fn second_list_missing_element_from_first_list() {
    test_sublist(Comparison::Unequal, &[1, 2, 3], &[1, 3]);
}

#[test]
fn superlist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();

    test_sublist(Comparison::Superlist, &huge, &[3, 4, 5]);
}

#[test]
fn recurring_values_sublist() {
    test_sublist(
        Comparison::Sublist,
        &[1, 2, 1, 2, 3],
        &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1],
    );
}

#[test]
fn recurring_values_unequal() {
    test_sublist(
        Comparison::Unequal,
        &[1, 2, 1, 2, 3],
        &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1],
    );
}
