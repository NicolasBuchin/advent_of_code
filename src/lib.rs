const NUMERIC_KEY_POSITIONS: [(usize, usize); 11] = [
    (1, 3), // 0
    (0, 2), // 1
    (1, 2), // 2
    (2, 2), // 3
    (0, 1), // 4
    (1, 1), // 5
    (2, 1), // 6
    (0, 0), // 7
    (1, 0), // 8
    (2, 0), // 9
    (2, 3), // A
];
const DIRECTION_KEY_POSITIONS: [(usize, usize); 5] = [
    (2, 0), // A
    (1, 0), // ^
    (0, 1), // <
    (1, 1), // v
    (2, 1), // >
];
const DEPTH: usize = 25;

#[derive(Debug, Clone, Copy)]
enum Key {
    A = 0,
    N = 1, // ^
    W = 2, // <
    S = 3, // v
    E = 4, // >
}

pub fn keypad_conundrum(input: &str) -> usize {
    let cost_table = cost_table();
    input
        .lines()
        .map(|line| {
            let number: usize = line[0..3].parse().unwrap();
            let mut position = 10;
            line.bytes()
                .map(|b| {
                    let mut paths = Vec::new();
                    let next_position = match b {
                        b'A' => 10,
                        _ => b as usize - 0x30,
                    };
                    get_path(
                        NUMERIC_KEY_POSITIONS[position],
                        NUMERIC_KEY_POSITIONS[next_position],
                        (0, 3),
                        &mut paths,
                    );
                    let cost: usize = paths
                        .iter()
                        .map(|path| {
                            let mut position = 0;
                            path.iter()
                                .map(|&p| {
                                    let next = p as usize;
                                    let cost = cost_table[position * 5 + next];
                                    position = next;
                                    cost
                                })
                                .sum()
                        })
                        .min()
                        .unwrap();
                    position = next_position;
                    cost
                })
                .sum::<usize>()
                * number
        })
        .sum()
}

fn cost_table() -> Vec<usize> {
    let all_paths: Vec<_> = (0..DIRECTION_KEY_POSITIONS.len().pow(2))
        .map(|i| {
            let mut paths = Vec::new();
            let start = i.div_euclid(DIRECTION_KEY_POSITIONS.len());
            let end = i.rem_euclid(DIRECTION_KEY_POSITIONS.len());
            get_path(
                DIRECTION_KEY_POSITIONS[start],
                DIRECTION_KEY_POSITIONS[end],
                (0, 0),
                &mut paths,
            );
            paths
        })
        .collect();

    let mut min_path_costs: Vec<usize> = all_paths
        .iter()
        .map(|paths| paths.iter().map(|path| path.len()).min().unwrap())
        .collect();

    for _ in 0..DEPTH - 1 {
        min_path_costs = new_path_cost(&min_path_costs, &all_paths);
    }

    min_path_costs
}

fn new_path_cost(previous_costs: &[usize], all_paths: &[Vec<Vec<Key>>]) -> Vec<usize> {
    all_paths
        .iter()
        .map(|paths| {
            paths
                .iter()
                .map(|path| {
                    let mut position = 0;
                    path.iter()
                        .map(|&p| {
                            let next = p as usize;
                            let cost = previous_costs[position * 5 + next];
                            position = next;
                            cost
                        })
                        .sum()
                })
                .min()
                .unwrap()
        })
        .collect()
}

#[allow(clippy::comparison_chain)]
fn get_path(start: (usize, usize), end: (usize, usize), forbidden: (usize, usize), paths: &mut Vec<Vec<Key>>) {
    if start.0 != forbidden.0 || end.1 != forbidden.1 {
        let mut path = Vec::new();
        if start.1 < end.1 {
            path.extend((start.1..end.1).map(|_| Key::S));
        } else if start.1 > end.1 {
            path.extend((end.1..start.1).map(|_| Key::N));
        }
        if start.0 < end.0 {
            path.extend((start.0..end.0).map(|_| Key::E));
        } else if start.0 > end.0 {
            path.extend((end.0..start.0).map(|_| Key::W));
        }
        path.push(Key::A);
        paths.push(path);
    }

    if start.0 != end.0 && start.1 != end.1 && !(start.1 == forbidden.1 && end.0 == forbidden.0) {
        let mut path = Vec::new();
        if start.0 < end.0 {
            path.extend((start.0..end.0).map(|_| Key::E));
        } else if start.0 > end.0 {
            path.extend((end.0..start.0).map(|_| Key::W));
        }
        if start.1 < end.1 {
            path.extend((start.1..end.1).map(|_| Key::S));
        } else if start.1 > end.1 {
            path.extend((end.1..start.1).map(|_| Key::N));
        }
        path.push(Key::A);
        paths.push(path);
    }
}
