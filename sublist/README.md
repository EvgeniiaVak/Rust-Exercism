# Sublist

Based on [Sublist on Exercism's Rust Track](https://exercism.org/tracks/rust/exercises/sublist) but 
with an additional challenge - exploring Rust multithreading capabilities for CPU bound tasks.

## Performance

Here is the comparison of three implementations: 

1. Sequential - just comparing two lists in a single thread.
2. Rayon - using [Rayon](https://github.com/rayon-rs/rayon)'s parallel iterators to compare two lists in parallel.
3. Threads - using Rust's `std::thread` to compare two lists in parallel.

![Performance Test](benches/violin.svg)