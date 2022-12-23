use std::collections::VecDeque;
use hashbrown::{HashMap, HashSet};

use num::complex::Complex;

const N: Complex<i32> = Complex::new(-1, 0);
const S: Complex<i32> = Complex::new(1, 0);
const E: Complex<i32> = Complex::new(0, 1);
const W: Complex<i32> = Complex::new(0, -1);
const NE: Complex<i32> = Complex::new(-1, 1);
const NW: Complex<i32> = Complex::new(-1, -1);
const SE: Complex<i32> = Complex::new(1, 1);
const SW: Complex<i32> = Complex::new(1, -1);

#[derive(Debug)]
enum Move {
    North,
    South,
    West,
    East,
}

fn get_new_coords(
    elf: &Complex<i32>,
    elves: &HashSet<Complex<i32>>,
    moves: &VecDeque<Move>,
) -> Complex<i32> {
    let free_n = !elves.contains(&(elf + N));
    let free_s = !elves.contains(&(elf + S));
    let free_e = !elves.contains(&(elf + E));
    let free_w = !elves.contains(&(elf + W));
    let free_ne = !elves.contains(&(elf + NE));
    let free_nw = !elves.contains(&(elf + NW));
    let free_se = !elves.contains(&(elf + SE));
    let free_sw = !elves.contains(&(elf + SW));

    if free_n && free_s && free_e && free_w && free_ne && free_nw && free_se && free_sw {
        *elf
    } else {
        for considered_move in moves {
            match considered_move {
                Move::North => {
                    if free_n && free_ne && free_nw {
                        return elf + N;
                    }
                }
                Move::South => {
                    if free_s && free_se && free_sw {
                        return elf + S;
                    }
                }
                Move::West => {
                    if free_w && free_nw && free_sw {
                        return elf + W;
                    }
                }
                Move::East => {
                    if free_e && free_ne && free_se {
                        return elf + E;
                    }
                }
            }
        }
        *elf
    }
}

fn main() {
    println!("Day 23:");
    let input = include_str!("../inputs/input.txt");

    let mut elves = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    elves.insert(Complex::new(row as i32, col as i32));
                }
                '.' => continue,
                _ => unreachable!("Unknown tile type: {char}"),
            }
        }
    }

    let mut moves = VecDeque::from_iter(vec![Move::North, Move::South, Move::West, Move::East]);

    let mut turn_counter = 0;
    loop {
        turn_counter += 1;

        let mut proposed: HashMap<Complex<i32>, Vec<Complex<i32>>> =
            HashMap::with_capacity(elves.len());

        // Propose moves
        for elf in elves.iter() {
            proposed
                .entry(get_new_coords(elf, &elves, &moves))
                .or_default()
                .push(*elf);
        }

        // Move elves
        let mut new_elves = HashSet::with_capacity(elves.len());
        for (considered_move, elves) in proposed.drain() {
            if elves.len() == 1 {
                new_elves.insert(considered_move);
            } else {
                for elf in elves {
                    new_elves.insert(elf);
                }
            }
        }

        if elves == new_elves {
            println!("\t2) {turn_counter}");
            break;
        }

        elves = new_elves;

        moves.rotate_left(1);

        // let front_move = moves.pop_front().unwrap();
        // moves.push_back(front_move);

        if turn_counter == 10 {
            println!("\t1) {}", count_ground(&elves));
        }
    }
}

fn count_ground(elves: &HashSet<Complex<i32>>) -> i32 {
    let mut count = 0;
    let bounds = elves.iter().fold(
        (
            Complex::new(i32::MAX, i32::MAX),
            Complex::new(i32::MIN, i32::MIN),
        ),
        |bounds: (Complex<i32>, Complex<i32>), cell| {
            let min_row = bounds.0.re.min(cell.re);
            let min_col = bounds.0.im.min(cell.im);
            let max_row = bounds.1.re.max(cell.re);
            let max_col = bounds.1.im.max(cell.im);

            (
                Complex::new(min_row, min_col),
                Complex::new(max_row, max_col),
            )
        },
    );

    for row in bounds.0.re..=bounds.1.re {
        for col in bounds.0.im..=bounds.1.im {
            if !elves.contains(&Complex::new(row, col)) {
                count += 1;
            }
        }
    }

    count
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<Complex<i32>>) {
    let bounds = elves.iter().fold(
        (
            Complex::new(i32::MAX, i32::MAX),
            Complex::new(i32::MIN, i32::MIN),
        ),
        |bounds: (Complex<i32>, Complex<i32>), cell| {
            let min_row = bounds.0.re.min(cell.re);
            let min_col = bounds.0.im.min(cell.im);
            let max_row = bounds.1.re.max(cell.re);
            let max_col = bounds.1.im.max(cell.im);

            (
                Complex::new(min_row, min_col),
                Complex::new(max_row, max_col),
            )
        },
    );

    for row in bounds.0.re - 2..=bounds.1.re + 2 {
        for col in bounds.0.im - 2..=bounds.1.im + 2 {
            if elves.contains(&Complex::new(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
