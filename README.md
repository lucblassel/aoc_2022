# Advent of Code 2022

This repo stores the code that I wrote during [advent of code 2022](https://adventofcode.com/2022).  
I am using AoC this year as a way to learn Rust, which I have been toying with the last couple weeks and it has been enjoyable so far!

## Timing

I used [hyperfine](https://github.com/sharkdp/hyperfine) with warmup to time my solutions for different days, compiled in `release` mode:  
`hyperfine -N --warmup 100 <executable>`

**Individual Results**:

```
Benchmark 1: ./aoc-01
  Time (mean ± σ):       3.3 ms ±   0.2 ms    [User: 0.9 ms, System: 1.0 ms]
  Range (min … max):     2.9 ms …   4.8 ms    988 runs

Benchmark 1: ./aoc-02
  Time (mean ± σ):       6.0 ms ±   8.9 ms    [User: 1.3 ms, System: 1.5 ms]
  Range (min … max):     3.6 ms … 148.8 ms    628 runs

Benchmark 1: ./aoc-03
  Time (mean ± σ):       6.7 ms ±   7.1 ms    [User: 2.1 ms, System: 1.5 ms]
  Range (min … max):     4.3 ms … 102.5 ms    283 runs

Benchmark 1: ./aoc-04
  Time (mean ± σ):       5.9 ms ±   5.3 ms    [User: 1.9 ms, System: 1.5 ms]
  Range (min … max):     4.4 ms … 110.0 ms    453 runs

Benchmark 1: ./aoc-05
  Time (mean ± σ):       5.3 ms ±   0.5 ms    [User: 2.2 ms, System: 1.3 ms]
  Range (min … max):     4.4 ms …   9.6 ms    552 runs

Benchmark 1: ./aoc-06
  Time (mean ± σ):       5.0 ms ±   0.4 ms    [User: 2.4 ms, System: 1.0 ms]
  Range (min … max):     4.4 ms …   7.4 ms    638 runs

Benchmark 1: ./aoc-07
  Time (mean ± σ):       5.0 ms ±   0.4 ms    [User: 2.0 ms, System: 1.3 ms]
  Range (min … max):     4.3 ms …   7.6 ms    652 runs

Benchmark 1: ./aoc-08
  Time (mean ± σ):      11.1 ms ±   0.5 ms    [User: 8.3 ms, System: 1.1 ms]
  Range (min … max):    10.5 ms …  15.5 ms    238 runs

Benchmark 1: ./aoc-09
  Time (mean ± σ):       6.4 ms ±   0.3 ms    [User: 3.6 ms, System: 1.2 ms]
  Range (min … max):     6.0 ms …   8.0 ms    443 runs

Benchmark 1: ./aoc-10
  Time (mean ± σ):       3.4 ms ±   0.4 ms    [User: 0.9 ms, System: 1.0 ms]
  Range (min … max):     3.0 ms …  10.6 ms    951 runs

Benchmark 1: ./aoc-11
  Time (mean ± σ):      13.8 ms ±   0.5 ms    [User: 10.7 ms, System: 1.2 ms]
  Range (min … max):    13.1 ms …  17.1 ms    208 runs

Benchmark 1: ./aoc-12
  Time (mean ± σ):      77.2 ms ±   3.0 ms    [User: 72.3 ms, System: 2.4 ms]
  Range (min … max):    74.6 ms …  86.9 ms    34 runs

Benchmark 1: ./aoc-13
  Time (mean ± σ):       7.1 ms ±   3.4 ms    [User: 2.8 ms, System: 1.6 ms]
  Range (min … max):     5.4 ms …  49.6 ms    513 runs

Benchmark 1: ./aoc-14
  Time (mean ± σ):     203.4 ms ±   4.1 ms    [User: 198.3 ms, System: 2.2 ms]
  Range (min … max):   197.7 ms … 212.5 ms    14 runs

Benchmark 1: ./aoc-15
  Time (mean ± σ):       4.1 ms ±   0.3 ms    [User: 1.5 ms, System: 1.2 ms]
  Range (min … max):     3.7 ms …   6.1 ms    712 runs

Benchmark 1: ./aoc-16
  Time (mean ± σ):     322.2 ms ±  11.7 ms    [User: 300.4 ms, System: 14.5 ms]
  Range (min … max):   309.4 ms … 343.3 ms    10 runs

Benchmark 1: ./aoc-17
  Time (mean ± σ):      16.7 ms ±   0.9 ms    [User: 13.1 ms, System: 1.5 ms]
  Range (min … max):    15.2 ms …  20.7 ms    153 runs

Benchmark 1: ./aoc-18
  Time (mean ± σ):      11.8 ms ±   1.0 ms    [User: 7.5 ms, System: 2.2 ms]
  Range (min … max):    10.6 ms …  17.6 ms    273 runs

Benchmark 1: ./aoc-20
  Time (mean ± σ):     123.1 ms ±   4.2 ms    [User: 118.3 ms, System: 1.9 ms]
  Range (min … max):   118.5 ms … 136.5 ms    24 runs

Benchmark 1: ./aoc-21
  Time (mean ± σ):       6.1 ms ±   8.5 ms    [User: 1.8 ms, System: 1.4 ms]
  Range (min … max):     3.9 ms … 186.3 ms    735 runs

Benchmark 1: ./aoc-23
  Time (mean ± σ):     497.9 ms ±  76.3 ms    [User: 455.4 ms, System: 18.4 ms]
  Range (min … max):   439.7 ms … 644.6 ms    10 runs

```

**Total results:**
```
$ cargo build --release
$ hyperfine -N --warmup 30 './benchmark.sh'

Benchmark 1: ./benchmark.sh
  Time (mean ± σ):      1.277 s ±  0.011 s    [User: 1.155 s, System: 0.073 s]
  Range (min … max):    1.255 s …  1.291 s    10 runs
```