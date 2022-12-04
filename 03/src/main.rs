use std::collections::{HashMap, HashSet};
fn main() {
    println!("Day 3:");
    let input = include_str!("../inputs/input.txt");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priorities: HashMap<char, usize> =
        HashMap::from_iter(alphabet.chars().enumerate().map(|(i, c)| (c, i + 1)));

    let mut score = 0;

    for line in input.lines() {
        let (s1, s2) = line.split_at(line.len() / 2);

        let comp_1: HashSet<char> = HashSet::from_iter(s1.chars());
        let comp_2: HashSet<char> = HashSet::from_iter(s2.chars());

        score += comp_1
            .intersection(&comp_2)
            .map(|v| priorities.get(v).unwrap())
            .sum::<usize>();
    }

    println!("\t1) {score}");

    let mut groups: Vec<Vec<HashSet<char>>> = vec![];
    let mut sets: Vec<HashSet<char>> = vec![];

    for (i, line) in input.lines().enumerate() {
        if i % 3 == 0 && i != 0 {
            groups.push(sets);
            sets = vec![];
        }
        sets.push(HashSet::from_iter(line.chars()));
    }
    groups.push(sets);

    score = groups
        .iter()
        .map(|sets| {
            let element = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });
            priorities.get(element.iter().next().unwrap()).unwrap()
        })
        .sum();

    println!("\t2) {score}");
}
