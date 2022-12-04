use std::ops::RangeInclusive;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    println!("Day 4");
    let input = include_str!("../inputs/input.txt");
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$")?;

    let mut contained = 0;
    let mut ranges: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = vec![];

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let mut iter = caps.iter();
        iter.next();

        // Ugly as hell
        let s1: u32 = iter.next().unwrap().unwrap().as_str().parse()?;
        let e1: u32 = iter.next().unwrap().unwrap().as_str().parse()?;
        let s2: u32 = iter.next().unwrap().unwrap().as_str().parse()?;
        let e2: u32 = iter.next().unwrap().unwrap().as_str().parse()?;

        let range_1 = s1..=e1;
        let range_2 = s2..=e2;

        if (range_1.contains(&s2) && range_1.contains(&e2))
            || (range_2.contains(&s1) && range_2.contains(&e1))
        {
            contained += 1;
        }

        ranges.push((range_1, range_2));
    }

    println!("\t1) {contained}");

    let mut overlaps = 0;

    for (range_1, range_2) in ranges.iter() {
        if range_1.contains(range_2.start())
            || range_1.contains(range_2.end())
            || range_2.contains(range_1.start())
            || range_2.contains(range_1.end())
        {
            overlaps += 1;
        }
    }

    println!("\t2) {overlaps}");

    Ok(())
}
