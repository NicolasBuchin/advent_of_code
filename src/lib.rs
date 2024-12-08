use std::collections::{HashMap, HashSet};

pub fn resonant_collinearity(input: &str) -> usize {
    let mut map: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let (mut x, mut y) = (0, 0);
    let mut width = 0;

    for b in input.bytes() {
        if b == b'\r' {
            continue;
        } else if b == b'.' {
            x += 1;
        } else if b == b'\n' {
            y += 1;
            width = x;
            x = 0;
        } else {
            if let Some(positions) = map.get(&b) {
                positions.iter().for_each(|(i, j)| {
                    let (dx, dy) = (i - x, j - y);
                    antinodes.insert((x - dx, y - dy));
                    antinodes.insert((i + dx, j + dy));
                });
            }
            map.entry(b).or_default().push((x, y));
            x += 1;
        }
    }

    antinodes
        .iter()
        .map(|(i, j)| {
            if *i >= 0 && *j >= 0 && *i < width && *j < y {
                1
            } else {
                0
            }
        })
        .sum()
}
