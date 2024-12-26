# Default `max-threads`

The default `max-threads` uses a simple algorithm to get how many threads.

```rs
if total_threads >= 32 {
    return total_threads - 4;
} else if total_threads >= 12 {
    return total_threads - 2;
} else if total_threads >= 3 {
    return total_threads - 1;
} else {
    return total_threads;
}
```

i.e. with `total_threads` as the number of threads the CPU(s) has:

- If the CPU has 32 or more threads, it will use all but 4 threads
- If the CPU has 12 or more threads, it will use all but 2 threads
- If the CPU has 3 or more threads, it will use all but 1 thread
- Otherwise, i.e. if the CPU has 1 to 2 threads, it will use all threads

---

Alternative algorithms I tried:

```rs
if total_threads >= 32 {
    return total_threads - total_threads.div_ceil(10);
} else if total_threads >= 12 {
    return total_threads - 2;
} else if total_threads >= 3 {
    return total_threads - 1;
} else {
    return total_threads;
}
```
