use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Monkey<'a> {
    Num(i64),
    Op((&'a str, &'a str, Operation)),
}

#[derive(Debug)]
enum Operation {
    Add,
    Substract,
    Multiply,
    Divide,
}

use crate::Operation::*;

fn main() {
    println!("Day 21:");
    let input = include_str!("../inputs/input.txt");

    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, value) = line.split_at(4);
        let (_, value) = value.split_at(2);
        if value.len() == 11 {
            let tokens: Vec<_> = value.split_ascii_whitespace().collect();
            let op = match tokens[1] {
                "+" => Add,
                "-" => Substract,
                "*" => Multiply,
                "/" => Divide,
                _ => unreachable!("Unknown operator {}", tokens[1]),
            };
            monkeys.insert(name, Monkey::Op((tokens[0], tokens[2], op)));
        } else {
            monkeys.insert(name, Monkey::Num(value.parse().unwrap()));
        }
    }

    let mut cache = HashMap::new();
    let mut uses_human_input = HashSet::new();
    let (answer_1, _) = get_result("root", &monkeys, &mut cache, &mut uses_human_input).unwrap();

    println!("\t1) {answer_1:?}");

    if let Monkey::Op((monkey1, monkey2, _)) = monkeys.get("root").unwrap() {
        if cache.get(monkey1) == cache.get(monkey2) {}
        let (target, root) = if uses_human_input.contains(monkey1) {
            (cache.get(monkey2).unwrap(), monkey1)
        } else {
            (cache.get(monkey1).unwrap(), monkey2)
        };

        get_human_input(root, *target, &monkeys, &cache, &uses_human_input);
    }
}

fn get_result<'a>(
    name: &'a str,
    monkeys: &HashMap<&'a str, Monkey<'a>>,
    cache: &mut HashMap<&'a str, i64>,
    uses_humn: &mut HashSet<&'a str>,
) -> Result<(i64, bool), ()> {
    if let Some(val) = cache.get(name) {
        return Ok((*val, uses_humn.contains(name)));
    }

    if let Some(monkey) = monkeys.get(name) {
        let result = match monkey {
            Monkey::Num(value) => {
                let human_input = name == "humn";
                (*value, human_input)
            }
            Monkey::Op((monkey1, monkey2, op)) => {
                let (val1, human_1) = get_result(monkey1, monkeys, cache, uses_humn)?;
                let (val2, human_2) = get_result(monkey2, monkeys, cache, uses_humn)?;
                let val = match op {
                    Add => val1 + val2,
                    Substract => val1 - val2,
                    Multiply => val1 * val2,
                    Divide => val1 / val2,
                };
                (val, human_1 || human_2)
            }
        };

        if result.1 {
            uses_humn.insert(name);
        }
        cache.insert(name, result.0);

        Ok(result)
    } else {
        Err(())
    }
}

fn get_human_input(
    root: &str,
    target: i64,
    monkeys: &HashMap<&str, Monkey>,
    cache: &HashMap<&str, i64>,
    uses_human_input: &HashSet<&str>,
) {
    if root == "humn" {
        println!("\t2) {target}");
    }

    match monkeys.get(root).unwrap() {
        Monkey::Num(_) => {}
        Monkey::Op((monkey1, monkey2, op)) => {
            let (new_root, human_first, other) = if uses_human_input.contains(monkey1) {
                (monkey1, true, cache.get(monkey2).unwrap())
            } else {
                (monkey2, false, cache.get(monkey1).unwrap())
            };

            let new_target = match op {
                Add => target - other,
                Substract => {
                    if human_first {
                        target + other
                    } else {
                        other - target
                    }
                }
                Multiply => target / other,
                Divide => {
                    if human_first {
                        target * other
                    } else {
                        other / target
                    }
                }
            };
            get_human_input(new_root, new_target, monkeys, cache, uses_human_input)
        }
    }
}
