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

        i += 4;

        let mut prize_y = 0;
        while bytes[i] != b'\n' {
            prize_y *= 10;
            prize_y += bytes[i] as usize - 0x30;
            i += 1;
        }

        i += 14;

        score += solve(ax, ay, bx, by, prize_x, prize_y);
    }

    score
}

fn solve(ax: usize, ay: usize, bx: usize, by: usize, prize_x: usize, prize_y: usize) -> usize {
    let mut best_score = 0usize;
    for ac in 0..=100 {
        for bc in 0..100 {
            let score = ac * 3 + bc;
            if ac * ax + bc * bx == prize_x && ac * ay + bc * by == prize_y && (score < best_score || best_score == 0) {
                best_score = score;
            }
        }
    }
    best_score
}
