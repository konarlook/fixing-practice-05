pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

pub fn slow_fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut prev, mut cur) = (0_u64, 1_u64);
    for _ in 1..n {
        (prev, cur) = (cur, prev + cur);
    }
    cur
}
