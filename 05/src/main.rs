use regex::Regex;
use std::collections::HashMap;
fn main() {
    println!("Day 5");
    let input = include_str!("../inputs/input.txt");
    let mut stacks: HashMap<String, Vec<char>> = HashMap::new();

    let mut stack_lines: Vec<&str> = vec![];
    let mut moves: Vec<&str> = vec![];
    let mut is_stack = true;

    for line in input.lines() {
        if line.is_empty() {
            is_stack = false;
            continue;
        }
        if is_stack {
            stack_lines.push(line);
        } else {
            moves.push(line);
        }
    }

    let identifiers = stack_lines.pop();

    for line in stack_lines.iter() {
        for (i, c) in line.chars().enumerate() {
            if i % 4 == 1 && c != ' ' {
                let identifier = format!("{}", i / 4 + 1);
                stacks.entry(identifier).or_default().push(c);
            }
        }
    }

    let mut stacks_part_1: HashMap<String, Vec<char>> = HashMap::new();
    for k in stacks.keys() {
        let mut v = stacks.get(k).unwrap().clone();
        v.reverse();
        stacks_part_1.insert(k.clone(), v);
    }
    let mut stacks_part_2 = stacks_part_1.clone();

    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    for line in moves.iter() {
        let caps = re.captures(line).unwrap();
        let amount: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let identifier_from = caps.get(2).unwrap().as_str().to_owned();
        let identifier_to = caps.get(3).unwrap().as_str().to_owned();

        let mut stack_from_1 = stacks_part_1.get(&identifier_from).unwrap().clone();
        let mut stack_from_2 = stacks_part_2.get(&identifier_from).unwrap().clone();
        let mut stack_to_1 = stacks_part_1.get(&identifier_to).unwrap().clone();
        let mut stack_to_2 = stacks_part_2.get(&identifier_to).unwrap().clone();

        let mut temp: Vec<char> = vec![];
        let mut temp2: Vec<char> = vec![];
        for _ in 0..amount {
            temp.push(stack_from_1.pop().unwrap());
            temp2.push(stack_from_2.pop().unwrap());
        }

        temp2.reverse();

        for c in temp {
            stack_to_1.push(c);
        }

        for c in temp2 {
            stack_to_2.push(c);
        }

        stacks_part_1.insert(identifier_from.clone(), stack_from_1.clone());
        stacks_part_1.insert(identifier_to.clone(), stack_to_1);
        stacks_part_2.insert(identifier_from, stack_from_2);
        stacks_part_2.insert(identifier_to, stack_to_2);
    }

    print!("\t1) ");
    for identifier in identifiers.unwrap().split_ascii_whitespace() {
        print!("{}", stacks_part_1.get(identifier).unwrap().last().unwrap());
    }
    println!();
    print!("\t2) ");
    for identifier in identifiers.unwrap().split_ascii_whitespace() {
        print!("{}", stacks_part_2.get(identifier).unwrap().last().unwrap());
    }
    println!();
}
