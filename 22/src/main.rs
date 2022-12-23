use std::collections::HashMap;

use num::complex::Complex;

const UP: Complex<i32> = Complex::new(-1, 0);
const DOWN: Complex<i32> = Complex::new(1, 0);
const LEFT: Complex<i32> = Complex::new(0, -1);
const RIGHT: Complex<i32> = Complex::new(0, 1);

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

impl Cube {
    fn get_new_face_coords(
        &self,
        direction: &Direction,
        coord: Complex<i32>,
    ) -> (Self, Direction, Complex<i32>) {
        use Cube::*;
        use Direction::*;

        let Complex { re: row, im: col } = coord;

        match (self, direction) {
            (One, Up) => (Six, Right, Complex::new(0, row)),
            (One, Down) => (Three, Down, Complex::new(row, 0)),
            (One, Left) => (Four, Right, Complex::new(0, 49 - col)),
            (One, Right) => (Two, Right, Complex::new(0, col)),
            (Two, Up) => (Six, Up, Complex::new(row, 49)),
            (Two, Down) => (Three, Left, Complex::new(49, row)),
            (Two, Left) => (One, Left, Complex::new(49, col)),
            (Two, Right) => (Five, Left, Complex::new(49, 49 - col)),
            (Three, Up) => (One, Up, Complex::new(row, 49)),
            (Three, Down) => (Five, Down, Complex::new(row, 0)),
            (Three, Left) => (Four, Down, Complex::new(col, 0)),
            (Three, Right) => (Two, Up, Complex::new(col, 49)),
            (Four, Up) => (Three, Right, Complex::new(0, row)),
            (Four, Down) => (Six, Down, Complex::new(row, 0)),
            (Four, Left) => (One, Right, Complex::new(0, 49 - col)),
            (Four, Right) => (Five, Right, Complex::new(0, col)),
            (Five, Right) => (Two, Left, Complex::new(49, 49 - col)),
            (Five, Up) => (Three, Up, Complex::new(row, 49)),
            (Five, Left) => (Four, Left, Complex::new(49, col)),
            (Five, Down) => (Six, Left, Complex::new(49, row)),
            (Six, Left) => (One, Down, Complex::new(col, 0)),
            (Six, Down) => (Two, Down, Complex::new(row, 0)),
            (Six, Up) => (Four, Up, Complex::new(row, 49)),
            (Six, Right) => (Five, Up, Complex::new(col, 49)),
        }
    }
}

// fn transition(
//     from: Cube,
//     orientation: Direction,
//     row: i32,
//     col: i32,
// ) -> (Cube, Direction, Complex<i32>) {
//     use Cube::*;
//     use Direction::*;

//    match (from, orientation) {
//         (One, Up) => (Six, Right, Complex::new(0, row)),
//         (One, Down) => (Three, Down, Complex::new(row, 0)),
//         (One, Left) => (Four, Right, Complex::new(0, 49 - col)),
//         (One, Right) => (Two, Right, Complex::new(0, col)),
//         (Two, Up) => (Six, Up, Complex::new(row, 49)),
//         (Two, Down) => (Three, Left, Complex::new(49, row)),
//         (Two, Left) => (One, Left, Complex::new(49, col)),
//         (Two, Right) => (Five, Left, Complex::new(49, 49 - col)),
//         (Three, Up) => (One, Up, Complex::new(row, 49)),
//         (Three, Down) => (Five, Down, Complex::new(row, 0)),
//         (Three, Left) => (Four, Down, Complex::new(col, 0)),
//         (Three, Right) => (Two, Up, Complex::new(col, 49)),
//         (Four, Up) => (Three, Right, Complex::new(0, row)),
//         (Four, Down) => (Six, Down, Complex::new(row, 0)),
//         (Four, Left) => (One, Right, Complex::new(0, 49 - col)),
//         (Four, Right) => (Five, Right, Complex::new(0, col)),
//         (Five, Right) => (Two, Left, Complex::new(49, 49 - col)),
//         (Five, Up) => (Three, Up, Complex::new(row, 49)),
//         (Five, Left) => (Four, Left, Complex::new(49, col)),
//         (Five, Down) => (Six, Left, Complex::new(49, row)),
//         (Six, Left) => (One, Down, Complex::new(col, 0)),
//         (Six, Down) => (Two, Down, Complex::new(row, 0)),
//         (Six, Up) => (Four, Up, Complex::new(row, 49)),
//         (Six, Right) => (Five, Up, Complex::new(col, 49)),
//         _ => unreachable!(),
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
    println!("Day 22: ");
    let input = include_str!("../inputs/input.txt");

    let mut set_cursor = true;
    let mut parse_password = false;

    let mut grid = HashMap::new();
    let mut cube = HashMap::new();
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

            let cube_coord = Complex::new(row as i32 % 50, col as i32 % 50);

            let cell_type = match cell {
                '.' => Cell::Path,
                '#' => Cell::Wall,
                _ => unreachable!("Unknown cell type: {cell}"),
            };

            max_row = max_row.max(row as i32);
            max_col = max_col.max(col as i32);
            min_row = min_row.min(row as i32);
            min_col = min_col.min(col as i32);

            cube.insert(
                (cube_face, cube_coord),
                Complex::new(row as i32, col as i32),
            );
            grid.insert(
                Complex::new(row as i32, col as i32),
                (cell_type, cube_face, cube_coord),
            );
        }
    }

    let bounds = (
        Complex::new(min_row, min_col),
        Complex::new(max_row, max_col),
    );

    let mut distance = "".to_string();

    for char in password.chars() {
        if char.is_ascii_digit() {
            distance.push_str(&format!("{char}"));
        } else {
            let turn = match char {
                'L' => Turn::Left,
                'R' => Turn::Right,
                _ => unreachable!("Unknown turn instruction: '{char}'"),
            };
            let dist: i32 = distance.parse().unwrap();

            cursor_1 = move_cursor(cursor_1, &direction_1, dist, &grid, &bounds);
            (cursor_2, direction_2) =
                move_cursor_on_cube(cursor_2, &direction_2, dist, &grid, &cube);

            direction_1 = direction_1.turn(turn);
            direction_2 = direction_2.turn(turn);

            distance = "".to_string();
        }
    }

    if !distance.is_empty() {
        let dist: i32 = distance.parse().unwrap();
        cursor_1 = move_cursor(cursor_1, &direction_1, dist, &grid, &bounds);
        (cursor_2, direction_2) = move_cursor_on_cube(cursor_2, &direction_2, dist, &grid, &cube);
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
    grid: &HashMap<Complex<i32>, (Cell, Cube, Complex<i32>)>,
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
        if let Some((cell, _, _)) = grid.get(&(new_cursor + vector)) {
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
    start: Complex<i32>,
    direction: &Direction,
    distance: i32,
    grid: &HashMap<Complex<i32>, (Cell, Cube, Complex<i32>)>,
    cube: &HashMap<(Cube, Complex<i32>), Complex<i32>>,
) -> (Complex<i32>, Direction) {
    let mut new_cursor = start;
    let mut direction = *direction;
    let vector = match direction {
        Direction::Up => UP,
        Direction::Down => DOWN,
        Direction::Left => LEFT,
        Direction::Right => RIGHT,
    };

    for _ in 0..distance {
        if let Some((cell, _, _)) = grid.get(&(new_cursor + vector)) {
            if let Cell::Wall = cell {
                break;
            } else {
                new_cursor += vector;
            }
        } else {
            // Wrap around cube
            if let Some((_, face, face_coords)) = grid.get(&new_cursor) {
                let (new_face, new_dir, new_face_coords) =
                    (*face).get_new_face_coords(&direction, *face_coords);

                println!("Going from ({face:?}, {direction:?}) to ({new_face:?}, {new_dir:?})");
                println!("\t{face_coords} -> {new_face_coords}");


                let map_coords = cube.get(&(new_face, new_face_coords)).unwrap();
                match grid.get(map_coords).unwrap() {
                    (Cell::Wall, _, _) => break,
                    (Cell::Path, _, _) => {
                        direction = new_dir;
                        new_cursor = *map_coords;
                    }
                }
            } else {
                panic!("Problem wrapping around cube")
            }
        }
    }

    (new_cursor, direction)
}
