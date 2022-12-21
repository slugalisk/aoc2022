use std::{collections::HashMap, fs};

#[derive(Debug)]
enum MonkeyJob<'a> {
    Op {
        a_name: &'a str,
        b_name: &'a str,
        op: &'a str,
    },
    Value(i64),
}

fn resolve_value(name: &str, monkeys: &HashMap<&str, MonkeyJob>) -> i64 {
    match monkeys.get(name).unwrap() {
        MonkeyJob::Op { a_name, b_name, op } => match *op {
            "+" => resolve_value(a_name, monkeys) + resolve_value(b_name, monkeys),
            "-" => resolve_value(a_name, monkeys) - resolve_value(b_name, monkeys),
            "*" => resolve_value(a_name, monkeys) * resolve_value(b_name, monkeys),
            "/" => resolve_value(a_name, monkeys) / resolve_value(b_name, monkeys),
            _ => panic!("invalid operator"),
        },
        MonkeyJob::Value(v) => *v,
    }
}

fn find_target(target: &str, name: &str, monkeys: &HashMap<&str, MonkeyJob>) -> bool {
    match monkeys.get(name).unwrap() {
        MonkeyJob::Op {
            a_name,
            b_name,
            op: _,
        } => find_target(target, a_name, monkeys) || find_target(target, b_name, monkeys),
        MonkeyJob::Value(_) => target == name,
    }
}

fn find_target_value(
    target: &str,
    target_value: i64,
    name: &str,
    monkeys: &HashMap<&str, MonkeyJob>,
) -> i64 {
    match monkeys.get(name).unwrap() {
        MonkeyJob::Op { a_name, b_name, op } => {
            if find_target(target, a_name, monkeys) {
                let target_value = match *op {
                    "+" => target_value - resolve_value(b_name, monkeys),
                    "-" => target_value + resolve_value(b_name, monkeys),
                    "*" => target_value / resolve_value(b_name, monkeys),
                    "/" => target_value * resolve_value(b_name, monkeys),
                    _ => panic!("invalid operator"),
                };
                find_target_value(target, target_value, a_name, monkeys)
            } else {
                let target_value = match *op {
                    "+" => target_value - resolve_value(a_name, monkeys),
                    "-" => resolve_value(a_name, monkeys) - target_value,
                    "*" => target_value / resolve_value(a_name, monkeys),
                    "/" => resolve_value(a_name, monkeys) / target_value,
                    _ => panic!("invalid operator"),
                };
                find_target_value(target, target_value, b_name, monkeys)
            }
        }
        MonkeyJob::Value(_) => target_value,
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/21.txt").unwrap();
    let monkeys = input
        .split('\n')
        .filter(|t| !t.is_empty())
        .map(|t| {
            let (name, job) = t.split_once(": ").unwrap();
            let job = if let Ok(n) = job.parse::<i64>() {
                MonkeyJob::Value(n)
            } else {
                let p = job.split(' ').collect::<Vec<_>>();
                MonkeyJob::Op {
                    a_name: p[0],
                    b_name: p[2],
                    op: p[1],
                }
            };
            (name, job)
        })
        .collect::<HashMap<_, _>>();

    println!("part1: {:?}", resolve_value("root", &monkeys));

    let target = "humn";
    if let Some(MonkeyJob::Op {
        a_name,
        b_name,
        op: _,
    }) = monkeys.get("root")
    {
        let part2_n = if find_target(target, a_name, &monkeys) {
            find_target_value(target, resolve_value(b_name, &monkeys), a_name, &monkeys)
        } else {
            find_target_value(target, resolve_value(a_name, &monkeys), b_name, &monkeys)
        };
        println!("part2: {:?}", part2_n);
    }
}
