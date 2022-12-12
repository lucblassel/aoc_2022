use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut adjacency: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut heights: HashMap<(usize, usize), i32> = HashMap::new();
    let mut source: (usize, usize) = (0, 0);
    let mut sink: (usize, usize) = (0, 0);
    let mut sources: Vec<(usize, usize)> = vec![source];

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            adjacency.insert((x, y), HashSet::new());
            match c {
                'S' => {
                    source = (x, y);
                    heights.insert((x, y), ('a' as u8 - 97) as i32)
                }
                'E' => {
                    sink = (x, y);
                    heights.insert((x, y), ('z' as u8 - 97) as i32)
                }
                'a' => {
                    sources.push((x, y));
                    heights.insert((x, y), ('a' as u8 - 97) as i32)
                }
                c => heights.insert((x, y), (c as u8 - 97) as i32),
            };
        }
    }

    for ((x, y), height) in heights.iter() {
        let mut neighbors: Vec<(usize, usize)> = vec![(x + 1, *y), (*x, 1 + y)];

        if *x > 0 {
            neighbors.push((x - 1, *y))
        }
        if *y > 0 {
            neighbors.push((*x, y - 1))
        }
        for (x_n, y_n) in neighbors {
            match heights.get(&(x_n, y_n)) {
                Some(height_n) => {
                    if height_n - height <= 1 {
                        adjacency.entry((*x, *y)).and_modify(|set| {
                            set.insert((x_n, y_n));
                        });
                    };
                }
                None => continue,
            };
        }
    }

    let length = find_path(&adjacency, source, sink);
    println!("\t1) {length}");

    let length_2 = sources.iter().map(|source| find_path(&adjacency, *source, sink)).min().unwrap();
    println!("\t2) {length_2}");
}

fn find_path(
    adjacency: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    source: (usize, usize),
    sink: (usize, usize),
) -> i32 {
    let mut distances: HashMap<(usize, usize), i32> = HashMap::new();
    let mut queue: Vec<(i32, (usize, usize))> = vec![];
    let mut predecessors: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    distances.insert(source, 0);
    for node in adjacency.keys() {
        if *node != source {
            distances.insert(*node, 10_000);
        }
        queue.push((*distances.get(node).unwrap(), *node));
    }

    queue.sort();

    while !queue.is_empty() {
        let (_, node) = queue[0];
        queue.remove(0);
        let neighbors = adjacency.get(&node).unwrap();
        for neighbor in neighbors {
            let new_dist = distances.get(&node).unwrap() + 1;
            if new_dist < *distances.get(neighbor).unwrap() {
                distances.insert(*neighbor, new_dist);
                predecessors.insert(*neighbor, node);
                let index = queue.iter().position(|(_, node)| node == neighbor).unwrap();
                queue.remove(index);
                queue.push((new_dist, *neighbor));
                queue.sort();
            }
        }
    }

    let mut length = 0;
    let mut next = sink;

    while next != source {
        length += 1;
        next = match predecessors.get(&next) {
            Some(x) => *x,
            None => return i32::MAX,
        };
    }

    length
}
