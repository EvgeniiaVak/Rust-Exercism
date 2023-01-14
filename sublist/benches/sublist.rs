use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs;
use sublist::{sublist, Method};

use pprof::criterion::{Output, PProfProfiler};

fn bench_sublist(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sublist");
    group.sample_size(10);
    let methods = vec![Method::Rayon, Method::Sequential, Method::Threads];
    let i = 100_000_000;

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
        group.bench_function(
            BenchmarkId::new("is_sublist", format!("{method:?}")),
            |bencher| bencher.iter(|| sublist(&a, &b, method)),
        );
    }

    group.finish();
}

criterion_group!(
    name = benches;
    // FIXME: this fails on macos
    // command: cargo bench --bench sublist -- --profile-time=5
    // error: process didn't exit successfully: ... (signal: 10, SIGBUS: access to undefined memory)
    // sometimes it finishes, but the results look weird
    config = Criterion::default().with_profiler(PProfProfiler::new(1, Output::Flamegraph(None)));
    targets = bench_sublist
);
criterion_main!(benches);
