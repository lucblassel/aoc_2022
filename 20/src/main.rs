const DECRYPTION_KEY: i64 = 811589153;

fn main() {
    println!("Day 20:");
    let input = include_str!("../inputs/input.txt");
    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .collect();
    let mut state = numbers.clone();

    mix(&numbers, &mut state);

    let zero_index = state.iter().position(|v| v.1 == 0).unwrap();
    let answer_1: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|offset| state[(offset + zero_index) % state.len()].1)
        .sum();
    println!("\t1) {answer_1}");

    let numbers_2: Vec<_> = numbers
        .iter()
        .map(|(index, value)| (*index, *value * DECRYPTION_KEY))
        .collect();
    let mut state = numbers_2.clone();

    for _ in 0..10 {
        mix(&numbers_2, &mut state);
    }

    let zero_index = state.iter().position(|v| v.1 == 0).unwrap();
    let answer_2: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|offset| state[(offset + zero_index) % state.len()].1)
        .sum();
    println!("\t2) {answer_2}");
}

fn mix(numbers: &[(usize, i64)], state: &mut Vec<(usize, i64)>) {
    for (index, value) in numbers.iter() {
        if *value == 0 {
            continue;
        }

        let Some(state_index) = state.iter().position(|v| v.0 == *index) else {
            panic!("Could not find element with index {index}");
        };

        let elem = state.remove(state_index);
        let mut new_index = (state_index as i64 + value).rem_euclid(state.len() as i64);
        // This is black magic...
        if new_index == 0 {
            new_index = state.len() as i64;
        }

        state.insert(new_index as usize, elem);
    }
}
