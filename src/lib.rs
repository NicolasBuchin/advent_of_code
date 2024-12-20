use std::collections::{HashMap, HashSet};

const SCORE_AIM: usize = 100;

pub fn race_condition(input: &str) -> usize {
    let (road, walls) = parse(input);

    find_shortcuts(road, walls)
}

fn find_shortcuts(road: HashMap<(usize, usize), usize>, walls: HashSet<(usize, usize)>) -> usize {
    let mut shortcuts = 0;

    for wall in walls {
        let (x, y) = wall;
        let mut scores = Vec::new();
        if x > 0 {
            if let Some(&score) = road.get(&(x - 1, y)) {
                scores.push(score);
            }
        }
        if y > 0 {
            if let Some(&score) = road.get(&(x, y - 1)) {
                for score_bis in &scores {
                    if score_bis.abs_diff(score) > SCORE_AIM {
                        shortcuts += 1;
                    }
                }
                scores.push(score);
            }
        }
        if let Some(&score) = road.get(&(x + 1, y)) {
            for score_bis in &scores {
                if score_bis.abs_diff(score) > SCORE_AIM {
                    shortcuts += 1;
                }
            }
            scores.push(score);
        }
        if let Some(&score) = road.get(&(x, y + 1)) {
            for score_bis in &scores {
                if score_bis.abs_diff(score) > SCORE_AIM {
                    shortcuts += 1;
                }
            }
        }
    }

    shortcuts
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> (HashMap<(usize, usize), usize>, HashSet<(usize, usize)>) {
    let bytes = input.as_bytes();
    let mut path = HashSet::new();
    let mut walls = HashSet::new();
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
                if x != 0 && y != 0 {
                    walls.insert((x, y));
                }
                x += 1;
            }
            b'S' => {
                start = (x, y);
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

    let mut road = HashMap::new();
    let mut current = start;
    let mut score = 0;

    while current != end {
        let (x, y) = current;
        score += 1;
        if x > 0 && path.contains(&(x - 1, y)) {
            road.insert(current, score);
            path.remove(&current);
            current = (x - 1, y);
            continue;
        }
        if y > 0 && path.contains(&(x, y - 1)) {
            road.insert(current, score);
            path.remove(&current);
            current = (x, y - 1);
            continue;
        }
        if path.contains(&(x + 1, y)) {
            road.insert(current, score);
            path.remove(&current);
            current = (x + 1, y);
            continue;
        }
        if path.contains(&(x, y + 1)) {
            road.insert(current, score);
            path.remove(&current);
            current = (x, y + 1);
            continue;
        }
    }
    road.insert(current, score + 1);

    (road, walls)
}
