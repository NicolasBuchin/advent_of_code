use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
};

const WIDTH: usize = 70 + 1;
const HEIGHT: usize = 70 + 1;
const NUMBER: usize = 1024;

pub fn ram_run(input: &str) -> usize {
    let falling = parse(input);

    let path = find_path((0, 0), (WIDTH - 1, HEIGHT - 1), &falling);

    show(&falling, &path);

    path.len() - 1
}

fn show(falling: &HashSet<(usize, usize)>, path: &HashSet<(usize, usize)>) {
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            if falling.contains(&(i, j)) {
                print!("#")
            } else if path.contains(&(i, j)) {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn parse(input: &str) -> HashSet<(usize, usize)> {
    let bytes = input.as_bytes();

    let mut falling = HashSet::new();
    let mut i = 0;

    while i < bytes.len() && falling.len() < NUMBER {
        let mut x = 0;
        while bytes[i] != b',' {
            x *= 10;
            x += bytes[i] as usize - 0x30;
            i += 1;
        }
        i += 1;
        let mut y = 0;
        while bytes[i] != b'\n' {
            y *= 10;
            y += bytes[i] as usize - 0x30;
            i += 1;
        }
        i += 1;
        falling.insert((x, y));
    }

    falling
}

#[derive(Debug, Eq, PartialEq)]
struct Tile {
    position: (usize, usize),
    g_score: usize,
    f_score: usize,
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

fn get_neighbors(position: (usize, usize), walls: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if position.0 > 0 && !walls.contains(&(position.0 - 1, position.1)) {
        neighbors.push((position.0 - 1, position.1));
    }
    if position.0 < WIDTH - 1 && !walls.contains(&(position.0 + 1, position.1)) {
        neighbors.push((position.0 + 1, position.1));
    }
    if position.1 > 0 && !walls.contains(&(position.0, position.1 - 1)) {
        neighbors.push((position.0, position.1 - 1));
    }
    if position.1 < HEIGHT - 1 && !walls.contains(&(position.0, position.1 + 1)) {
        neighbors.push((position.0, position.1 + 1));
    }

    neighbors
}

pub fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    walls: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut g_scores: HashMap<(usize, usize), usize> = HashMap::new();
    g_scores.insert(start, 0);

    open_set.push(Tile {
        position: start,
        g_score: 0,
        f_score: heuristic(start, end),
    });

    while let Some(current_tile) = open_set.pop() {
        let current = current_tile.position;

        if current == end {
            let mut steps = HashSet::new();
            let mut current = current;
            while let Some(&previous_step) = came_from.get(&current) {
                steps.insert(current);
                current = previous_step;
            }
            steps.insert(start);
            return steps;
        }

        for neighbor in get_neighbors(current, walls) {
            let g_score = g_scores.get(&current).unwrap() + 1;

            if let Entry::Vacant(entry) = g_scores.entry(neighbor) {
                entry.insert(g_score);
                came_from.insert(neighbor, current);
                let f_score = g_score + heuristic(neighbor, end);

                open_set.push(Tile {
                    position: neighbor,
                    g_score,
                    f_score,
                });
            }
        }
    }

    panic!("Cant find path!")
}
