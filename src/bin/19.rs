use std::{
    collections::{HashSet, VecDeque},
    fs,
    hash::Hash,
    ops,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Inventory {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl Inventory {
    const NIL: Inventory = Inventory {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    const ORE: Inventory = Inventory {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    const CLAY: Inventory = Inventory {
        ore: 0,
        clay: 1,
        obsidian: 0,
        geode: 0,
    };
    const OBSIDIAN: Inventory = Inventory {
        ore: 0,
        clay: 0,
        obsidian: 1,
        geode: 0,
    };
    const GEODE: Inventory = Inventory {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 1,
    };

    fn gte(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
}

impl ops::Sub<Inventory> for Inventory {
    type Output = Inventory;

    fn sub(self, other: Inventory) -> Inventory {
        Inventory {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl ops::Add<Inventory> for Inventory {
    type Output = Inventory;

    fn add(self, other: Inventory) -> Inventory {
        Inventory {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    i: i32,
    ore_robot_cost: Inventory,
    clay_robot_cost: Inventory,
    obsidian_robot_cost: Inventory,
    geode_robot_cost: Inventory,
}

impl Blueprint {
    fn get_max_geodes(&self, t: i32, resources: Inventory, robots: Inventory) -> i32 {
        let mut queue = VecDeque::from([(t, resources, robots)]);
        let mut checked = HashSet::from([(t, resources, robots)]);

        macro_rules! enqueue {
            ($l:tt) => {
                if !checked.contains(&$l) {
                    checked.insert($l);
                    queue.push_back($l);
                }
            };
        }

        let mut max = 0;
        while let Some((t, resources, robots)) = queue.pop_front() {
            max = max.max(t * robots.geode as i32 + resources.geode as i32);

            let tmax = (t - 2) * (t - 1) / 2 + t * robots.geode as i32 + resources.geode as i32;
            if t == 0 || tmax < max {
                continue;
            }

            let next_resources = resources + robots;

            if resources.gte(&self.geode_robot_cost) {
                let next_resources = next_resources - self.geode_robot_cost;
                let next_robots = robots + Inventory::GEODE;
                enqueue!((t - 1, next_resources, next_robots));
                continue;
            }

            let mut branches = 0;

            if resources.gte(&self.ore_robot_cost) {
                let next_resources = next_resources - self.ore_robot_cost;
                let next_robots = robots + Inventory::ORE;
                enqueue!((t - 1, next_resources, next_robots));
                branches += 1;
            }

            if resources.gte(&self.clay_robot_cost) {
                let next_resources = next_resources - self.clay_robot_cost;
                let next_robots = robots + Inventory::CLAY;
                enqueue!((t - 1, next_resources, next_robots));
                branches += 1;
            }

            if resources.gte(&self.obsidian_robot_cost) {
                let next_resources = next_resources - self.obsidian_robot_cost;
                let next_robots = robots + Inventory::OBSIDIAN;
                enqueue!((t - 1, next_resources, next_robots));
                branches += 1;
            }

            if branches < 3 {
                enqueue!((t - 1, next_resources, robots));
            }
        }
        max
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/19.txt").unwrap();

    let blueprints = input
        .split('\n')
        .filter(|t| !t.is_empty())
        .map(|t| {
            let (robot, resources) = t.split_once(": ").unwrap();
            let (_, i) = robot.split_once(' ').unwrap();
            let resources = resources.split(' ').collect::<Vec<_>>();
            Blueprint {
                i: i.parse().unwrap(),
                ore_robot_cost: Inventory {
                    ore: resources[4].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_robot_cost: Inventory {
                    ore: resources[10].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_robot_cost: Inventory {
                    ore: resources[16].parse().unwrap(),
                    clay: resources[19].parse().unwrap(),
                    obsidian: 0,
                    geode: 0,
                },
                geode_robot_cost: Inventory {
                    ore: resources[25].parse().unwrap(),
                    clay: 0,
                    obsidian: resources[28].parse().unwrap(),
                    geode: 0,
                },
            }
        })
        .collect::<Vec<_>>();

    let mut part1_sum = 0;
    for b in blueprints.iter() {
        let geodes = b.get_max_geodes(24, Inventory::NIL, Inventory::ORE);
        println!("index {:?} geodes {:?}", b.i, geodes);
        part1_sum += b.i * geodes;
    }
    println!("part1: {:?}", part1_sum);

    let mut part2_product = 1;
    for b in blueprints.iter().take(3) {
        let geodes = b.get_max_geodes(32, Inventory::NIL, Inventory::ORE);
        println!("index {:?} geodes {:?}", b.i, geodes);
        part2_product *= geodes;
    }
    println!("part2: {:?}", part2_product);
}
