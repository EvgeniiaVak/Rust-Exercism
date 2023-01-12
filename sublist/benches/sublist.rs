use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs;
use sublist::{sublist, Method};

use pprof::criterion::{Output, PProfProfiler};

fn bench_sublist(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sublist");
    group.sample_size(10);
    let methods = vec![Method::Rayon, Method::Sequential, Method::Threads];

    for i in [100_000_000] {
        // random chars
        let rand = |i| {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(i)
                .map(char::from)
                .collect::<Vec<char>>()
        };

        for method in methods.iter() {
            let a = rand(i);
            let b = rand(i / 4);
            let method = method.clone();
            group.bench_function(BenchmarkId::new(format!("{method:?}"), i), |bencher| {
                bencher.iter(|| sublist(&a, &b, method))
            });
        }
    }

    group.finish();

    // copy the result comparison image
    match fs::copy(
        "target/criterion/Sublist/report/violin.svg",
        "benches/test_result.svg",
    ) {
        Ok(_) => println!("Copied the result comparison image to benches/test_result.svg"),
        Err(e) => println!("Failed to copy the result comparison image: {}", e),
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_sublist
);
criterion_main!(benches);
