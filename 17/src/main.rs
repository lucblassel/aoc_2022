use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Shape {
    Horizontal,
    Cross,
    LShape,
    Vertical,
    Square,
}

const SHAPE_ITEMS: &[Shape] = &[
    Shape::Horizontal,
    Shape::Cross,
    Shape::LShape,
    Shape::Vertical,
    Shape::Square,
];

impl Shape {
    fn is_colliding(&self, bottom: i64, left: i64, grid: &HashSet<(i64, i64)>) -> bool {
        for (y, x) in self.get_coords(bottom, left) {
            // Check collision with walls
            if y <= 0 || x <= 0 || x > 7 {
                return true;
            }
            if grid.contains(&(y, x)) {
                return true;
            }
        }
        false
    }

    fn get_coords(&self, bottom: i64, left: i64) -> Vec<(i64, i64)> {
        match self {
            Shape::Horizontal => (0..4).map(|v| (bottom, left + v)).collect(),
            Shape::Vertical => (0..4).map(|v| (bottom + v, left)).collect(),
            Shape::Cross => {
                vec![
                    (bottom + 2, left + 1),
                    (bottom + 1, left),
                    (bottom + 1, left + 1),
                    (bottom + 1, left + 2),
                    (bottom, left + 1),
                ]
            }
            Shape::Square => {
                vec![
                    (bottom, left),
                    (bottom, left + 1),
                    (bottom + 1, left),
                    (bottom + 1, left + 1),
                ]
            }
            Shape::LShape => {
                vec![
                    (bottom, left),
                    (bottom, left + 1),
                    (bottom, left + 2),
                    (bottom + 1, left + 2),
                    (bottom + 2, left + 2),
                ]
            }
        }
    }
}

fn main() {
    println!("Day 17:");
    let input = include_str!("../inputs/input.txt");
    let mut grid = HashSet::new();

    for col in 0..=7 {
        grid.insert((0, col));
    }

    let mut winds = input.chars().enumerate().cycle();

    let mut top = 0;
    let mut current_bottom = top + 4;
    let mut current_left = 3;

    let mut stopped_rocks: i64 = 0;
    let mut longest_fall = 0;

    let mut memory = HashMap::new();

    let mut cycle_stopped = 1000000000001;
    let mut cycle_height = 1000000000001;

    let mut found = false;

    let mut answer_1 = 0;
    let mut answer_2 = 0;

    let mut wind_index = 0;

    for (shape_index, shape) in SHAPE_ITEMS.iter().enumerate().cycle().take(8000) {
        let mut fall = 0;
        if !found {
            if let Some((stopped, height)) = memory.get(&(shape_index, wind_index)) {
                cycle_stopped = stopped_rocks - stopped;
                cycle_height = top - height;
            }
            memory.insert((shape_index, wind_index), (stopped_rocks, top));
            if top - 1 - longest_fall - 3 * cycle_height > 0
                && (1000000000000 - stopped_rocks) % cycle_stopped == 0
            {
                found = true;
                let remaining_cycles = (1000000000000 - stopped_rocks) / cycle_stopped;
                let extra_height = remaining_cycles * cycle_height;
                answer_2 = top + extra_height;
            }
        }

        if stopped_rocks == 2022 {
            answer_1 = top;
        }

        if answer_1 != 0 && answer_2 != 0 {
            break;
        }

        loop {
            let (wi, wind) = winds.next().unwrap();
            wind_index = wi + 1;

            let new_left = match wind {
                '>' => current_left + 1,
                '<' => current_left - 1,
                _ => unreachable!("Unknown wind: {wind}"),
            };
            // Apply wind
            if !shape.is_colliding(current_bottom, new_left, &grid) {
                current_left = new_left;
            }

            // Fall down
            if shape.is_colliding(current_bottom - 1, current_left, &grid) {
                let highest = shape
                    .get_coords(current_bottom, current_left)
                    .into_iter()
                    .map(|(y, x)| {
                        grid.insert((y, x));
                        y
                    })
                    .max()
                    .unwrap();
                top = top.max(highest);

                stopped_rocks += 1;
                longest_fall = longest_fall.max(fall);

                // Setting starting positions of nect rock
                current_left = 3;
                current_bottom = top + 4;

                break;
            } else {
                fall += 1;
                current_bottom -= 1;
            }
        }
    }

    println!("\t1) {answer_1}");
    println!("\t2) {answer_2}");
}
