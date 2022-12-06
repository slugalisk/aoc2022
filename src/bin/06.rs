use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn find_sop(vals: &String, n: usize) -> Option<usize> {
    let mut seq: VecDeque<char> = VecDeque::new();
    let mut chars: HashMap<char, i32> = HashMap::new();
    for (i, c) in vals.chars().enumerate() {
        seq.push_back(c);
        if let Some(v) = chars.get_mut(&c) {
            *v += 1
        } else {
            chars.insert(c, 1);
        }

        if i >= n {
            let t = seq.pop_front().unwrap();
            let v = chars.get_mut(&t).unwrap();
            if *v > 1 {
                *v -= 1;
            } else {
                chars.remove(&t);
            }
            if chars.len() == n {
                return Some(i + 1);
            }
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("./inputs/06.txt").unwrap();

    println!("{:?}", find_sop(&input, 4).unwrap());
    println!("{:?}", find_sop(&input, 14).unwrap());
}
