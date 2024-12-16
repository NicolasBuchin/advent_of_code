use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

pub fn reindeer_maze(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut walls = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    let (mut x, mut y) = (0, 0);
    let mut i = 0;

    loop {
        let b = bytes[i];
        match b {
            b'\n' => {
                y += 1;
                if i == bytes.len() - 1 {
                    break;
                }
                x = 0;
            }
            b'#' => {
                walls.insert((x, y));
                x += 1;
            }
            b'S' => {
                start = (x, y);
                x += 1;
            }
            b'E' => {
                end = (x, y);
                x += 1;
            }
            b'.' => {
                x += 1;
            }
            _ => (),
        }
        i += 1;
    }

    find_path(start, end, &walls, x, y)
}

#[derive(Debug, Eq, PartialEq)]
struct Tile {
    position: (usize, usize),
    g_score: usize,
    f_score: usize,
    direction: (isize, isize), // Direction of movement to this Tile
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn get_neighbors(
    position: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if position.0 > 0 && !walls.contains(&(position.0 - 1, position.1)) {
        neighbors.push((position.0 - 1, position.1));
    }
    if position.0 < width - 1 && !walls.contains(&(position.0 + 1, position.1)) {
        neighbors.push((position.0 + 1, position.1));
    }
    if position.1 > 0 && !walls.contains(&(position.0, position.1 - 1)) {
        neighbors.push((position.0, position.1 - 1));
    }
    if position.1 < height - 1 && !walls.contains(&(position.0, position.1 + 1)) {
        neighbors.push((position.0, position.1 + 1));
    }

    neighbors
}

pub fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> usize {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut g_scores: HashMap<(usize, usize), usize> = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores: HashMap<(usize, usize), usize> = HashMap::new();
    f_scores.insert(start, heuristic(start, end));

    open_set.push(Tile {
        position: start,
        g_score: 0,
        f_score: heuristic(start, end),
        direction: (1, 0),
    });

    while let Some(current_tile) = open_set.pop() {
        let current = current_tile.position;
        let current_direction = current_tile.direction;

        if current == end {
            return *g_scores.get(&current).unwrap();
        }

        for neighbor in get_neighbors(current, walls, width, height) {
            let direction = (
                neighbor.0 as isize - current.0 as isize,
                neighbor.1 as isize - current.1 as isize,
            );

            let turn_penalty = {
                if direction != current_direction {
                    1000
                } else {
                    0
                }
            };

            let g_score = g_scores.get(&current).unwrap() + 1 + turn_penalty;

            if let Entry::Vacant(entry) = g_scores.entry(neighbor) {
                entry.insert(g_score);
                came_from.insert(neighbor, current);
                let f_score = g_score + heuristic(neighbor, end);
                f_scores.insert(neighbor, f_score);

                open_set.push(Tile {
                    position: neighbor,
                    g_score,
                    f_score,
                    direction,
                });
            }
        }
    }
    0
}
