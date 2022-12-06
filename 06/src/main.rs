use std::collections::{HashSet, VecDeque};

fn main() {
    println!("Day 6:");
    let input = include_str!("../inputs/input.txt");
    let mut start_buf: VecDeque<char> = VecDeque::new();
    let mut message_buf: VecDeque<char> = VecDeque::new();

    let mut found_start = false;

    for (i, char) in input.chars().enumerate() {
        start_buf.push_back(char);
        message_buf.push_back(char);

        if start_buf.len() < 4 {
            continue;
        }

        let set: HashSet<&char> = HashSet::from_iter(start_buf.iter());
        if !found_start && set.len() == 4 {
            println!("\t1) {}", i + 1);
            found_start = true;
        }

        start_buf.pop_front();

        if message_buf.len() >= 14 {
            let set: HashSet<&char> = HashSet::from_iter(message_buf.iter());
            if set.len() == 14 {
                println!("\t2) {}", i + 1);
                break;
            }
    
            message_buf.pop_front();
        }
    }
}
