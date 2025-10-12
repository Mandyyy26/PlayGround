some variables are fixed at compile time and some variables may change at run time.

Stack: Fast allocation and deallocation. Rust uses the stack for most primitive data types and for data where the size is known at compile time.

Heap: Used for data that can grow at runtime, such as vectors or strings.

Whats stored on the stack?
Numbers, booleans, fixed sized arrays.

Whats stored on heap?
Strings, Vectors
