use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sublist::{is_superlist, is_superlist_rayon};

type Input = [&'static str; 3];

static SMALL_FOUND: Input = ["SMALL_FOUND", "aaaa_aaaa_bbbb_aaaa_aaaa", "bbbb"];

static SMALL_NOT_FOUND: Input = ["SMALL_NOT_FOUND", "aaaa_aaaa_bbbb_aaaa_aaaa", "cccc"];

// mostly lines of many a's, with one line of b's in the middle.
static LARGE_INPUT: &str = include_str!("large_input.txt");

static LARGE_FOUND: Input = [
    "LARGE_FOUND",
    LARGE_INPUT,
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
];

static LARGE_NOT_FOUND: Input = [
    "LARGE_NOT_FOUND",
    LARGE_INPUT,
    "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
];

static INPUTS: [Input; 4] = [SMALL_FOUND, SMALL_NOT_FOUND, LARGE_FOUND, LARGE_NOT_FOUND];

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_superlist");
    for input in INPUTS {
        group.bench_with_input(
            BenchmarkId::new("serial", input[0]),
            &input,
            |b, i: &Input| b.iter(|| is_superlist(i[1].as_bytes(), i[2].as_bytes())),
        );

        group.bench_with_input(
            BenchmarkId::new("rayon", input[0]),
            &input,
            |b, i: &Input| b.iter(|| is_superlist_rayon(i[1].as_bytes(), i[2].as_bytes())),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
