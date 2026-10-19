#!/usr/bin/env bash

set -uo pipefail

OUT="artifacts/${1:?укажи папку для логов, например 03_after_fix}"
TARGET=aarch64-unknown-linux-gnu
VALGRIND="valgrind --leak-check=full --error-exitcode=1 --suppressions=scripts/rust-std.supp"
mkdir -p "$OUT"

run() {
    local name=$1; shift
    echo "=== $name ..."
    "$@" > "$OUT/$name.log" 2>&1
    echo "    код выхода: $?   (лог: $OUT/$name.log)"
}

run cargo_test cargo test
run miri       cargo +nightly miri test
run valgrind   env CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="$VALGRIND" cargo test --tests
run asan       env RUSTFLAGS="-Zsanitizer=address -Zub-checks=no" \
                   cargo +nightly test -Zbuild-std --target "$TARGET" --target-dir target/asan --tests
run tsan       env RUSTFLAGS="-Zsanitizer=thread" \
                   cargo +nightly test -Zbuild-std --target "$TARGET" --target-dir target/tsan --tests

echo
echo "=== Сводка"
grep -hE "^test result|ERROR SUMMARY|definitely lost|Undefined Behavior|memory leaked|ERROR: (Address|Leak)Sanitizer|WARNING: ThreadSanitizer" "$OUT"/*.log