use std::collections::HashSet;

use anyhow::{format_err, Result};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;
use nom::{multi, sequence};
use num::Complex;

const DOWN: Complex<i32> = Complex::new(0, 1);
const DOWN_RIGHT: Complex<i32> = Complex::new(1, 1);
const DOWN_LEFT: Complex<i32> = Complex::new(-1, 1);

fn main() -> Result<()> {
    println!("Day 14:");
    let input = include_str!("../inputs/input.txt");
    let mut grid: HashSet<Complex<i32>> = HashSet::new();

    let (_, ranges) = parse_input(input)?;
    for range in ranges.iter() {
        for (s, e) in range.iter().tuple_windows() {
            for c in get_range(*s, *e).unwrap() {
                grid.insert(c);
            }
        }
    }

    let bottom: i32 = grid.iter().map(|c| c.im).max().unwrap();

    let start = Complex::new(500, 0);

    let mut grains = 0;

    // Part 1
    'outer: loop {
        let mut coord = start;
        loop {
            if coord.im >= bottom {
                // fall out into void
                break 'outer;
            }

            if let Some(new_coord) = check_coordinate(coord, &grid) {
                coord = new_coord;
                continue;
            }

            grid.insert(coord);
            grains += 1;
            break;
        }
    }
    println!("\t1) {grains}");

    // Add floor
    for re in -1000..=1000 {
        grid.insert(Complex::new(re, bottom + 2));
    }

    // Part 2
    'outer: loop {
        let mut coord = start;
        loop {
            if grid.contains(&start) {
                break 'outer;
            }

            if let Some(new_coord) = check_coordinate(coord, &grid) {
                coord = new_coord;
                continue;
            }

            grid.insert(coord);
            grains += 1;
            break;
        }
    }
    println!("\t2) {grains}");

    Ok(())
}

fn check_coordinate(coord: Complex<i32>, grid: &HashSet<Complex<i32>>) -> Option<Complex<i32>> {
    if !grid.contains(&(coord + DOWN)) {
        Some(coord + DOWN)
    } else if !grid.contains(&(coord + DOWN_LEFT)) {
        Some(coord + DOWN_LEFT)
    } else if !grid.contains(&(coord + DOWN_RIGHT)) {
        Some(coord + DOWN_RIGHT)
    } else {
        None
    }
}

fn coord_parser(input: &str) -> IResult<&str, Complex<i32>> {
    let (input, (x, y)) = sequence::separated_pair(complete::i32, tag(","), complete::i32)(input)?;

    Ok((input, Complex::new(x, y)))
}

fn range_parser(input: &str) -> IResult<&str, Vec<Complex<i32>>> {
    let (input, coords) = multi::separated_list1(tag(" -> "), coord_parser)(input)?;

    Ok((input, coords))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Complex<i32>>>> {
    let (input, parsed) = multi::separated_list1(tag("\n"), range_parser)(input)?;

    Ok((input, parsed))
}

fn get_range(c1: Complex<i32>, c2: Complex<i32>) -> Result<Vec<Complex<i32>>> {
    if c1.re == c2.re {
        let mut v = vec![c1.im, c2.im];
        v.sort();
        return Ok((v[0]..=v[1]).map(|im| Complex::new(c1.re, im)).collect());
    } else if c1.im == c2.im {
        let mut v = vec![c1.re, c2.re];
        v.sort();
        return Ok((v[0]..=v[1]).map(|re| Complex::new(re, c1.im)).collect());
    };

    Err(format_err!("Diagonal range"))
}
