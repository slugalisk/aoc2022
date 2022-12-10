use std::fs;

fn update_signal_sum(s: &mut i32, c: i32, r: i32) {
    if c == 20 || c == 60 || c == 100 || c == 140 || c == 180 || c == 220 {
        *s += c * r;
    }
}

fn draw_pixel(c: i32, r: i32) {
    let p = (c - 1) % 40;
    if p >= r - 1 && p <= r + 1 {
        print!("#");
    } else {
        print!(" ");
    }
    if p == 39 {
        print!("\n");
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/10.txt").unwrap();
    let program = input.split('\n').filter_map(|t| {
        let mut p = t.split(' ');
        if let Some(op) = p.next() {
            let n = p
                .next()
                .map(|t| t.parse::<i32>().unwrap())
                .unwrap_or_default();
            Some((op, n))
        } else {
            None
        }
    });

    let mut cycle = 1;
    let mut reg = 1;
    let mut signal_sum = 0;

    for (op, n) in program {
        match op {
            "addx" => {
                for _i in 0..2 {
                    update_signal_sum(&mut signal_sum, cycle, reg);
                    draw_pixel(cycle, reg);
                    cycle += 1;
                }
                reg += n;
            }
            "noop" => {
                update_signal_sum(&mut signal_sum, cycle, reg);
                draw_pixel(cycle, reg);
                cycle += 1;
            }
            &_ => {}
        }
    }

    println!("part1: {:?}", signal_sum);
}
