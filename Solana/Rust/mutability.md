By default all variables in rust are immutable because:

1. Immutable data is inherently thread-safe because if no thread can alter the data, then no syncronization is needed when data is accessed concurrently.

2. Knowing that certain data will not change allows the compiler to optimize code better.
