use std::fs;

fn fill_space<T>(space: &mut [Vec<u8>], done: T) -> i32
where
    T: Fn((usize, usize)) -> bool,
{
    let mut n = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            if space[y + 1][x] == 0 {
                y += 1;
            } else if space[y + 1][x - 1] == 0 {
                x -= 1;
                y += 1;
            } else if space[y + 1][x + 1] == 0 {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }
        if done((x, y)) {
            return n;
        }
        *(space.get_mut(y).unwrap().get_mut(x).unwrap()) = 1;
        n += 1;
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/14.txt").unwrap();
    let barriers: Vec<Vec<(usize, usize)>> = input
        .trim()
        .split('\n')
        .map(|t| {
            t.split(" -> ")
                .map(|t| {
                    t.split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let mut space = vec![vec![0; 1000]; 1000];
    let mut floor = 0;
    for b in barriers {
        for i in 1..b.len() {
            let (x0, y0) = b[i - 1];
            let (x1, y1) = b[i];
            for x in x0.min(x1)..=x0.max(x1) {
                for y in y0.min(y1)..=y0.max(y1) {
                    *(space.get_mut(y).unwrap().get_mut(x).unwrap()) = 2;
                }
            }
            floor = floor.max(y0.max(y1));
        }
    }
    floor += 2;
    for x in 0..1000 {
        *(space.get_mut(floor).unwrap().get_mut(x).unwrap()) = 2;
    }

    println!(
        "part1: {:?}",
        fill_space(&mut space.clone(), |(_, y)| y == floor - 1)
    );

    println!(
        "part2: {:?}",
        fill_space(&mut space.clone(), |p| p == (500, 0)) + 1
    );
}
