use advent_of_code_2020::{get_day, get_days};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    for i in get_days() {
        c.bench_function(&format!("day {}", i), |b| b.iter(|| get_day(i)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
