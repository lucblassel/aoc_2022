# Advent of Code 2022

This repo stores the code that I wrote during [advent of code 2022](https://adventofcode.com/2022).  
I am using AoC this year as a way to learn Rust, which I have been toying with the last couple weeks and it has been enjoyable so far!

## Timing
I used [hyperfine](https://github.com/sharkdp/hyperfine) with warmup to time my solutions for different days, compiled in `release` mode:  
`hyperfine -N --warmup 500 <executable>` 

**Results**: 
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
  Time (mean ± σ):      5.829 s ±  0.143 s    [User: 5.742 s, System: 0.048 s]
  Range (min … max):    5.674 s …  6.153 s    10 runs
```