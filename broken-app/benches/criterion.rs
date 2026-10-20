use broken_app::{algo, average_positive, leak_buffer, sum_even};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

fn pseudo_random(len: usize, modulo: u64) -> Vec<u64> {
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x % modulo
        })
        .collect()
}

fn bench_sum_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_even");
    for size in [1_000usize, 1_000_000] {
        let data: Vec<i64> = (0..size as i64).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, d| {
            b.iter(|| sum_even(black_box(d)))
        });
    }
    group.finish();
}

fn bench_leak_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("leak_buffer");
    for size in [64usize, 4_096, 1_048_576] {
        let data: Vec<u8> = (0..size).map(|i| (i % 7) as u8).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, d| {
            b.iter(|| leak_buffer(black_box(d)))
        });
    }
    group.finish();
}

fn bench_average_positive(c: &mut Criterion) {
    let mut group = c.benchmark_group("average_positive");
    for size in [1_000usize, 1_000_000] {
        let data: Vec<i64> = (0..size as i64).map(|i| i - size as i64 / 2).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, d| {
            b.iter(|| average_positive(black_box(d)))
        });
    }
    group.finish();
}

fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("fib");
    for n in [20u64, 32] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| algo::slow_fib(black_box(n)))
        });
    }
    group.finish();
}

fn bench_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));
    for size in [1_000usize, 5_000, 20_000] {
        let data = pseudo_random(size, (size / 2) as u64);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, d| {
            b.iter(|| algo::slow_dedup(black_box(d)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_sum_even,
    bench_leak_buffer,
    bench_average_positive,
    bench_fib,
    bench_dedup
);
criterion_main!(benches);
