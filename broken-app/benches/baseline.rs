use broken_app::{algo, sum_even};
use std::hint::black_box;
use std::time::Instant;

fn time_it<T>(label: &str, mut f: impl FnMut() -> T) {
    let start = Instant::now();
    black_box(f());
    println!("{label}: {:?}", start.elapsed());
}

fn main() {
    let data: Vec<i64> = (0..50_000).collect();
    let fib_n = 32;
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    
    for _ in 0..3 {
        time_it("sum_even", || sum_even(black_box(&data)));
        time_it("slow_fib", || algo::slow_fib(black_box(fib_n)));
        time_it("slow_dedup", || algo::slow_dedup(black_box(&dedup_data)));
    }
}
