use regex::Regex;

#[derive(Debug, Clone)]
struct FileNode {
    name: String,
    children: Vec<usize>,
    size: u32,
    is_dir: bool,
}

impl FileNode {
    fn new_dir(name: String) -> Self {
        FileNode {
            name,
            children: vec![],
            size: 0,
            is_dir: true,
        }
    }
    fn new_file(name: String, size: u32) -> Self {
        FileNode {
            name,
            children: vec![],
            size,
            is_dir: false,
        }
    }
}

fn main() {
    println!("Day 7:");
    let input = include_str!("../inputs/input.txt");

    let mut dir_stack: Vec<usize> = vec![];

    let cd_regex = Regex::new(r"\$ cd (.+)").unwrap();
    let dir_regex = Regex::new(r"dir (.+)").unwrap();
    let file_regex = Regex::new(r"(\d+) (.+)").unwrap();

    let mut all_nodes: Vec<FileNode> = vec![];
    let mut curr_index: usize = 0;

    all_nodes.push(FileNode::new_dir("/".to_owned()));
    dir_stack.push(curr_index);

    let mut iter = input.lines();
    iter.next();

    for line in iter {
        if line == "$ ls" {
            continue;
        }
        if let Some(caps) = cd_regex.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_owned();
            if name == ".." {
                dir_stack.pop();
                continue;
            }
            let parent_index = dir_stack.last().unwrap();
            let parent = all_nodes[*parent_index].clone();
            for child_index in parent.children {
                if all_nodes[child_index].name == name {
                    dir_stack.push(child_index);
                    continue;
                }
            }
        } else if let Some(caps) = dir_regex.captures(line) {
            let parent = dir_stack.last().unwrap();
            let name = caps.get(1).unwrap().as_str().to_owned();
            curr_index += 1;
            all_nodes[*parent].children.push(curr_index);
            all_nodes.push(FileNode::new_dir(name));
        } else if let Some(caps) = file_regex.captures(line) {
            let parent = dir_stack.last().unwrap();
            let size: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let name = caps.get(2).unwrap().as_str().to_owned();

            curr_index += 1;
            all_nodes[*parent].children.push(curr_index);
            all_nodes.push(FileNode::new_file(name, size));
        } else {
            panic!("Problem with your regexes...")
        }
    }

    get_size(&mut all_nodes, 0);

    let answer_1: u32 = all_nodes
        .iter()
        .filter(|node| node.is_dir && node.size <= 100000)
        .map(|node| node.size)
        .sum();

    println!("\t1) {answer_1}");

    let free = 70_000_000 - all_nodes[0].size;
    let to_free = 30_000_000 - free;

    let answer_2 = all_nodes
        .iter()
        .filter(|node| node.is_dir && node.size >= to_free)
        .map(|node| node.size)
        .min()
        .unwrap();

    println!("\t2) {answer_2}");
}

fn get_size(nodes: &mut Vec<FileNode>, current_dir: usize) -> u32 {
    let mut size = 0;

    let children = nodes[current_dir].children.clone();

    for child_index in children {
        if nodes[child_index].is_dir && nodes[child_index].size == 0 {
            size += get_size(nodes, child_index);
        } else {
            size += nodes[child_index].size;
        }
    }

    nodes[current_dir].size = size;
    size
}
