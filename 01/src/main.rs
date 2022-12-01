fn main() {
    println!("Day 1:");
    let input = include_str!("../inputs/input.txt");
    let mut calories: Vec<u64> = vec![0];

    for line in input.lines() {
        if line.is_empty() {
            calories.push(0);
            continue;
        }
        let num: u64 = line.parse().unwrap();
        let last = calories.last_mut().unwrap();
        *last += num;
    }

    calories.sort();

    println!("\t1) {}", calories.last().unwrap());
    println!(
        "\t2) {}",
        calories[calories.len() - 3..calories.len()]
            .iter()
            .sum::<u64>()
    );
}
