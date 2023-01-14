use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sublist::{sublist, Method};

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
    config = Criterion::default();
    targets = bench_sublist
);
criterion_main!(benches);
