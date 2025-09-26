// Benchmark Template
// Copy into an exercise directory as `benches/<exercise>_bench.rs` and run with:
// cargo bench
// Requires adding to Cargo.toml:
// [dev-dependencies]
// criterion = "0.5"
// [[bench]]
// name = "<exercise>_bench"
// harness = false

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import the function(s) you want to benchmark from the crate
// use <exercise_crate_name>::target_function;

fn baseline_example(c: &mut Criterion) {
    c.bench_function("baseline_example", |b| {
        b.iter(|| {
            // Replace with real call
            let mut acc = 0u64;
            for i in 0..10_000 { acc = acc.wrapping_add(i); }
            black_box(acc);
        })
    });
}

criterion_group!(benches, baseline_example);
criterion_main!(benches);
