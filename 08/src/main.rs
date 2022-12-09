use ndarray::prelude::*;
use ndarray::Array;

fn main() {
    println!("Day 8:");
    let input = include_str!("../inputs/input.txt");
    let numbers: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let nrows = numbers.len();
    let ncols = numbers[0].len();
    let grid =
        Array::from_shape_vec((nrows, ncols), numbers.into_iter().flatten().collect()).unwrap();
    let shape = grid.shape();

    let mut visible = 0;

    let mut scores: Vec<i32> = vec![];

    for x in 0..shape[0] {
        for y in 0..shape[1] {
            let tree = grid[[x, y]];
            let slicers = vec![s![0..=x, y], s![x.., y], s![x, 0..=y], s![x, y..]];

            let mut is_visible = false;
            let mut scenic_scores: Vec<i32> = vec![];

            for (i, slicer) in slicers.into_iter().enumerate() {
                let slice = grid.slice(slicer);

                // Part 1
                let m = slice.iter().max().unwrap();
                let l = slice.iter().filter(|v| **v == tree).count();
                if *m == tree && l == 1 {
                    is_visible = true;
                }

                // Part 2
                let mut visible = 0;

                let iter: Box<dyn Iterator<Item = _>> = if i % 2 == 0 {
                    Box::new(slice.iter().rev())
                } else {
                    Box::new(slice.iter())
                };

                for other in iter.skip(1) {
                    visible += 1;
                    if *other >= tree {
                        break;
                    }
                }

                scenic_scores.push(visible);
            }

            if is_visible {
                visible += 1
            };
            scores.push(scenic_scores.iter().product());
        }
    }

    println!("\t1) {visible}");
    println!("\t2) {}", scores.iter().max().unwrap());
}
