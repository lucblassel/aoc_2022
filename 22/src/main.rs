use std::collections::HashMap;

use num::complex::Complex;
use regex::Regex;

const UP: Complex<i32> = Complex::new(-1, 0);
const DOWN: Complex<i32> = Complex::new(1, 0);
const LEFT: Complex<i32> = Complex::new(0, -1);
const RIGHT: Complex<i32> = Complex::new(0, 1);

const WIDTH: i32 = 50;

#[derive(Debug)]
enum Cell {
    Wall,
    Path,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Cube {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

// impl Cube {
//     fn get_new_face_coords(
//         &self,
//         direction: &Direction,
//         coord: Complex<i32>,
//     ) -> (Self, Direction, Complex<i32>) {
//         use Cube::*;
//         use Direction::*;

//         let Complex { re: row, im: col } = coord;

//         match (self, direction) {
//             (One, Up) => (Six, Right, Complex::new(2 * WIDTH + col, 0)), //
//             (One, Down) => (Three, Down, Complex::new(row + 1, col)),
//             (One, Left) => (Four, Right, Complex::new(3 * WIDTH - row, 0)),
//             (One, Right) => (Two, Right, Complex::new(row, col + 1)),
//             (Two, Up) => (Six, Up, Complex::new(4 * WIDTH - 1, col - 2 * WIDTH)), //
//             (Two, Down) => (Three, Left, Complex::new(col - WIDTH, 2 * WIDTH - 1)),
//             (Two, Left) => (One, Left, Complex::new(row, col - 1)),
//             (Two, Right) => (Five, Left, Complex::new(col, 2 * WIDTH - 1)),
//             (Three, Up) => (One, Up, Complex::new(row - 1, col)),
//             (Three, Down) => (Five, Down, Complex::new(row + 1, col)),
//             (Three, Left) => (Four, Down, Complex::new(2 * WIDTH, row - WIDTH)),
//             (Three, Right) => (Two, Up, Complex::new(WIDTH - 1, row + WIDTH)),
//             (Four, Up) => (Three, Right, Complex::new(WIDTH + col, WIDTH)),
//             (Four, Down) => (Six, Down, Complex::new(row + 1, col)),
//             (Four, Left) => (One, Right, Complex::new(WIDTH * 3 - row, WIDTH)),
//             (Four, Right) => (Five, Right, Complex::new(row, col + 1)),
//             (Five, Right) => (Two, Left, Complex::new(WIDTH * 3 - row, 3 * WIDTH - 1)),
//             (Five, Up) => (Three, Up, Complex::new(row - 1, col)),
//             (Five, Left) => (Four, Left, Complex::new(row, col - 1)),
//             (Five, Down) => (Six, Left, Complex::new(2 * WIDTH + col, WIDTH - 1)),
//             (Six, Left) => (One, Down, Complex::new(0, row - 2 * WIDTH)), //
//             (Six, Down) => (Two, Down, Complex::new(0, col + 2 * WIDTH)), //
//             (Six, Up) => (Four, Up, Complex::new(row - 1, col)),
//             (Six, Right) => (Five, Up, Complex::new(3 * WIDTH - 1, row - 2 * WIDTH)),
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, dir: Turn) -> Self {
        match (self, dir) {
            (Self::Up, Turn::Left) => Self::Left,
            (Self::Up, Turn::Right) => Self::Right,
            (Self::Down, Turn::Left) => Self::Right,
            (Self::Down, Turn::Right) => Self::Left,
            (Self::Left, Turn::Left) => Self::Down,
            (Self::Left, Turn::Right) => Self::Up,
            (Self::Right, Turn::Left) => Self::Up,
            (Self::Right, Turn::Right) => Self::Down,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Self::Up => 3,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 0,
        }
    }
}

fn main() {
    println!("Day 22:");
    let input = include_str!("../inputs/input.txt");
    let mut set_cursor = true;
    let mut parse_password = false;

    let mut grid = HashMap::new();
    let mut password = "";

    let mut cursor_1 = Complex::new(0, 0);
    let mut cursor_2 = Complex::new(0, 0);
    let mut direction_1 = Direction::Right;
    let mut direction_2 = direction_1;

    let (mut max_row, mut max_col) = (i32::MIN, i32::MIN);
    let (mut min_row, mut min_col) = (i32::MAX, i32::MAX);

    for (row, line) in input.lines().enumerate() {
        if parse_password {
            password = line;
            continue;
        }

        if line.is_empty() {
            parse_password = true;
        }

        for (col, cell) in line.chars().enumerate() {
            if set_cursor && cell != ' ' {
                cursor_1 = Complex::new(row as i32, col as i32);
                cursor_2 = cursor_1;
                set_cursor = false;
            }

            let cube_face = match (row / 50, col / 50) {
                (0, 1) => Cube::One,
                (0, 2) => Cube::Two,
                (1, 1) => Cube::Three,
                (2, 0) => Cube::Four,
                (2, 1) => Cube::Five,
                (3, 0) => Cube::Six,
                _ => continue,
            };

            let cell_type = match cell {
                '.' => Cell::Path,
                '#' => Cell::Wall,
                _ => unreachable!("Unknown cell type: {cell}"),
            };

            max_row = max_row.max(row as i32);
            max_col = max_col.max(col as i32);
            min_row = min_row.min(row as i32);
            min_col = min_col.min(col as i32);

            grid.insert(Complex::new(row as i32, col as i32), (cell_type, cube_face));
        }
    }

    let bounds = (
        Complex::new(min_row, min_col),
        Complex::new(max_row, max_col),
    );

    let pwd_regex = Regex::new(r"(\d+)([RL]?)").unwrap();
    for cap in pwd_regex.captures_iter(password) {
        if let Some(dist) = cap.get(1) {
            let dist: i32 = dist.as_str().parse().unwrap();
            cursor_1 = move_cursor(cursor_1, &direction_1, dist, &grid, &bounds);
            move_cursor_on_cube(dist, &mut cursor_2, &mut direction_2, &grid);
        }
        if let Some(rot) = cap.get(2) {
            let turn = match rot.as_str() {
                "L" => Turn::Left,
                "R" => Turn::Right,
                "" => continue,
                _ => unreachable!("Unknown turn instruction: '{rot:?}'"),
            };

            direction_1 = direction_1.turn(turn);
            direction_2 = direction_2.turn(turn);
        }
    }

    println!(
        "\t1) {}",
        1000 * (cursor_1.re + 1) + 4 * (cursor_1.im + 1) + direction_1.value()
    );

    println!(
        "\t2) {}",
        1000 * (cursor_2.re + 1) + 4 * (cursor_2.im + 1) + direction_2.value()
    );
}

fn move_cursor(
    start: Complex<i32>,
    direction: &Direction,
    distance: i32,
    grid: &HashMap<Complex<i32>, (Cell, Cube)>,
    bounds: &(Complex<i32>, Complex<i32>),
) -> Complex<i32> {
    let mut new_cursor = start;
    let vector = match direction {
        Direction::Up => UP,
        Direction::Down => DOWN,
        Direction::Left => LEFT,
        Direction::Right => RIGHT,
    };

    for _ in 0..distance {
        if let Some((cell, _)) = grid.get(&(new_cursor + vector)) {
            if let Cell::Wall = cell {
                break;
            } else {
                new_cursor += vector;
            }
        } else {
            let mut seeker = match direction {
                Direction::Up => Complex::new(bounds.1.re, new_cursor.im),
                Direction::Down => Complex::new(bounds.0.re, new_cursor.im),
                Direction::Left => Complex::new(new_cursor.re, bounds.1.im),
                Direction::Right => Complex::new(new_cursor.re, bounds.0.im),
            };
            // Find non-empty cell
            while grid.get(&seeker).is_none() {
                seeker += vector;
            }
            if let Cell::Wall = grid.get(&seeker).unwrap().0 {
                break;
            } else {
                new_cursor = seeker;
            }
        }
    }

    new_cursor
}

fn move_cursor_on_cube(
    distance: i32,
    coord: &mut Complex<i32>,
    direction: &mut Direction,
    grid: &HashMap<Complex<i32>, (Cell, Cube)>,
) {
    for _ in 0..distance {
        let vector = match direction {
            Direction::Up => UP,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
            Direction::Right => RIGHT,
        };
        let cache_direction = *direction;
        let Complex {
            re: mut new_row,
            im: mut new_col,
        } = vector + *coord;

        match direction {
            Direction::Up => {
                if new_row < 0 && (WIDTH..WIDTH * 2).contains(&new_col) {
                    (new_row, new_col) = (new_col + WIDTH * 2, 0);
                    *direction = Direction::Right;
                } else if new_row < 0 && (WIDTH * 2..WIDTH * 3).contains(&new_col) {
                    (new_row, new_col) = (WIDTH * 4 - 1, new_col - WIDTH * 2);
                } else if new_row == WIDTH * 2 - 1 && (0..WIDTH).contains(&new_col) {
                    (new_row, new_col) = (new_col + WIDTH, WIDTH);
                    *direction = Direction::Right;
                }
            }
            Direction::Down => {
                if new_row >= WIDTH * 4 && (0..WIDTH).contains(&new_col) {
                    (new_row, new_col) = (0, new_col + WIDTH * 2);
                } else if new_row == WIDTH && (WIDTH * 2..WIDTH * 3).contains(&new_col) {
                    (new_row, new_col) = (new_col - WIDTH, WIDTH * 2 - 1);
                    *direction = Direction::Left;
                } else if new_row == WIDTH * 3 && (WIDTH..WIDTH * 2).contains(&new_col) {
                    (new_row, new_col) = (new_col + WIDTH * 2, WIDTH - 1);
                    *direction = Direction::Left;
                }
            }
            Direction::Right => {
                if new_col >= WIDTH * 3 && (0..WIDTH).contains(&new_row) {
                    (new_row, new_col) = (WIDTH * 3 - 1 - new_row, WIDTH * 2 - 1);
                    *direction = Direction::Left;
                } else if new_col == WIDTH * 2 && (WIDTH * 2..WIDTH * 3).contains(&new_row) {
                    (new_row, new_col) = (WIDTH * 3 - 1 - new_row, WIDTH * 3 - 1);
                    *direction = Direction::Left;
                } else if new_col == WIDTH * 2 && (WIDTH..WIDTH * 2).contains(&new_row) {
                    (new_row, new_col) = (WIDTH - 1, new_row + WIDTH);
                    *direction = Direction::Up;
                } else if new_col == WIDTH && (WIDTH * 3..WIDTH * 4).contains(&new_row) {
                    (new_row, new_col) = (WIDTH * 3 - 1, new_row - WIDTH * 2);
                    *direction = Direction::Up;
                }
            }
            Direction::Left => {
                if new_col < 0 && (WIDTH * 3..WIDTH * 4).contains(&new_row) {
                    (new_row, new_col) = (0, new_row - WIDTH * 2);
                    *direction = Direction::Down;
                } else if new_col == WIDTH - 1 && (WIDTH..WIDTH * 2).contains(&new_row) {
                    (new_row, new_col) = (WIDTH * 2, new_row - WIDTH);
                    *direction = Direction::Down;
                } else if new_col == WIDTH - 1 && (0..WIDTH).contains(&new_row) {
                    (new_row, new_col) = (WIDTH * 3 - 1 - new_row, 0);
                    *direction = Direction::Right;
                } else if new_col < 0 && (WIDTH * 2..WIDTH * 3).contains(&new_row) {
                    (new_row, new_col) = (WIDTH * 3 - 1 - new_row, WIDTH);
                    *direction = Direction::Right;
                }
            }
        }

        if let (Cell::Wall, _) = grid.get(&Complex::new(new_row, new_col)).unwrap() {
            *direction = cache_direction;
            break;
        }

        coord.re = new_row;
        coord.im = new_col;
    }
}
