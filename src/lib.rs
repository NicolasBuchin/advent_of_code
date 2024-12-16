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

    find_path_unique_tiles(start, end, &walls, x, y)
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

#[allow(clippy::type_complexity, clippy::comparison_chain)]
pub fn find_path_unique_tiles(
    start: (usize, usize),
    end: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> usize {
    let mut heap = BinaryHeap::new();
    let mut predecessors: HashMap<((usize, usize), (i32, i32)), HashSet<((usize, usize), (i32, i32))>> = HashMap::new();
    let mut costs = HashMap::new();

    let initial_state = State {
        cost: 0,
        position: start,
        direction: (1, 0),
    };
    heap.push(initial_state);
    costs.insert((start, (1, 0)), 0);

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut min_cost_to_end = usize::MAX;
    let mut end_states = HashSet::new();

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if cost > min_cost_to_end {
            break;
        }

        if position == end {
            if cost < min_cost_to_end {
                min_cost_to_end = cost;
                end_states.clear();
            }
            if cost == min_cost_to_end {
                end_states.insert((position, direction));
            }
            continue;
        }

        let current_best = costs.get(&(position, direction)).copied().unwrap_or(usize::MAX);
        if cost > current_best {
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

            let direction_change_cost = if direction != next_dir { 1000 } else { 0 };
            let new_cost = cost + direction_change_cost + 1;

            let next_state = (new_pos, next_dir);
            let current_state = (position, direction);

            match costs.get(&next_state) {
                Some(&existing_cost) => {
                    if new_cost < existing_cost {
                        costs.insert(next_state, new_cost);
                        let pred_set = predecessors.entry(next_state).or_default();
                        pred_set.clear();
                        pred_set.insert(current_state);
                        heap.push(State {
                            cost: new_cost,
                            position: new_pos,
                            direction: next_dir,
                        });
                    } else if new_cost == existing_cost {
                        predecessors.entry(next_state).or_default().insert(current_state);
                    }
                }
                None => {
                    costs.insert(next_state, new_cost);
                    predecessors.entry(next_state).or_default().insert(current_state);
                    heap.push(State {
                        cost: new_cost,
                        position: new_pos,
                        direction: next_dir,
                    });
                }
            }
        }
    }

    let mut unique_tiles = HashSet::new();
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    for &end_state in &end_states {
        stack.push(end_state);
    }

    while let Some(current_state) = stack.pop() {
        unique_tiles.insert(current_state.0);
        if current_state.0 == start {
            continue;
        }
        if let Some(prev_states) = predecessors.get(&current_state) {
            for &prev_state in prev_states {
                if visited.insert((prev_state, current_state)) {
                    stack.push(prev_state);
                }
            }
        }
    }

    unique_tiles.len()
}
