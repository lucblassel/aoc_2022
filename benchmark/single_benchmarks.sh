#!/usr/bin/env bash

# SHORT
short=("01" "02" "03" "04" "05" "06" "07" "09" "10" "13" "08" "11" "19" "15" "17" "18" "21" "22" "25")
command="hyperfine -N --warmup 1000 --export-json short-bench.json"
for name in "${short[@]}"; do
command="$command '../target/release/aoc-$name'"
done
eval "$command"

# MEDIUM
medium=("12" "14" "16" "20" "23")
command="hyperfine -N --warmup 300 --export-json medium-bench.json"
for name in "${medium[@]}"; do
command="$command '../target/release/aoc-$name'"
done
eval "$command"

# LONG
hyperfine -N --warmup 30 --export-json long-bench.json '../target/release/aoc-24'