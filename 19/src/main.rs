use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use std::ops::{Add, Mul};

lazy_static! {
    static ref ROBOT_RE: Regex = Regex::new(r"Each \w+ robot costs (.+)").unwrap();
    static ref COST_RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
    static ref ORE_MAP: HashMap<&'static str, usize> =
        vec![("ore", 0), ("clay", 1), ("obsidian", 2), ("geode", 3),]
            .iter()
            .copied()
            .collect();
}

fn main() {
    println!("Day 19:");
    let input = include_str!("../inputs/input.txt");

    let blueprints: Vec<_> = input.lines().map(parse_blueprint).collect();

    let answer_1: usize = blueprints
        .iter()
        .map(|blueprint| {
            let mut max = 0;
            solve(blueprint, State::new(24), &mut max);
            max as usize * blueprint.id
        })
        .sum();
    println!("\t1) {answer_1}");

    let answer_2: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let mut max = 0;
            solve(blueprint, State::new(32), &mut max);
            max as usize
        })
        .product();

    println!("\t2) {answer_2}");
}

fn solve(blueprint: &Blueprint, state: State, max_geodes: &mut u8) {
    *max_geodes = state.geodes.max(*max_geodes);
    for child_state in state.branch(blueprint) {
        if child_state.compute_upper_bound(blueprint) > *max_geodes {
            solve(blueprint, child_state, max_geodes)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: usize,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

impl Resources {
    fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(other.ore)?,
            clay: self.clay.checked_sub(other.clay)?,
            obsidian: self.obsidian.checked_sub(other.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    time_left: u8,
    geodes: u8,
    resources: Resources,
    built_robots: Resources,
}

impl State {
    fn new(time_left: u8) -> Self {
        Self {
            time_left,
            geodes: 0,
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
            },
            built_robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
            },
        }
    }

    fn build_robot(self, cost: Resources, robot: Resources) -> Option<Self> {
        let mut passed = 0;
        while passed < self.time_left {
            let resources = self.resources + self.built_robots * passed;
            if let Some(new_resources) = resources.checked_sub(cost) {
                return Some(Self {
                    time_left: self.time_left - passed - 1,
                    resources: new_resources + self.built_robots,
                    built_robots: self.built_robots + robot,
                    ..self
                });
            }
            passed += 1;
        }

        // Robot is not buildable in available time
        None
    }

    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        let max_ore_cost = blueprint
            .clay_robot
            .ore
            .max(blueprint.obsidian_robot.ore)
            .max(blueprint.geode_robot.ore);

        let mut buildable = vec![];

        // Ore robot is theoretically buildable
        if self.built_robots.ore < max_ore_cost {
            buildable.push(self.build_robot(
                blueprint.ore_robot,
                Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                },
            ))
        }

        // Clay robot is theoretically buildable
        if self.built_robots.clay < blueprint.obsidian_robot.clay {
            buildable.push(self.build_robot(
                blueprint.clay_robot,
                Resources {
                    ore: 0,
                    clay: 1,
                    obsidian: 0,
                },
            ))
        }

        // Obsidian robot is theoretically buildable
        if self.built_robots.obsidian < blueprint.geode_robot.obsidian && self.built_robots.clay > 0
        {
            buildable.push(self.build_robot(
                blueprint.obsidian_robot,
                Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 1,
                },
            ))
        }

        // Geode robot is theoretically buildable
        if self.built_robots.obsidian > 0 {
            buildable.push(
                self.build_robot(
                    blueprint.geode_robot,
                    Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                    },
                )
                .map(|state| Self {
                    geodes: state.geodes + state.time_left,
                    ..state
                }),
            )
        }

        buildable.into_iter().flatten()
    }

    fn compute_upper_bound(self, blueprint: &Blueprint) -> u8 {
        let geode_cost = blueprint.geode_robot.obsidian;

        let mut obsidian = self.resources.obsidian;
        let mut obsidian_robots = self.built_robots.obsidian;
        let mut geodes = self.geodes;

        for time_left in (0..self.time_left).rev() {
            if obsidian >= geode_cost {
                obsidian = obsidian + obsidian_robots - geode_cost;
                geodes = geodes.saturating_add(time_left);
            } else {
                obsidian += obsidian_robots;
                obsidian_robots += 1;
            }
        }

        geodes
    }
}

// PARSING CODE
fn parse_cost(cost: &str) -> (&str, u8) {
    let captures = COST_RE.captures(cost).unwrap();
    let cost = captures.get(1).unwrap().as_str().parse().unwrap();
    let ore = captures.get(2).unwrap().as_str();

    (ore, cost)
}

fn parse_robot(entry: &str) -> Resources {
    let captures = ROBOT_RE.captures(entry).unwrap();
    let costs: HashMap<&str, _> = captures
        .get(1)
        .unwrap()
        .as_str()
        .split(" and ")
        .map(parse_cost)
        .collect();

    Resources {
        ore: *costs.get("ore").unwrap_or(&0),
        clay: *costs.get("clay").unwrap_or(&0),
        obsidian: *costs.get("obsidian").unwrap_or(&0),
    }
}

fn parse_blueprint(line: &str) -> Blueprint {
    let fields = line.split(": ").collect::<Vec<_>>();
    let robots: Vec<_> = fields[1].split(". ").map(parse_robot).collect();

    Blueprint {
        id: fields[0].split_at(10).1.parse().unwrap(),
        ore_robot: robots[0],
        clay_robot: robots[1],
        obsidian_robot: robots[2],
        geode_robot: robots[3],
    }
}
