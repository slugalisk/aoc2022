use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Shape {
    Rock = 1,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
enum Call {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Shape {
    fn call_match(self, other: Shape) -> Call {
        if self as i32 - 1 == other as i32 || self == Shape::Rock && other == Shape::Scissors {
            Call::Win
        } else if self == other {
            Call::Draw
        } else {
            Call::Loss
        }
    }

    fn score_match(self, other: Shape) -> i32 {
        self as i32 + self.call_match(other) as i32
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/02.txt").unwrap();
    let strategy: Vec<(char, char)> = input
        .split('\n')
        .filter_map(|c| {
            let t: Vec<char> = c.split(' ').filter_map(|t| t.chars().next()).collect();
            if t.len() == 2 {
                Some((t[0], t[1]))
            } else {
                None
            }
        })
        .collect();

    let op_key: HashMap<char, Shape> = HashMap::from([
        ('A', Shape::Rock),
        ('B', Shape::Paper),
        ('C', Shape::Scissors),
    ]);

    let mut part1_score = 0;

    let code = vec!['X', 'Y', 'Z'];
    for c in code.iter().permutations(code.len()).unique() {
        let key: HashMap<char, Shape> = HashMap::from([
            (*(c[0]), Shape::Rock),
            (*(c[1]), Shape::Paper),
            (*(c[2]), Shape::Scissors),
        ]);
        let mut score = 0;

        for round in strategy.iter() {
            let op_shape = op_key.get(&(round.0)).unwrap();
            let my_shape = key.get(&(round.1)).unwrap();
            score += my_shape.score_match(*op_shape);
        }

        if score > part1_score {
            part1_score = score;
        }
    }

    println!("part1: {:?}", part1_score);

    let call_key: HashMap<char, Call> =
        HashMap::from([('X', Call::Loss), ('Y', Call::Draw), ('Z', Call::Win)]);
    let shapes = vec![Shape::Rock, Shape::Paper, Shape::Scissors];

    let mut part2_score = 0;

    for round in strategy.iter() {
        let op_shape = op_key.get(&(round.0)).unwrap();
        let call = call_key.get(&(round.1)).unwrap();
        for my_shape in shapes.iter() {
            if my_shape.call_match(*op_shape) == *call {
                part2_score += my_shape.score_match(*op_shape);
            }
        }
    }

    println!("part2: {:?}", part2_score);
}
