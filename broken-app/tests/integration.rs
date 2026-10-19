use broken_app::concurrency::reset_counter;
use broken_app::{algo, leak_buffer, normalize, sum_even};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), 6);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn dedup_preserves_uniques() {
    let uniq = algo::slow_dedup(&[5, 5, 1, 2, 2, 3]);
    assert_eq!(uniq, vec![1, 2, 3, 5]); // порядок и состав важны
}

#[test]
fn fib_small_numbers() {
    assert_eq!(algo::slow_fib(10), 55);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    // Ожидается (5 + 15) / 2 = 10, но текущая реализация делит на все элементы.
    assert!((broken_app::average_positive(&nums) - 10.0).abs() < f64::EPSILON);
}

#[test]
fn use_after_free_returns_double() {
    let v = unsafe { broken_app::use_after_free() };
    assert_eq!(v, 84)
}

#[test]
fn race_increment_counts_every_increment() {
    use broken_app::concurrency::{race_increment, read_after_sleep, reset_counter};

    let iters = if cfg!(miri) { 1000 } else { 100000 };
    let expected = 4 * iters as u64;

    assert_eq!(race_increment(iters, 4), expected);
    assert_eq!(read_after_sleep(), expected);
    reset_counter();
    assert_eq!(read_after_sleep(), 0);
}

#[test]
fn normalize_removes_all_whitespace() {
    assert_eq!(normalize(" Hello\tWorld "), "helloworld");
    assert_eq!(normalize("A\n B\r\n\tC"), "abc")
}

#[test]
fn average_positive_ignores_non_positive() {
    assert_eq!(broken_app::average_positive(&[]), 0.0);
    assert_eq!(broken_app::average_positive(&[-1, -2, 0]), 0.0);
    assert_eq!(broken_app::average_positive(&[4]), 4.0);
}
