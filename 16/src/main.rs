use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow_rates: HashMap<&str, u32> = HashMap::new();

    let flow_re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+);").unwrap();
    let path_re = Regex::new(r"valves? (([A-Z]{2},? ?)*)").unwrap();

    for line in input.lines() {
        let flow_cap = flow_re.captures(line).unwrap();
        let valve = flow_cap.get(1).unwrap().as_str();
        let rate = flow_cap.get(2).unwrap().as_str().parse().unwrap();
        flow_rates.insert(valve, rate);

        let path_cap = path_re.captures(line).unwrap();
        let valves = path_cap.get(1).unwrap().as_str().split(", ").collect();
        println!("{valve} ({rate}) -> {valves:?}");
        paths.insert(valve, valves);
    }

    let open = HashSet::from_iter(vec!["AA"]);
    let mut visited: HashMap<(String, u8, u32), u32> = HashMap::new();
    let flow = do_round("AA", 1, 0, open.clone(), &paths, &flow_rates, &mut visited);
    println!("\t1) {flow}");

    let mut visited2: HashMap<((String, String), u8, u32), u32> = HashMap::new();
    let flow = do_round_2(("AA", "AA"), 1, 0, open, &paths, &flow_rates, &mut visited2);
    println!("\t2) {flow}");
}

fn do_round(
    valve: &str,
    minute: u8,
    flow: u32,
    open: HashSet<&str>,
    paths: &HashMap<&str, Vec<&str>>,
    rates: &HashMap<&str, u32>,
    visited: &mut HashMap<(String, u8, u32), u32>,
) -> u32 {
    if let Some(flow) = visited.get(&(valve.to_owned(), minute, flow)) {
        return *flow;
    }

    if minute > 30 {
        return flow;
    };

    // Add flow from open valves
    let mut new_flow = flow
        + open
            .iter()
            .map(|valve| rates.get(valve).unwrap())
            .sum::<u32>();

    // Move
    let move_flow: u32 = paths
        .get(valve)
        .unwrap()
        .iter()
        .map(|valve| {
            do_round(
                valve,
                minute + 1,
                new_flow,
                open.clone(),
                paths,
                rates,
                visited,
            )
        })
        .max()
        .unwrap();
    // Open current valve
    let open_flow = if open.contains(valve) || *rates.get(valve).unwrap() == 0 {
        0
    } else {
        let mut open = open;
        open.insert(valve);
        do_round(
            valve,
            minute + 1,
            new_flow,
            open.clone(),
            paths,
            rates,
            visited,
        )
    };

    new_flow = open_flow.max(move_flow);
    visited.insert((valve.to_owned(), minute, flow), new_flow);

    new_flow
}

fn do_round_2(
    valves: (&str, &str),
    minute: u8,
    flow: u32,
    open: HashSet<&str>,
    paths: &HashMap<&str, Vec<&str>>,
    rates: &HashMap<&str, u32>,
    visited: &mut HashMap<((String, String), u8, u32), u32>,
) -> u32 {
    if let Some(flow) = visited.get(&((valves.0.to_owned(), valves.1.to_owned()), minute, flow)) {
        return *flow;
    }

    if minute > 26 {
        return flow;
    };

    // Add flow from open valves
    let mut new_flow = flow
        + open
            .iter()
            .map(|valve| rates.get(valve).unwrap())
            .sum::<u32>();

    // I move + elephant move
    let flow_1 = paths
        .get(valves.0)
        .unwrap()
        .iter()
        .map(|valve_me| {
            paths
                .get(valves.1)
                .unwrap()
                .iter()
                .map(|valve_elephant| {
                    do_round_2(
                        (valve_me, valve_elephant),
                        minute + 1,
                        new_flow,
                        open.clone(),
                        paths,
                        rates,
                        visited,
                    )
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    // I open + elephant move
    let mut open_1 = open.clone();
    open_1.insert(valves.0);
    let flow_2 = paths
        .get(valves.1)
        .unwrap()
        .iter()
        .map(|valve_elephant| {
            do_round_2(
                (valves.0, valve_elephant),
                minute + 1,
                new_flow,
                open_1.clone(),
                paths,
                rates,
                visited,
            )
        })
        .max()
        .unwrap();

    // I move + elephant open
    let mut open_2 = open.clone();
    open_2.insert(valves.1);
    let flow_3 = paths
        .get(valves.0)
        .unwrap()
        .iter()
        .map(|valve_me| {
            do_round_2(
                (valve_me, valves.1),
                minute + 1,
                new_flow,
                open_2.clone(),
                paths,
                rates,
                visited,
            )
        })
        .max()
        .unwrap();

    // I open + elephant open
    let mut open_3 = open.clone();
    open_3.insert(valves.0);
    open_3.insert(valves.1);
    let flow_4 = do_round_2(valves, minute + 1, new_flow, open_3, paths, rates, visited);

    new_flow = flow_1.max(flow_2).max(flow_3).max(flow_4);

    visited.insert(
        ((valves.0.to_owned(), valves.1.to_owned()), minute, flow),
        new_flow,
    );

    new_flow
}
