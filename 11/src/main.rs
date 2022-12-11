#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test: u64,
    result: (usize, usize),
}

impl Monkey {
    fn empty_items(&mut self) {
        self.items = vec![];
    }
}

fn main() {

    println!("Day 11:");
    // let mut monkeys: Vec<Monkey> = vec![
    //     Monkey {
    //         items: vec![79, 98],
    //         operation: |old| old * 19,
    //         test: 23,
    //         result: (2, 3),
    //     },
    //     Monkey {
    //         items: vec![54, 65, 75, 74],
    //         operation: |old| old + 6,
    //         test: 19,
    //         result: (2, 0),
    //     },
    //     Monkey {
    //         items: vec![79, 60, 97],
    //         operation: |old| old * old,
    //         test: 13,
    //         result: (1, 3),
    //     },
    //     Monkey {
    //         items: vec![74],
    //         operation: |old| old + 3,
    //         test: 17,
    //         result: (0, 1),
    //     },
    // ];

    let mut monkeys = vec![
        Monkey {
            items: vec![74, 64, 74, 63, 53],
            operation: |old| old * 7,
            test: 5,
            result: (1, 6),
        },
        Monkey {
            items: vec![69, 99, 95, 62],
            operation: |old| old * old,
            test: 17,
            result: (2, 5),
        },
        Monkey {
            items: vec![59, 81],
            operation: |old| old + 8,
            test: 7,
            result: (4, 3),
        },
        Monkey {
            items: vec![50, 67, 63, 57, 63, 83, 97],
            operation: |old| old + 4,
            test: 13,
            result: (0, 7),
        },
        Monkey {
            items: vec![61, 94, 85, 52, 81, 90, 94, 70],
            operation: |old| old + 3,
            test: 19,
            result: (7, 3),
        },
        Monkey {
            items: vec![69],
            operation: |old| old + 5,
            test: 3,
            result: (4, 2),
        },
        Monkey {
            items: vec![54, 55, 58],
            operation: |old| old + 7,
            test: 11,
            result: (1, 5),
        },
        Monkey {
            items: vec![79, 51, 83, 88, 93, 76],
            operation: |old| old * 3,
            test: 2,
            result: (0, 6),
        },
    ];

    let p2_modulo: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    let mut monkeys_2 = monkeys.clone();

    let mut activity = vec![0; monkeys.len()];

    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            let mut new_worries: Vec<(u64, usize)> = vec![];

            for item in monkeys[i].items.iter() {
                let new_worry = (monkeys[i].operation)(*item) / 3;
                let target = if new_worry % monkeys[i].test == 0 {
                    monkeys[i].result.0
                } else {
                    monkeys[i].result.1
                };
                new_worries.push((new_worry, target));
                activity[i] += 1;
            }

            for (item, target) in new_worries {
                monkeys[target].items.push(item);
            }

            monkeys[i].empty_items();
        }
    }

    activity.sort();
    let answer_1: u64 = activity.iter().rev().take(2).product();

    println!("\t1) {answer_1}");

    let mut activity = vec![0; monkeys_2.len()];

    for _ in 1..=10000 {
        for i in 0..monkeys_2.len() {
            let mut new_worries: Vec<(u64, usize)> = vec![];

            for item in monkeys_2[i].items.iter() {
                let new_worry = (monkeys_2[i].operation)(*item) % p2_modulo;
                let target = if new_worry % monkeys[i].test == 0 {
                    monkeys_2[i].result.0
                } else {
                    monkeys_2[i].result.1
                };
                new_worries.push((new_worry, target));
                activity[i] += 1;
            }

            for (item, target) in new_worries {
                monkeys_2[target].items.push(item);
            }

            monkeys_2[i].empty_items();
        }
    }

    activity.sort();
    let answer_2: u64 = activity.iter().rev().take(2).product();

    println!("\t2) {answer_2}");
}

fn print_monkeys(monkeys: &[Monkey]) {
    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i}: {:?}", monkey.items);
    }
}
