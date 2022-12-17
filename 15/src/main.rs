use core::ops::RangeInclusive;
use num::complex::Complex;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Square {
    a: Complex<i64>,
    b: Complex<i64>,
    c: Complex<i64>,
    d: Complex<i64>,
}

impl Square {
    fn contains_point(&self, point: &Complex<i64>) -> bool {
        let am = point - self.a;
        let ab = self.b - self.a;
        let ad = self.d - self.a;

        0 <= dot(am, ab)
            && dot(am, ab) <= dot(ab, ab)
            && 0 <= dot(am, ad)
            && dot(am, ad) <= dot(ad, ad)
    }

    fn from_point(center: Complex<i64>, radius: i64) -> Self {
        let x_offset = Complex::new(radius, 0);
        let y_offset = Complex::new(0, radius);
        Square {
            a: center + y_offset,
            b: center + x_offset,
            c: center - y_offset,
            d: center - x_offset,
        }
    }

    fn offset_edges(&self) -> Vec<Line> {
        let mut lines = vec![];
        let x_off = Complex::new(1, 0);

        lines.push(Line::from_point(-1, self.a + x_off));
        lines.push(Line::from_point(1, self.b + x_off));
        lines.push(Line::from_point(-1, self.c - x_off));
        lines.push(Line::from_point(1, self.d + x_off));

        lines
    }
}

#[derive(Debug)]
struct Line {
    slope: i64,
    intercept: i64,
}

impl Line {
    fn from_point(slope: i64, point: Complex<i64>) -> Self {
        let intercept = point.im - slope * point.re;
        Line { slope, intercept }
    }

    fn intersection(&self, other: &Line) -> Option<Complex<i64>> {
        if self.slope == other.slope {
            None
        } else {
            let x = (self.intercept - other.intercept) / (other.slope - self.slope);
            let y = other.intercept + other.slope * x;
            Some(Complex::new(x, y))
        }
    }
}

fn dot(c1: Complex<i64>, c2: Complex<i64>) -> i64 {
    c1.re * c2.re + c1.im * c2.im
}

fn merge(range1: RangeInclusive<i64>, range2: RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    if range2.contains(range1.start()) || range1.contains(range1.start()) {
        let start = range1.start().min(range2.start());
        let end = range1.end().max(range2.end());
        Some(*start..=*end)
    } else {
        None
    }
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut sensors: Vec<(Complex<i64>, Complex<i64>)> = vec![];
    let mut beacons = HashSet::new();
    let mut squares: Vec<Square> = vec![];
    let mut grid: Vec<Line> = vec![];
    let y: i64 = 2_000_000;
    let y_line = Line::from_point(0, Complex::new(0, y));

    let mut ranges = vec![];

    for line in input.lines() {
        let cap = regex.captures(line).unwrap();

        let sensor_x: i64 = cap.get(1).unwrap().as_str().parse().unwrap();
        let sensor_y: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
        let beacon_x: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
        let beacon_y: i64 = cap.get(4).unwrap().as_str().parse().unwrap();

        let sensor = Complex::new(sensor_x, sensor_y);
        let beacon = Complex::new(beacon_x, beacon_y);

        sensors.push((sensor, beacon));

        let dist = (sensor - beacon).l1_norm();

        beacons.insert(beacon);

        let rect = Square::from_point(sensor, dist);
        grid.extend(rect.offset_edges().into_iter());

        let mut xs: Vec<_> = rect
            .offset_edges()
            .iter()
            .flat_map(|line| line.intersection(&y_line))
            .map(|c| c.re)
            .collect();
        xs.sort();

        squares.push(rect);
        if sensor.im + dist < y || sensor.im - dist > y {
            continue;
        }

        // Get correct boundary points
        let mut left = Complex::new(xs[1] - 1, y);
        let mut right = Complex::new(xs[2] + 1, y);
        while (left - sensor).l1_norm() > dist {
            left = Complex::new(left.re + 1, y);
        }
        while (right - sensor).l1_norm() > dist {
            right = Complex::new(right.re - 1, y);
        }

        ranges.push((left.re, right.re));
    }

    ranges.sort();
    let ranges: Vec<_> = ranges.iter().map(|(start, end)| *start..=*end).collect();
    let mut merged_ranges = vec![ranges[0].clone()];
    for range in ranges.iter().skip(1) {
        let last = merged_ranges.pop().unwrap();
        if let Some(merged) = merge(last.clone(), range.clone()) {
            merged_ranges.push(merged);
        } else {
            merged_ranges.push(last);
        }
    }

    let merged = merged_ranges[0].clone();
    let num_beacons = beacons.iter().filter(|c| c.im == y).count();
    let answer_1 = merged.end() - merged.start() - num_beacons as i64 + 1;
    println!("\t1) {answer_1}");

    const MAX: i64 = 4_000_000;

    let iter = grid.iter();
    let point = grid
        .iter()
        .flat_map(|l1| iter.clone().map(move |l2| l1.intersection(l2)))
        .flatten()
        .filter(|point| point.re >= 0 && point.re <= MAX && point.im >= 0 && point.im <= MAX) // check bounds
        .find(|point| {
            squares
                .iter()
                .filter(|square| square.contains_point(point))
                .count()
                == 0
        })
        .unwrap();

    println!("\t2) {}", point.re * 4000000 + point.im);
}
