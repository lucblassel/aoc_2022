use std::collections::{HashMap, HashSet, VecDeque};

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

impl Move {
    fn check_move(&self, position: &Complex<i32>, elves: &HashSet<Complex<i32>>) -> bool {
        use Move::*;
        match self {
            North => {
                !elves.contains(&(N + position))
                    && !elves.contains(&(NE + position))
                    && !elves.contains(&(NW + position))
            }
            South => {
                !elves.contains(&(S + position))
                    && !elves.contains(&(SE + position))
                    && !elves.contains(&(SW + position))
            }
            West => {
                !elves.contains(&(W + position))
                    && !elves.contains(&(NW + position))
                    && !elves.contains(&(SW + position))
            }
            East => {
                !elves.contains(&(E + position))
                    && !elves.contains(&(NE + position))
                    && !elves.contains(&(SE + position))
            }
        }
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
                _ => unreachable!("Unknwon tile type: {char}"),
            }
        }
    }

    let mut move_order =
        VecDeque::from_iter(vec![Move::North, Move::South, Move::West, Move::East]);

    let mut turn_counter = 0;
    loop {
        turn_counter += 1;

        let mut proposed: HashMap<Complex<i32>, Vec<_>> = HashMap::new();

        // Propose moves
        for elf in elves.iter() {
            let valid_moves: Vec<_> = move_order
                .iter()
                .filter(|selected_move| selected_move.check_move(elf, &elves))
                .collect();

            let new_coords = if valid_moves.len() == 4 || valid_moves.is_empty() {
                *elf
            } else {
                match valid_moves[0] {
                    Move::North => elf + N,
                    Move::South => elf + S,
                    Move::East => elf + E,
                    Move::West => elf + W,
                }
            };

            proposed.entry(new_coords).or_default().push(*elf);
        }

        // Move elves
        let mut new_elves = HashSet::new();
        for (considered_move, elves) in proposed.into_iter() {
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

        let front_move = move_order.pop_front().unwrap();
        move_order.push_back(front_move);

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
