use std::fs;

#[derive(Debug)]
struct Sensor {
    pub location: (i64, i64),
    pub beacon: (i64, i64),
}

impl Sensor {
    fn new(location: (i64, i64), beacon: (i64, i64)) -> Sensor {
        Sensor { location, beacon }
    }

    fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn scan_distance(&self) -> i64 {
        Sensor::distance(self.location, self.beacon)
    }

    fn covers(&self, p: (i64, i64)) -> bool {
        Sensor::distance(self.location, p) <= self.scan_distance()
    }

    fn has_beacon(&self, p: (i64, i64)) -> bool {
        p == self.beacon
    }

    fn scan_range_at_y(&self, y: i64) -> (i64, i64) {
        let w = self.scan_distance() - (self.location.1 - y).abs();
        (self.location.0 - w, self.location.0 + w)
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/15.txt").unwrap();
    let p = input
        .trim()
        .split('\n')
        .map(|t| {
            let p = t
                .split(|c| ['=', ':', ',', ' '].contains(&c))
                .collect::<Vec<_>>();
            Sensor::new(
                (p[3].parse().unwrap(), p[6].parse().unwrap()),
                (p[13].parse().unwrap(), p[16].parse().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let mut min = p[0].location;
    let mut max = p[0].location;
    for s in p.iter() {
        let d = s.scan_distance();
        min.0 = min.0.min(s.location.0 - d);
        min.1 = min.1.min(s.location.1 - d);
        max.0 = max.0.max(s.location.0 + d);
        max.1 = max.1.max(s.location.1 + d);
    }

    let mut part1_covered = 0;
    let part1_row = 2000000;
    for x in min.0..=max.0 {
        for s in p.iter() {
            if s.covers((x, part1_row)) {
                part1_covered += 1;
                break;
            }
        }
        for s in p.iter() {
            if s.has_beacon((x, part1_row)) {
                part1_covered -= 1;
                break;
            }
        }
    }

    println!("part1: {:?}", part1_covered);

    let mut x = 0;
    let mut y = 0;
    let mut part2_gap = (0, 0);
    let part2_bound = 4000000;
    loop {
        if let Some(s) = p.iter().find(|t| (*t).covers((x, y))) {
            let r = s.scan_range_at_y(y);
            x = r.1 + 1;
        } else {
            part2_gap = (x, y);
            break;
        }
        if x >= part2_bound {
            x = 0;
            y += 1;
        }
        if y >= part2_bound {
            println!("not found");
            break;
        }
    }

    println!("part2: {:?}", part2_gap.0 * 4000000 + part2_gap.1);
}
