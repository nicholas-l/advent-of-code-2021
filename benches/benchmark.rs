use advent_of_code_2021::{get_data, get_day, get_days};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    for i in get_days() {
        let (day1, day2, path) = get_day(i);
        c.bench_function(&format!("day {} A", i), |b| {
            b.iter(|| {
                day1(get_data(&path));
            })
        });

        c.bench_function(&format!("day {} B", i), |b| {
            b.iter(|| {
                day2(get_data(&path));
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
