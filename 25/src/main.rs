fn main() {
    println!("Day 25:");
    let input = include_str!("../inputs/input.txt");

    let total_fuel: i64 = input.lines().map(to_decimal).sum();
    to_snafu(total_fuel);
    println!("\t1) {}", to_snafu(total_fuel));
}

fn to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(pow, c)| {
            let digit = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => unreachable!("Unknown SNAFU digit {c}"),
            };
            digit * 5i64.pow(pow as u32)
        })
        .sum()
}

fn to_snafu(num: i64) -> String {
    let mut digits = vec![];
    let mut current = num;

    while current > 0 {
        let n = (current + 2) / 5;
        digits.push(current - n * 5);
        current = n;
    }

    digits
        .iter()
        .rev()
        .map(|digit| match digit {
            -2 => "=".to_owned(),
            -1 => "-".to_owned(),
            0 | 1 | 2 => format!("{digit}"),
            _ => unreachable!("Invalid snafu digit {digit}"),
        })
        .collect()
}
