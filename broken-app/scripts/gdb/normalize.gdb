set pagination off
set confirm off
break broken_app::normalize
run --exact normalize_removes_all_whitespace --test-threads=1
info args
finish