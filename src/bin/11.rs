use std::collections::VecDeque;
use std::fs;

use itertools::Itertools;

#[derive(Debug)]
enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Debug)]
enum Operand {
    Old,
    Int { v: i64 },
}

#[derive(Debug)]
struct Monkey {
    pub id: i64,
    pub items: VecDeque<i64>,
    pub op_operator: Operator,
    pub op_a_operand: Operand,
    pub op_b_operand: Operand,
    pub condition_arg: i64,
    pub if_true: usize,
    pub if_false: usize,
    pub items_inspected: i64,
}

impl Monkey {
    fn pass_item(&mut self, div: i64) -> Option<(usize, i64)> {
        if let Some(item) = self.items.pop_front() {
            self.items_inspected += 1;

            let a_operand = match self.op_a_operand {
                Operand::Old => item,
                Operand::Int { v } => v,
            };
            let b_operand = match self.op_b_operand {
                Operand::Old => item,
                Operand::Int { v } => v,
            };

            let item = match self.op_operator {
                Operator::Multiply => a_operand * b_operand,
                Operator::Divide => a_operand / b_operand,
                Operator::Add => a_operand + b_operand,
                Operator::Subtract => a_operand - b_operand,
            } / div;

            let condition_result = item % self.condition_arg == 0;
            let dst = match condition_result {
                true => self.if_true,
                false => self.if_false,
            };

            return Some((dst, item));
        }
        None
    }

    fn recv_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
}

fn get_activity_score(monkeys: &Vec<Monkey>) -> i64 {
    let most_active = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .collect::<Vec<_>>();
    most_active[0] * most_active[1]
}

fn load_monkeys() -> Vec<Monkey> {
    let input = fs::read_to_string("./inputs/11.txt").unwrap();
    input
        .split("\n\n")
        .filter(|t| !t.is_empty())
        .map(|t| {
            let mut p = t.split('\n');
            let id = p
                .next()
                .unwrap()
                .trim_end_matches(':')
                .split(' ')
                .last()
                .map(|t| t.parse().unwrap())
                .unwrap();
            let (_, items) = p.next().unwrap().split_once(": ").unwrap();
            let items = items.split(", ").map(|t| t.parse().unwrap()).collect();
            let mut op = p.next().unwrap().split(' ').rev();
            let op_b_operand = op
                .next()
                .map(|t| match t {
                    "old" => Operand::Old {},
                    &_ => Operand::Int {
                        v: t.parse().unwrap(),
                    },
                })
                .unwrap();
            let op_operator = op
                .next()
                .map(|t| match t {
                    "*" => Some(Operator::Multiply {}),
                    "/" => Some(Operator::Divide {}),
                    "+" => Some(Operator::Add {}),
                    "-" => Some(Operator::Subtract {}),
                    &_ => None,
                })
                .unwrap()
                .unwrap();
            let op_a_operand = op
                .next()
                .map(|t| match t {
                    "old" => Operand::Old {},
                    &_ => Operand::Int {
                        v: t.parse().unwrap(),
                    },
                })
                .unwrap();
            let condition_arg = p
                .next()
                .unwrap()
                .split(' ')
                .last()
                .map(|t| t.parse().unwrap())
                .unwrap();
            let if_true = p
                .next()
                .unwrap()
                .split(' ')
                .last()
                .map(|t| t.parse().unwrap())
                .unwrap();
            let if_false = p
                .next()
                .unwrap()
                .split(' ')
                .last()
                .map(|t| t.parse().unwrap())
                .unwrap();
            Monkey {
                id,
                items,
                op_operator,
                op_a_operand,
                op_b_operand,
                condition_arg,
                if_true,
                if_false,
                items_inspected: 0,
            }
        })
        .collect()
}

fn main() {
    let mut monkeys = load_monkeys();
    for _i in 0..20 {
        for j in 0..monkeys.len() {
            while let Some((dst, item)) = monkeys.get_mut(j).unwrap().pass_item(3) {
                monkeys.get_mut(dst as usize).unwrap().recv_item(item);
            }
        }
    }

    println!("part1: {:?}", get_activity_score(&monkeys));

    let mut monkeys = load_monkeys();
    let mut d = 1;
    for m in monkeys.iter() {
        d *= m.condition_arg;
    }

    for _i in 0..10000 {
        for j in 0..monkeys.len() {
            while let Some((dst, item)) = monkeys.get_mut(j).unwrap().pass_item(1) {
                let item = item % d;
                monkeys.get_mut(dst as usize).unwrap().recv_item(item);
            }
        }
    }

    println!("part2: {:?}", get_activity_score(&monkeys));
}
