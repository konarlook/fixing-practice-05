set pagination off
set confirm off
break broken_app::normalize
run --exact normalize_removes_all_whitespace --test-threads=1
info args
set language c
x/13cb input.data_ptr
set language auto