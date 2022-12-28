use std::fmt::Display;

use hashbrown::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cell {
    Wall,
    Blizzard(Direction),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Wall => '#',
            Cell::Blizzard(Direction::Up) => '^',
            Cell::Blizzard(Direction::Down) => 'v',
            Cell::Blizzard(Direction::Left) => '<',
            Cell::Blizzard(Direction::Right) => '>',
        };

        write!(f, "{c}")
    }
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut grid: HashMap<_, Vec<_>> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let (mut max_row, mut max_col) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if let Some(cell) = match char {
                '#' => Some(Cell::Wall),
                '.' => {
                    if row == 0 {
                        start = (row, col)
                    }
                    end = (row, col);
                    None
                }
                '>' => Some(Cell::Blizzard(Direction::Right)),
                '<' => Some(Cell::Blizzard(Direction::Left)),
                '^' => Some(Cell::Blizzard(Direction::Up)),
                'v' => Some(Cell::Blizzard(Direction::Down)),
                _ => unreachable!("Unknown cell type: {char}"),
            } {
                grid.entry((row, col)).or_default().push(cell);
            }
            max_row = max_row.max(row);
            max_col = max_col.max(col);
        }
    }

    let bounds = (max_row, max_col);
    let cycle_length = lcm(bounds.0 - 1, bounds.1 - 1);

    let mut possible_states = Vec::with_capacity(cycle_length);
    possible_states.push(grid.clone());

    for _ in 1..cycle_length {
        grid = move_blizzard(&grid, bounds);
        possible_states.push(grid.clone());
    }

    let answer_1 = get_shortest_path_2(end, start, bounds, 0, cycle_length, &possible_states).unwrap();
    let answer_2_1 = get_shortest_path_2(start, end, bounds, answer_1, cycle_length, &possible_states).unwrap();
    let answer_2_2 = get_shortest_path_2(end, start, bounds, (answer_1 + answer_2_1) % cycle_length, cycle_length, &possible_states).unwrap();

    println!("\t1) {answer_1}");
    println!("\t1) {}", answer_1 + answer_2_1 + answer_2_2);

}

fn lcm(a: usize, b: usize) -> usize {
    let mut candidate = a.max(b);
    loop {
        if candidate % a == 0 && candidate % b == 0 {
            break;
        }
        candidate += 1;
    }
    candidate
}

fn get_shortest_path_2(
    goal: (usize, usize),
    start: (usize, usize),
    bounds: (usize, usize),
    initial_time: usize,
    cycle_length: usize,
    grids: &[HashMap<(usize, usize), Vec<Cell>>],
) -> Result<usize, &'static str> {
    let mut open_set = HashSet::new();
    let start_state = (start.0, start.1, initial_time);
    open_set.insert(start_state);

    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start_state, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start_state, manhattan(start, goal));

    while !open_set.is_empty() {
        let (_, current) = open_set
            .clone()
            .into_iter()
            .map(|state| {
                let dist = if let Some(d) = f_score.get(&state) {
                    *d
                } else {
                    usize::MAX
                };
                (dist, state)
            })
            .min_by_key(|k| k.0)
            .unwrap();

        if (current.0, current.1) == goal {
            let tentative_g_score = g_score.get(&current).unwrap();
            return Ok(*tentative_g_score);
        }

        open_set.remove(&current);

        for neighbour in get_valid_moves_2(
            grids,
            (current.0, current.1),
            bounds,
            cycle_length,
            current.2,
        ) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);

                f_score.insert(
                    neighbour,
                    tentative_g_score + manhattan((neighbour.0, neighbour.1), goal),
                );
                if !open_set.contains(&neighbour) {
                    open_set.insert(neighbour);
                }
            }
        }
    }

    Err("I did not manage to find a path")
}

fn manhattan(coord: (usize, usize), goal: (usize, usize)) -> usize {
    let x = (goal.0 as i64 - coord.0 as i64).abs();
    let y = (goal.1 as i64 - coord.1 as i64).abs();

    (x + y) as usize
}

fn get_valid_moves_2(
    grids: &[HashMap<(usize, usize), Vec<Cell>>],
    start: (usize, usize),
    bounds: (usize, usize),
    cycle_length: usize,
    timestep: usize,
) -> Vec<(usize, usize, usize)> {
    let next_timestep = (timestep + 1) % cycle_length;
    let mut to_explore = vec![start];

    if start.0 >= 1 {
        to_explore.push((start.0 - 1, start.1))
    }
    if start.1 >= 1 {
        to_explore.push((start.0, start.1 - 1))
    }
    if start.0 < bounds.0 {
        to_explore.push((start.0 + 1, start.1))
    }
    if start.1 < bounds.1 {
        to_explore.push((start.0, start.1 + 1))
    }

    to_explore
        .into_iter()
        .filter(|coord| grids[next_timestep].get(coord).is_none())
        .map(|coord| (coord.0, coord.1, next_timestep))
        .collect()
}

fn move_blizzard(
    grid: &HashMap<(usize, usize), Vec<Cell>>,
    bounds: (usize, usize),
) -> HashMap<(usize, usize), Vec<Cell>> {
    use Cell::*;
    use Direction::*;

    let mut new_grid: HashMap<(usize, usize), Vec<Cell>> = HashMap::new();

    for ((row, col), cells) in grid.iter() {
        for cell in cells {
            let new_coords = match cell {
                Wall => (*row, *col),
                Blizzard(Up) => {
                    let new_row = if *row == 1 { bounds.0 - 1 } else { row - 1 };
                    (new_row, *col)
                }
                Blizzard(Down) => {
                    let mut new_row = row + 1;
                    if new_row >= bounds.0 {
                        new_row = 1;
                    }
                    (new_row, *col)
                }
                Blizzard(Left) => {
                    let new_col = if *col == 1 { bounds.1 - 1 } else { col - 1 };
                    (*row, new_col)
                }
                Blizzard(Right) => {
                    let mut new_col = col + 1;
                    if new_col >= bounds.1 {
                        new_col = 1;
                    }
                    (*row, new_col)
                }
            };
            new_grid.entry(new_coords).or_default().push(*cell);
        }
    }

    new_grid
}

fn print_grid(grid: &HashMap<(usize, usize), Vec<Cell>>, bounds: (usize, usize)) {
    for row in 0..=bounds.0 {
        for col in 0..=bounds.1 {
            if let Some(cells) = grid.get(&(row, col)) {
                if cells.len() > 1 {
                    print!("{}", cells.len())
                } else if cells.is_empty() {
                    print!("X")
                } else {
                    print!("{}", cells[0])
                }
            } else {
                print!(".")
            }
        }
        println!();
    }
}
