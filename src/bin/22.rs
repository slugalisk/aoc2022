use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right = 0,
    Down,
    Left,
    Up,
}

impl Direction {
    fn move_pos(&self, map: &[Vec<Tile>], pos: (usize, usize)) -> (usize, usize) {
        let h = map.len();
        let w = map[pos.1].len();
        match self {
            Direction::Left => ((pos.0 + w - 1) % w, pos.1),
            Direction::Right => ((pos.0 + w + 1) % w, pos.1),
            Direction::Up => (pos.0, (pos.1 + h - 1) % h),
            Direction::Down => (pos.0, (pos.1 + h + 1) % h),
        }
    }

    fn check_overflow(&self, map: &[Vec<Tile>], pos: (usize, usize)) -> bool {
        let h = map.len();
        let w = map[pos.1].len();
        match self {
            Direction::Left => pos.0 == 0 || map[pos.1][pos.0 - 1] == Tile::Border,
            Direction::Right => pos.0 == w - 1,
            Direction::Up => pos.1 == 0 || map[pos.1 - 1][pos.0] == Tile::Border,
            Direction::Down => {
                pos.1 == h - 1
                    || pos.0 >= map[pos.1 + 1].len()
                    || map[pos.1 + 1][pos.0] == Tile::Border
            }
        }
    }

    fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            match self {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            }
        } else {
            match self {
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
            }
        }
    }
}

#[derive(Debug)]
enum Face<const N: usize> {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl<const N: usize> Face<N> {
    fn move_pos(pos: (usize, usize), dir: Direction) -> ((usize, usize), Direction) {
        let ((x, y), face) = Face::<N>::face_pos(pos);
        match dir {
            Direction::Right => match face {
                Face::B => (Face::<N>::E.map_pos((N - 1, N - y - 1)), Direction::Left),
                Face::C => (Face::<N>::B.map_pos((y, N - 1)), Direction::Up),
                Face::E => (Face::<N>::B.map_pos((N - 1, N - y - 1)), Direction::Left),
                Face::F => (Face::<N>::E.map_pos((y, N - 1)), Direction::Up),
                _ => unreachable!(),
            },
            Direction::Down => match face {
                Face::B => (Face::<N>::C.map_pos((N - 1, x)), Direction::Left),
                Face::E => (Face::<N>::F.map_pos((N - 1, x)), Direction::Left),
                Face::F => (Face::<N>::B.map_pos((x, 0)), Direction::Down),
                _ => unreachable!(),
            },
            Direction::Left => match face {
                Face::A => (Face::<N>::D.map_pos((0, N - y - 1)), Direction::Right),
                Face::C => (Face::<N>::D.map_pos((y, 0)), Direction::Down),
                Face::D => (Face::<N>::A.map_pos((0, N - y - 1)), Direction::Right),
                Face::F => (Face::<N>::A.map_pos((y, 0)), Direction::Down),
                _ => unreachable!(),
            },
            Direction::Up => match face {
                Face::A => (Face::<N>::F.map_pos((0, x)), Direction::Right),
                Face::B => (Face::<N>::F.map_pos((x, N - 1)), Direction::Up),
                Face::D => (Face::<N>::C.map_pos((0, x)), Direction::Right),
                _ => unreachable!(),
            },
        }
    }

    fn face_pos(pos: (usize, usize)) -> ((usize, usize), Face<N>) {
        let f = match (pos.0 / N, pos.1 / N) {
            (1, 0) => Face::A,
            (2, 0) => Face::B,
            (1, 1) => Face::C,
            (0, 2) => Face::D,
            (1, 2) => Face::E,
            (0, 3) => Face::F,
            _ => unreachable!(),
        };
        ((pos.0 % N, pos.1 % N), f)
    }

    fn map_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Face::A => (pos.0 + N, pos.1),
            Face::B => (pos.0 + 2 * N, pos.1),
            Face::C => (pos.0 + N, pos.1 + N),
            Face::D => (pos.0, pos.1 + 2 * N),
            Face::E => (pos.0 + N, pos.1 + 2 * N),
            Face::F => (pos.0, pos.1 + 3 * N),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Border,
    Open,
    Wall,
}

type Wrap = fn(&[Vec<Tile>], (usize, usize), Direction) -> ((usize, usize), Direction);

fn move_pos(
    map: &[Vec<Tile>],
    pos: (usize, usize),
    dir: Direction,
    n: usize,
    wrap: Wrap,
) -> ((usize, usize), Direction) {
    let mut pos = pos;
    let mut dir = dir;
    for _ in 0..n {
        let (next, next_dir) = if dir.check_overflow(map, pos) {
            wrap(map, pos, dir)
        } else {
            (dir.move_pos(map, pos), dir)
        };
        if map[next.1][next.0] == Tile::Wall {
            return (pos, dir);
        }
        pos = next;
        dir = next_dir;
    }
    (pos, dir)
}

fn run(map: &[Vec<Tile>], ns: &[usize], rs: &[bool], wrap: Wrap) -> usize {
    let mut pos = (
        map[0].iter().position(|t| *t == Tile::Open).unwrap(),
        0_usize,
    );
    let mut dir = Direction::Right;

    for (i, n) in ns.iter().enumerate() {
        (pos, dir) = move_pos(&map, pos, dir, *n, wrap);
        if i < rs.len() {
            dir = dir.rotate(rs[i]);
        }
    }

    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir as usize
}

fn main() {
    let input = fs::read_to_string("./inputs/22.txt").unwrap();
    let (map, path) = input.trim_end().split_once("\n\n").unwrap();
    let map = map
        .split('\n')
        .map(|t| {
            t.chars()
                .map(|c| match c {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    ' ' => Tile::Border,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let ns = path
        .split(|t: char| t.is_alphabetic())
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let rs = path
        .chars()
        .filter(|t| t.is_alphabetic())
        .map(|t| t == 'R')
        .collect::<Vec<_>>();

    let wrap =
        |map: &[Vec<Tile>], pos: (usize, usize), dir: Direction| -> ((usize, usize), Direction) {
            let (mut x, mut y) = dir.move_pos(map, pos);
            while x >= map[y].len() || map[y][x] == Tile::Border {
                (x, y) = dir.move_pos(map, (x, y));
            }
            ((x, y), dir)
        };
    println!("part1: {:?}", run(&map, &ns, &rs, wrap));

    let wrap = |_: &[Vec<Tile>],
                pos: (usize, usize),
                dir: Direction|
     -> ((usize, usize), Direction) { Face::<50>::move_pos(pos, dir) };
    println!("part2: {:?}", run(&map, &ns, &rs, wrap));
}
