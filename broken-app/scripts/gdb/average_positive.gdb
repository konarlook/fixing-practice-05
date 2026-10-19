set pagination off
set confirm off
break broken_app::average_positive
run --exact averages_only_positive --test-threads=1
info args
next
print sum
next
finish