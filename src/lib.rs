use std::ops::Div;

pub fn claw_contraption(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut score = 0;
    let mut i = 12;

    while i < bytes.len() {
        let ax = (bytes[i] as usize - 0x30) * 10 + (bytes[i + 1] as usize - 0x30);
        i += 6;
        let ay = (bytes[i] as usize - 0x30) * 10 + (bytes[i + 1] as usize - 0x30);
        i += 15;
        let bx = (bytes[i] as usize - 0x30) * 10 + (bytes[i + 1] as usize - 0x30);
        i += 6;
        let by = (bytes[i] as usize - 0x30) * 10 + (bytes[i + 1] as usize - 0x30);
        i += 12;

        let mut prize_x = 0;
        while bytes[i] != b',' {
            prize_x *= 10;
            prize_x += bytes[i] as usize - 0x30;
            i += 1;
        }
        prize_x += 10000000000000;

        i += 4;

        let mut prize_y = 0;
        while bytes[i] != b'\n' {
            prize_y *= 10;
            prize_y += bytes[i] as usize - 0x30;
            i += 1;
        }
        prize_y += 10000000000000;

        i += 14;

        // println!(
        //     "A: x+{} y+{}\nB: x+{} y+{}\nP: x={} y={}",
        //     ax, ay, bx, by, prize_x, prize_y
        // );

        let solution = solve(
            (ax as f64, ay as f64),
            (bx as f64, by as f64),
            (prize_x as f64, prize_y as f64),
        );

        // println!("   solution {}\n", solution);

        score += solution;
    }

    score
}

fn solve(a: (f64, f64), b: (f64, f64), p: (f64, f64)) -> usize {
    let det = (a.0 * b.1) - (b.0 * a.1);
    // println!("det {} ", det);

    let i = (p.0 * b.1.div(det) - p.1 * b.0.div(det)).round() as usize;
    let j = (p.1 * a.0.div(det) - p.0 * a.1.div(det)).round() as usize;

    // println!("i: {}   j: {}", i, j);

    let x = i * a.0 as usize + j * b.0 as usize;
    let y = i * a.1 as usize + j * b.1 as usize;

    // println!("x : {}   y : {}", x, y);
    // println!("px: {}   py: {}", p.0 as usize, p.1 as usize);

    if x == p.0 as usize && y == p.1 as usize {
        return i * 3 + j;
    }

    0
}
