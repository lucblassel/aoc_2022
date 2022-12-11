#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operator: char,
    value: Option<u64>,
    test: u64,
    result: (usize, usize),
}

impl Monkey {
    fn operation(&self, old: u64) -> u64 {
        match self.operator {
            '*' => match self.value {
                Some(val) => old * val,
                None => old * old,
            },
            '+' => match self.value {
                Some(val) => old + val,
                None => unreachable!("Cannot add old to old"),
            },
            _ => unreachable!("Unkown operator {}", self.operator),
        }
    }

    fn empty_items(&mut self) {
        self.items = vec![];
    }

    fn from_vec(vec: Vec<&str>) -> Self {
        let items: Vec<u64> = vec[1]
            .split_at(18)
            .1
            .split(", ")
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect();
        let test: u64 = vec[3].split_at(21).1.parse().unwrap();
        let target_true: usize = vec[4].split_at(29).1.parse().unwrap();
        let target_false: usize = vec[5].split_at(30).1.parse().unwrap();

        let op_s = vec[2].split_at(23).1;
        let (op, val_s) = op_s.split_at(1);
        let operator = op.chars().collect::<Vec<char>>()[0];
        let value = match val_s.trim().parse::<u64>() {
            Ok(val) => Some(val),
            Err(_) => None, // Use 'old' as the value
        };

        Monkey {
            items,
            operator,
            value,
            test,
            result: (target_true, target_false),
        }
    }
}

fn main() {
    println!("Day 11:");
    let input = include_str!("../inputs/example.txt");
    let mut line_buf = vec![];

    let mut monkeys: Vec<Monkey> = vec![];

    for (_, line) in input.lines().enumerate() {
        if line.is_empty() {
            monkeys.push(Monkey::from_vec(line_buf));
            line_buf = vec![];
            continue;
        }
        line_buf.push(line);
    }
    monkeys.push(Monkey::from_vec(line_buf));

    let modulo: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    let part_config: Vec<(Vec<Monkey>, u32, bool, u64)> = vec![
        (monkeys.clone(), 20, true, 3),
        (monkeys, 10_000, false, modulo),
    ];

    for (i, (monkeys, turns, divide, operand)) in part_config.into_iter().enumerate() {
        let mut monkeys = monkeys;
        let mut activity = vec![0; monkeys.len()];

        for _ in 1..=turns {
            for i in 0..monkeys.len() {
                let (new_worries, actions) = do_turn(&monkeys[i], divide, operand);

                for (item, target) in new_worries {
                    monkeys[target].items.push(item);
                }

                activity[i] += actions;
                monkeys[i].empty_items();
            }
        }

        activity.sort();
        let answer: u64 = activity.iter().rev().take(2).product();

        println!("\t{}) {answer}", i + 1);
    }
}

fn do_turn(monkey: &Monkey, divide: bool, operand: u64) -> (Vec<(u64, usize)>, u64) {
    let mut new_worries: Vec<(u64, usize)> = vec![];
    let mut activity = 0;

    for item in monkey.items.iter() {
        let new_worry = if divide {
            monkey.operation(*item) / operand
        } else {
            monkey.operation(*item) % operand
        };
        let target = if new_worry % monkey.test == 0 {
            monkey.result.0
        } else {
            monkey.result.1
        };
        new_worries.push((new_worry, target));
        activity += 1;
    }

    (new_worries, activity)
}
