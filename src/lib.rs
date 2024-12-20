use std::collections::HashSet;

const SCORE_AIM: usize = 100;
const MAX_DISTANCE: usize = 20;

pub fn race_condition(input: &str) -> usize {
    let road = parse(input);

    find_shortcuts(road)
}

fn find_shortcuts(road: Vec<((usize, usize), usize)>) -> usize {
    let mut shortcuts = 0;

    for i in 0..road.len() {
        let (position, score) = road[i];
        for other_tile in &road[i + 1..] {
            let (other_position, other_score) = *other_tile;
            let distance = distance(position, other_position);
            if distance <= MAX_DISTANCE && score.abs_diff(other_score) - distance >= SCORE_AIM {
                shortcuts += 1;
            }
        }
    }

    shortcuts
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn parse(input: &str) -> Vec<((usize, usize), usize)> {
    let bytes = input.as_bytes();
    let mut path = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut i = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        match bytes[i] {
            b'\n' => {
                y += 1;
                if i == bytes.len() - 1 {
                    break;
                }
                x = 0;
            }
            b'.' => {
                path.insert((x, y));
                x += 1;
            }
            b'#' => {
                x += 1;
            }
            b'S' => {
                start = (x, y);
                path.insert((x, y));
                x += 1;
            }
            b'E' => {
                end = (x, y);
                path.insert((x, y));
                x += 1;
            }
            _ => (),
        }
        i += 1;
    }

    let mut road = Vec::new();
    let mut current = start;
    let mut score = 0;

    while current != end {
        let (x, y) = current;
        score += 1;
        if x > 0 && path.contains(&(x - 1, y)) {
            road.push((current, score));
            path.remove(&current);
            current = (x - 1, y);
            continue;
        }
        if y > 0 && path.contains(&(x, y - 1)) {
            road.push((current, score));
            path.remove(&current);
            current = (x, y - 1);
            continue;
        }
        if path.contains(&(x + 1, y)) {
            road.push((current, score));
            path.remove(&current);
            current = (x + 1, y);
            continue;
        }
        if path.contains(&(x, y + 1)) {
            road.push((current, score));
            path.remove(&current);
            current = (x, y + 1);
            continue;
        }
    }
    road.push((current, score + 1));

    road
}
