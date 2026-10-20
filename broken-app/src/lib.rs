pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|&&v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .filter(|&&v| v > 0)
        .fold((0_i64, 0_usize), |(sum, count), &v| (sum + v, count + 1));
    if count == 0 {
        return 0.0;
    }
    sum as f64 / count as f64
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    val + *b
}
