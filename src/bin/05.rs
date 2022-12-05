use std::fs;

fn get_tops(state: Vec<Vec<char>>) -> String {
    let mut tops = vec![];
    for s in state.iter() {
        if !s.is_empty() {
            tops.push(s[s.len() - 1]);
        }
    }
    tops.iter().collect::<String>()
}

fn main() {
    let input = fs::read_to_string("./inputs/05.txt").unwrap();
    let segments = input.split("\n\n").collect::<Vec<_>>();

    let mut init = segments[0].split('\n').collect::<Vec<_>>();

    let bins = init.pop().unwrap().len() / 4 + 1;
    let mut state: Vec<Vec<char>> = Vec::with_capacity(bins);
    for _i in 0..bins {
        state.push(vec![]);
    }
    for r in init.iter() {
        let bins = r.chars().collect::<Vec<_>>();
        let mut pos = 1;
        for i in 0..9 {
            if bins[pos] != ' ' {
                state[i].push(bins[pos]);
            }
            pos += 4
        }
    }
    for i in 0..bins {
        state[i].reverse();
    }

    let mut part1_state = state.clone();

    let moves = segments[1]
        .split('\n')
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>();

    for m in moves.iter() {
        let p = m.split(' ').collect::<Vec<_>>();
        let n = p[1].parse::<usize>().unwrap();
        let src = p[3].parse::<usize>().unwrap() - 1;
        let dst = p[5].parse::<usize>().unwrap() - 1;

        for _i in 0..n {
            let v = part1_state[src].pop().unwrap();
            part1_state[dst].push(v);
        }
    }

    println!("{:?}", get_tops(part1_state));

    let mut part2_state = state.clone();

    for m in moves.iter() {
        let p = m.split(' ').collect::<Vec<_>>();
        let src = p[3].parse::<usize>().unwrap() - 1;
        let dst = p[5].parse::<usize>().unwrap() - 1;
        let i = part2_state[src].len() - p[1].parse::<usize>().unwrap();

        for v in part2_state[src].split_off(i) {
            part2_state[dst].push(v);
        }
    }

    println!("{:?}", get_tops(part2_state));
}
