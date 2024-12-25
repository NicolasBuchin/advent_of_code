const DEPTH: usize = 25;

#[derive(Debug, Clone, Copy)]
enum Key {
    A = 0, // A
    N = 1, // ^
    W = 2, // <
    S = 3, // v
    E = 4, // >
}

pub fn keypad_conundrum(input: &str) -> usize {
    let cost_table = get_cost_table();

    let mut total = 0;

    for line in input.lines() {
        let number: usize = line[0..3].parse().unwrap();
        let mut position = 0;
        let mut line_total = 0;

        for b in line.bytes() {
            let next_position = match b {
                b'A' => 0,
                _ => b as usize - 0x30 + 1,
            };

            let path = get_numerical_path(position, next_position);
            let mut prev = 0;
            let mut path_cost = 0;

            for &p in path.iter() {
                let next = p as usize;
                let costs = cost_table[prev * 5 + next];
                path_cost += costs;
                prev = next;
            }

            line_total += path_cost;
            position = next_position;
        }

        total += line_total * number;
    }

    total
}

fn get_cost_table() -> [usize; 25] {
    let all_paths: [Vec<Key>; 25] = [
        vec![Key::A],                         // A => A (0, 0)
        vec![Key::W, Key::A],                 // A => ^ (0, 1)
        vec![Key::S, Key::W, Key::W, Key::A], // A => < (0, 2)
        vec![Key::W, Key::S, Key::A],         // A => v (0, 3)
        vec![Key::S, Key::A],                 // A => > (0, 4)
        vec![Key::E, Key::A],                 // ^ => A (1, 0)
        vec![Key::A],                         // ^ => ^ (1, 1)
        vec![Key::S, Key::W, Key::A],         // ^ => < (1, 2)
        vec![Key::S],                         // ^ => v (1, 3)
        vec![Key::S, Key::E, Key::A],         // ^ => > (1, 4)
        vec![Key::E, Key::E, Key::N, Key::A], // < => A (2, 0)
        vec![Key::E, Key::N, Key::A],         // < => ^ (2, 1)
        vec![Key::A],                         // < => < (2, 2)
        vec![Key::E, Key::A],                 // < => v (2, 3)
        vec![Key::E, Key::E],                 // < => > (2, 4)
        vec![Key::N, Key::E, Key::A],         // v => A (3, 0)
        vec![Key::N],                         // v => ^ (3, 1)
        vec![Key::W, Key::A],                 // v => < (3, 2)
        vec![Key::A],                         // v => v (3, 3)
        vec![Key::E, Key::A],                 // v => > (3, 4)
        vec![Key::N, Key::A],                 // > => > (4, 0)
        vec![Key::W, Key::N, Key::A],         // > => ^ (4, 1)
        vec![Key::W, Key::W],                 // > => < (4, 2)
        vec![Key::W, Key::A],                 // > => v (4, 3)
        vec![Key::A],                         // > => > (4, 4)
    ];

    let mut min_path_costs: [usize; 25] = [
        1, // A => A (0, 0)
        2, // A => ^ (0, 1)
        4, // A => < (0, 2)
        3, // A => v (0, 3)
        2, // A => > (0, 4)
        2, // ^ => A (1, 0)
        1, // ^ => ^ (1, 1)
        3, // ^ => < (1, 2)
        1, // ^ => v (1, 3)
        3, // ^ => > (1, 4)
        4, // < => A (2, 0)
        3, // < => ^ (2, 1)
        1, // < => < (2, 2)
        2, // < => v (2, 3)
        2, // < => > (2, 4)
        3, // v => A (3, 0)
        1, // v => ^ (3, 1)
        2, // v => < (3, 2)
        1, // v => v (3, 3)
        2, // v => > (3, 4)
        2, // > => A (4, 0)
        3, // > => ^ (4, 1)
        2, // > => < (4, 2)
        2, // > => v (4, 3)
        1, // > => > (4, 4)
    ];

    for _ in 1..DEPTH {
        min_path_costs = new_path_cost(&min_path_costs, &all_paths);
    }

    min_path_costs
}

fn new_path_cost(previous_costs: &[usize], all_paths: &[Vec<Key>]) -> [usize; 25] {
    let mut new_costs = [0_usize; 25];

    for i in 0..25 {
        let path = &all_paths[i];
        let mut path_total = 0;
        let mut prev = 0;

        for &p in path {
            let next = p as usize;
            let cost = previous_costs[prev * 5 + next];
            path_total += cost;
            prev = next;
        }

        new_costs[i] = path_total;
    }

    new_costs
}

fn get_numerical_path(start: usize, end: usize) -> Vec<Key> {
    match (start, end) {
        (1, 1) => vec![Key::A],                                         // 0 => 0
        (1, 2) => vec![Key::N, Key::W, Key::A],                         // 0 => 1
        (1, 3) => vec![Key::N, Key::A],                                 // 0 => 2
        (1, 4) => vec![Key::N, Key::E, Key::A],                         // 0 => 3
        (1, 5) => vec![Key::N, Key::N, Key::W, Key::A],                 // 0 => 4
        (1, 6) => vec![Key::N, Key::N, Key::A],                         // 0 => 5
        (1, 7) => vec![Key::N, Key::N, Key::E, Key::A],                 // 0 => 6
        (1, 8) => vec![Key::N, Key::N, Key::N, Key::W, Key::A],         // 0 => 7
        (1, 9) => vec![Key::N, Key::N, Key::N, Key::A],                 // 0 => 8
        (1, 10) => vec![Key::N, Key::N, Key::N, Key::E, Key::A],        // 0 => 9
        (1, 0) => vec![Key::E, Key::A],                                 // 0 => A
        (2, 1) => vec![Key::E, Key::S, Key::A],                         // 1 => 0
        (2, 2) => vec![Key::A],                                         // 1 => 1
        (2, 3) => vec![Key::E, Key::A],                                 // 1 => 2
        (2, 4) => vec![Key::E, Key::E, Key::A],                         // 1 => 3
        (2, 5) => vec![Key::N, Key::A],                                 // 1 => 4
        (2, 6) => vec![Key::N, Key::E, Key::A],                         // 1 => 5
        (2, 7) => vec![Key::N, Key::E, Key::E, Key::A],                 // 1 => 6
        (2, 8) => vec![Key::N, Key::N, Key::A],                         // 1 => 7
        (2, 9) => vec![Key::N, Key::N, Key::E, Key::A],                 // 1 => 8
        (2, 10) => vec![Key::N, Key::N, Key::E, Key::E, Key::A],        // 1 => 9
        (2, 0) => vec![Key::E, Key::E, Key::S, Key::A],                 // 1 => A
        (3, 1) => vec![Key::S, Key::A],                                 // 2 => 0
        (3, 2) => vec![Key::W, Key::A],                                 // 2 => 1
        (3, 3) => vec![Key::A],                                         // 2 => 2
        (3, 4) => vec![Key::E, Key::A],                                 // 2 => 3
        (3, 5) => vec![Key::W, Key::N, Key::A],                         // 2 => 4
        (3, 6) => vec![Key::N, Key::A],                                 // 2 => 5
        (3, 7) => vec![Key::N, Key::E, Key::A],                         // 2 => 6
        (3, 8) => vec![Key::W, Key::N, Key::N, Key::A],                 // 2 => 7
        (3, 9) => vec![Key::N, Key::N, Key::A],                         // 2 => 8
        (3, 10) => vec![Key::N, Key::N, Key::E, Key::A],                // 2 => 9
        (3, 0) => vec![Key::S, Key::E, Key::A],                         // 2 => A
        (4, 1) => vec![Key::W, Key::S, Key::A],                         // 3 => 0
        (4, 2) => vec![Key::W, Key::W, Key::A],                         // 3 => 1
        (4, 3) => vec![Key::W, Key::A],                                 // 3 => 2
        (4, 4) => vec![Key::A],                                         // 3 => 3
        (4, 5) => vec![Key::W, Key::W, Key::N, Key::A],                 // 3 => 4
        (4, 6) => vec![Key::W, Key::N, Key::A],                         // 3 => 5
        (4, 7) => vec![Key::N, Key::A],                                 // 3 => 6
        (4, 8) => vec![Key::W, Key::W, Key::N, Key::N, Key::A],         // 3 => 7
        (4, 9) => vec![Key::W, Key::N, Key::N, Key::A],                 // 3 => 8
        (4, 10) => vec![Key::N, Key::N, Key::A],                        // 3 => 9
        (4, 0) => vec![Key::S, Key::A],                                 // 3 => A
        (5, 1) => vec![Key::E, Key::S, Key::S, Key::A],                 // 4 => 0
        (5, 2) => vec![Key::S, Key::A],                                 // 4 => 1
        (5, 3) => vec![Key::S, Key::E, Key::A],                         // 4 => 2
        (5, 4) => vec![Key::S, Key::E, Key::E, Key::A],                 // 4 => 3
        (5, 5) => vec![Key::A],                                         // 4 => 4
        (5, 6) => vec![Key::E, Key::A],                                 // 4 => 5
        (5, 7) => vec![Key::E, Key::E, Key::A],                         // 4 => 6
        (5, 8) => vec![Key::N, Key::A],                                 // 4 => 7
        (5, 9) => vec![Key::N, Key::E, Key::A],                         // 4 => 8
        (5, 10) => vec![Key::N, Key::E, Key::E, Key::A],                // 4 => 9
        (5, 0) => vec![Key::E, Key::E, Key::S, Key::S, Key::A],         // 4 => A
        (6, 1) => vec![Key::S, Key::S, Key::A],                         // 5 => 0
        (6, 2) => vec![Key::W, Key::S, Key::A],                         // 5 => 1
        (6, 3) => vec![Key::S, Key::A],                                 // 5 => 2
        (6, 4) => vec![Key::S, Key::E, Key::A],                         // 5 => 3
        (6, 5) => vec![Key::W, Key::A],                                 // 5 => 4
        (6, 6) => vec![Key::A],                                         // 5 => 5
        (6, 7) => vec![Key::E, Key::A],                                 // 5 => 6
        (6, 8) => vec![Key::W, Key::N, Key::A],                         // 5 => 7
        (6, 9) => vec![Key::N, Key::A],                                 // 5 => 8
        (6, 10) => vec![Key::N, Key::E, Key::A],                        // 5 => 9
        (6, 0) => vec![Key::S, Key::S, Key::E, Key::A],                 // 5 => A
        (7, 1) => vec![Key::W, Key::S, Key::S, Key::A],                 // 6 => 0
        (7, 2) => vec![Key::W, Key::W, Key::S, Key::A],                 // 6 => 1
        (7, 3) => vec![Key::W, Key::S, Key::A],                         // 6 => 2
        (7, 4) => vec![Key::S, Key::A],                                 // 6 => 3
        (7, 5) => vec![Key::W, Key::W, Key::A],                         // 6 => 4
        (7, 6) => vec![Key::W, Key::A],                                 // 6 => 5
        (7, 7) => vec![Key::A],                                         // 6 => 6
        (7, 8) => vec![Key::W, Key::W, Key::N, Key::A],                 // 6 => 7
        (7, 9) => vec![Key::W, Key::N, Key::A],                         // 6 => 8
        (7, 10) => vec![Key::N, Key::A],                                // 6 => 9
        (7, 0) => vec![Key::S, Key::S, Key::A],                         // 6 => A
        (8, 1) => vec![Key::E, Key::S, Key::S, Key::S, Key::A],         // 7 => 0
        (8, 2) => vec![Key::S, Key::S, Key::A],                         // 7 => 1
        (8, 3) => vec![Key::S, Key::S, Key::E, Key::A],                 // 7 => 2
        (8, 4) => vec![Key::S, Key::S, Key::E, Key::E, Key::A],         // 7 => 3
        (8, 5) => vec![Key::S, Key::A],                                 // 7 => 4
        (8, 6) => vec![Key::S, Key::E, Key::A],                         // 7 => 5
        (8, 7) => vec![Key::S, Key::E, Key::E, Key::A],                 // 7 => 6
        (8, 8) => vec![Key::A],                                         // 7 => 7
        (8, 9) => vec![Key::E, Key::A],                                 // 7 => 8
        (8, 10) => vec![Key::E, Key::E, Key::A],                        // 7 => 9
        (8, 0) => vec![Key::E, Key::E, Key::S, Key::S, Key::S, Key::A], // 7 => A
        (9, 1) => vec![Key::S, Key::S, Key::S, Key::A],                 // 8 => 0
        (9, 2) => vec![Key::W, Key::S, Key::S, Key::A],                 // 8 => 1
        (9, 3) => vec![Key::S, Key::S, Key::A],                         // 8 => 2
        (9, 4) => vec![Key::S, Key::S, Key::E, Key::A],                 // 8 => 3
        (9, 5) => vec![Key::W, Key::S, Key::A],                         // 8 => 4
        (9, 6) => vec![Key::S, Key::A],                                 // 8 => 5
        (9, 7) => vec![Key::S, Key::E, Key::A],                         // 8 => 6
        (9, 8) => vec![Key::W, Key::A],                                 // 8 => 7
        (9, 9) => vec![Key::A],                                         // 8 => 8
        (9, 10) => vec![Key::E, Key::A],                                // 8 => 9
        (9, 0) => vec![Key::S, Key::S, Key::S, Key::E, Key::A],         // 8 => A
        (10, 1) => vec![Key::W, Key::S, Key::S, Key::S, Key::A],        // 9 => 0
        (10, 2) => vec![Key::W, Key::W, Key::S, Key::S, Key::A],        // 9 => 1
        (10, 3) => vec![Key::W, Key::S, Key::S, Key::A],                // 9 => 2
        (10, 4) => vec![Key::S, Key::S, Key::A],                        // 9 => 3
        (10, 5) => vec![Key::W, Key::W, Key::S, Key::A],                // 9 => 4
        (10, 6) => vec![Key::W, Key::S, Key::A],                        // 9 => 5
        (10, 7) => vec![Key::S, Key::A],                                // 9 => 6
        (10, 8) => vec![Key::W, Key::W, Key::A],                        // 9 => 7
        (10, 9) => vec![Key::W, Key::A],                                // 9 => 8
        (10, 10) => vec![Key::A],                                       // 9 => 9
        (10, 0) => vec![Key::S, Key::S, Key::S, Key::A],                // 9 => A
        (0, 1) => vec![Key::W, Key::A],                                 // A => 0
        (0, 2) => vec![Key::N, Key::W, Key::W, Key::A],                 // A => 1
        (0, 3) => vec![Key::W, Key::N, Key::A],                         // A => 2
        (0, 4) => vec![Key::N, Key::A],                                 // A => 3
        (0, 5) => vec![Key::N, Key::N, Key::W, Key::W, Key::A],         // A => 4
        (0, 6) => vec![Key::W, Key::N, Key::N, Key::A],                 // A => 5
        (0, 7) => vec![Key::N, Key::N, Key::A],                         // A => 6
        (0, 8) => vec![Key::N, Key::N, Key::N, Key::W, Key::W, Key::A], // A => 7
        (0, 9) => vec![Key::W, Key::N, Key::N, Key::N, Key::A],         // A => 8
        (0, 10) => vec![Key::N, Key::N, Key::N, Key::A],                // A => 9
        (0, 0) => vec![Key::A],                                         // A => A
        _ => unreachable!(),
    }
}
