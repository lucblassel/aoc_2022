use std::collections::HashMap;

use regex::Regex;

struct Graph<'a> {
    distances: HashMap<(&'a str, &'a str), i64>,
    rates: HashMap<&'a str, i64>,
    masks: HashMap<&'a str, i64>,
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut rates: HashMap<&str, i64> = HashMap::new();

    let flow_re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+);").unwrap();
    let path_re = Regex::new(r"valves? (([A-Z]{2},? ?)*)").unwrap();

    for line in input.lines() {
        let flow_cap = flow_re.captures(line).unwrap();
        let valve = flow_cap.get(1).unwrap().as_str();
        let rate = flow_cap.get(2).unwrap().as_str().parse().unwrap();
        if rate != 0 {
            rates.insert(valve, rate);
        }

        let valves = path_re
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .collect();
        graph.insert(valve, valves);
    }

    let masks: HashMap<&str, i64> =
        HashMap::from_iter(rates.keys().enumerate().map(|(i, valve)| (*valve, 1 << i)));

    let mut distances: HashMap<(&str, &str), i64> =
        HashMap::from_iter(graph.keys().flat_map(|valve1| {
            graph.keys().map(|valve2| {
                if graph.get(valve1).unwrap().contains(valve2) {
                    ((*valve1, *valve2), 1)
                } else {
                    ((*valve1, *valve2), 10_000)
                }
            })
        }));

    // Computing shortest distance between nodes
    for v1 in graph.keys() {
        for v2 in graph.keys() {
            for v3 in graph.keys() {
                let current = *distances.get(&(v2, v3)).unwrap();
                let new = distances.get(&(v2, v1)).unwrap() + distances.get(&(v1, v3)).unwrap();

                distances.insert((v2, v3), current.min(new));
            }
        }
    }

    let graph = Graph {
        distances,
        rates,
        masks,
    };

    let mut cache = HashMap::new();
    do_round("AA", 30, 0, 0, &mut cache, &graph);
    let answer_1 = cache.values().max().unwrap();
    println!("\t1) {answer_1}");

    let mut cache = HashMap::new();
    do_round("AA", 26, 0, 0, &mut cache, &graph);

    let mut possible: Vec<i64> = vec![];
    for (state1, flow1) in cache.iter() {
        for (state2, flow2) in cache.iter() {
            if (state1 & state2) == 0 {
                possible.push(flow1 + flow2)
            }
        }
    }
    let answer_2 = possible.iter().max().unwrap();
    println!("\t2) {answer_2}");
}

fn do_round(
    valve: &str,
    minutes: i64,
    state: i64,
    flow: i64,
    cache: &mut HashMap<i64, i64>,
    graph: &Graph,
) {
    let v = match cache.get(&state) {
        None => flow.max(0),
        Some(v) => flow.max(*v),
    };
    cache.insert(state, v);

    for valve2 in graph.rates.keys() {
        let minutes = minutes - graph.distances.get(&(valve, valve2)).unwrap() - 1;
        let mask = graph.masks.get(valve2).unwrap();
        if (mask & state) != 0 || minutes < 0 {
            continue;
        }
        let flow = flow + minutes * graph.rates.get(valve2).unwrap();
        do_round(valve2, minutes, state | mask, flow, cache, graph);
    }
}
