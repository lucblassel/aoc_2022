use std::collections::HashSet;

use num::complex::Complex;
use regex::Regex;

fn main() {
    println!("Day 9:");
    let input = include_str!("../inputs/input.txt");
    let mut tail_positions: HashSet<Complex<i32>> = HashSet::new();
    let mut long_tail_positions: HashSet<Complex<i32>> = HashSet::new();

    let regex = Regex::new(r"(U|D|L|R) (\d+)").unwrap();

    let mut head_coord = Complex::new(0, 0);
    let mut tail_coord = Complex::new(0, 0);

    let mut long_rope: [Complex<i32>; 10] = [Complex::new(0, 0); 10];

    tail_positions.insert(tail_coord);

    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let distance: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let step = match direction {
            "L" => Complex::new(-1, 0),
            "R" => Complex::new(1, 0),
            "D" => Complex::new(0, -1),
            "U" => Complex::new(0, 1),
            _ => unreachable!("Direction {direction} unknown..."),
        };

        for _ in 0..distance {
            // part 1
            head_coord += step;
            tail_coord = get_new_coord(&head_coord, &tail_coord);
            tail_positions.insert(tail_coord);

            // part 2
            long_rope[0] += step;
            for index in 1..long_rope.len() {
                long_rope[index] = get_new_coord(&long_rope[index-1], &long_rope[index]);
            }
            long_tail_positions.insert(*long_rope.last().unwrap());
        }
    }

    println!("\t1) {}", tail_positions.len());
    println!("\t2) {}", long_tail_positions.len());
}

fn get_new_coord(head: &Complex<i32>, tail: &Complex<i32>) -> Complex<i32> {
    if (head - tail).norm_sqr() <= 2 {
        return *tail;
    }

    let diff = unit(head - tail);
    tail + unit(diff)
}

fn unit(c: Complex<i32>) -> Complex<i32> {
    let re = if c.re != 0 { c.re / c.re.abs() } else { 0 };
    let im = if c.im != 0 { c.im / c.im.abs() } else { 0 };

    Complex::new(re, im)
}
