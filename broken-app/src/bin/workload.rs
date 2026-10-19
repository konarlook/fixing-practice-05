use broken_app::{algo, average_positive, leak_buffer, normalize, sum_even};
use std::hint::black_box;

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

fn main() {
    let ints: Vec<i64> = (0..1_000_000).map(|i| i - 500_000).collect();
    let bytes: Vec<u8> = (0..1_000_000_u32).map(|i| (i % 7) as u8).collect();
    let text = " Hello\tWorld \n".repeat(10_000);
    let dedup_input = pseudo_random(20_000, 10_000);

    for _ in 0..100 {
        black_box(sum_even(black_box(&ints)));
        black_box(leak_buffer(black_box(&bytes)));
        black_box(average_positive(black_box(&ints)));
    }
    for _ in 0..10 {
        black_box(normalize(black_box(&text)));
    }
    black_box(algo::slow_fib(black_box(35)));
    black_box(algo::slow_dedup(black_box(&dedup_input)));
}