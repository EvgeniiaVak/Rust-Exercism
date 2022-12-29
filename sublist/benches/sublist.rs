use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use sublist::{sublist, Method};
use std::fs;


static LARGE_STRING: &str = include_str!("large_input.txt");
static SUBSTRING: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn bench_sublist(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sublist");
    let methods = vec![Method::Sequential, Method::Rayon, Method::Threads];

    
    let b = SUBSTRING.as_bytes();

    for i in [100, LARGE_STRING.len() / 2, LARGE_STRING.len()] {
        let a = LARGE_STRING[..i].as_bytes();

        for method in methods.iter() {
            let method = method.clone();
            group.throughput(Throughput::Bytes(i as u64));
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}, {:?}", method, i), i),
                &(a, b, method),
                |bencher, (haystack, needle, method)| {
                    bencher.iter(|| sublist(*haystack, *needle, *method))
                },
            );
        }
        
    }

    group.finish();

    // copy the result comparison image
    match fs::copy("target/criterion/Sublist/report/violin.svg", "benches/test_result.svg") {
        Ok(_) => println!("Copied the result comparison image to benches/test_result.svg"),
        Err(e) => println!("Failed to copy the result comparison image: {}", e),
    }
}

criterion_group!(benches, bench_sublist);
criterion_main!(benches);



