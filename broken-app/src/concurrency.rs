use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Небезопасный инкремент через несколько потоков.
/// Использует global static mut без синхронизации — data race.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    COUNTER.store(0, Ordering::Relaxed);
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..iterations {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().expect("working thread failed");
    }
    COUNTER.load(Ordering::Relaxed)
}

/// Плохая «синхронизация» — просто sleep, возвращает потенциально устаревшее значение.
pub fn read_after_sleep() -> u64 {
    COUNTER.load(Ordering::Relaxed)
}

/// Сброс счётчика (также небезопасен, без синхронизации).
pub fn reset_counter() {
    COUNTER.store(0, Ordering::Relaxed);
}
