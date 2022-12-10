use std::collections::HashSet;

fn main() {
    println!("Day 10:");
    let input = include_str!("../inputs/input.txt");
    let to_check: HashSet<i32> = HashSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
    let mut screen: [char; 240] = ['.'; 240];

    let mut signal_strength = 0;
    let mut x = 1;
    let mut cycle = 1;
    let mut pixel = 0;

    for line in input.lines() {
        let (num, cycles) = if line.len() > 4 {
            let (_, num_s) = line.split_at(5);
            let num: i32 = num_s.parse().unwrap();
            (num, 2)
        } else {
            (0, 1)
        };

        cycle += 1;
        if cycles == 2 {
            if check_visible(pixel, x) {
                screen[pixel as usize] = '#';
            }
            pixel += 1;
            if to_check.contains(&cycle) {
                signal_strength += cycle * x;
            }
            cycle += 1;
        }

        if check_visible(pixel, x) {
            screen[pixel as usize] = '#';
        }
        pixel += 1;
        x += num;

        if to_check.contains(&cycle) {
            signal_strength += cycle * x;
        }
    }

    println!("\t1) {signal_strength}");
    println!("\t2)");
    for line in screen.chunks(40) {
        println!("{}", line.iter().collect::<String>());
    }
}

fn check_visible(pixel: i32, x: i32) -> bool {
    let relative_pixel = pixel % 40;

    relative_pixel >= x - 1 && relative_pixel <= x + 1
}
