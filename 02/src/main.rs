fn main() {
    println!("Day 2:");
    let input = include_str!("../inputs/input.txt");
    let mut score = 0;

    let mut moves: Vec<(&str, &str)> = vec![];

    for line in input.lines() {
        let (op_move, mut my_move) = line.split_at(1);
        my_move = my_move.strip_prefix(" ").unwrap();
        moves.push((op_move, my_move));

        match my_move {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => unreachable!("Unknown move:{my_move}"),
        };

        match (op_move, my_move) {
            ("A", "Y") | ("B", "Z") | ("C", "X") => score += 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => score += 3,
            _ => continue,
        }
    }
    println!("\t1) {score}");

    score = 0;
    for (op_move, my_move) in moves {
        match my_move {
            "Z" => {
                // Win
                score += 6;
                match op_move {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => unreachable!(),
                };
            }
            "Y" => {
                // Draw
                score += 3;
                match op_move {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => unreachable!(),
                }
            }
            "X" => {
                // Lose
                match op_move {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!("Unknown move {my_move}"),
        };
    }

    println!("\t2) {score}");
}
