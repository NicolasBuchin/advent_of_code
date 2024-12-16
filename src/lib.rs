use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> usize {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        cost: 0,
        position: start,
        direction: (1, 0),
    });

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            return cost;
        }

        let visit_key = (position, direction);
        if !visited.insert(visit_key) {
            continue;
        }

        for &next_dir in directions.iter() {
            let (x, y) = position;
            let new_x = x as i32 + next_dir.0;
            let new_y = y as i32 + next_dir.1;

            if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
                continue;
            }

            let new_pos = (new_x as usize, new_y as usize);

            if walls.contains(&new_pos) {
                continue;
            }

            let movement_cost = 1;
            let direction_change_cost = if direction != next_dir { 1000 } else { 0 };
            let new_cost = cost + movement_cost + direction_change_cost;

            let new_state = State {
                cost: new_cost,
                position: new_pos,
                direction: next_dir,
            };

            if !visited.contains(&(new_pos, next_dir)) {
                heap.push(new_state);
            }
        }
    }

    usize::MAX
}
