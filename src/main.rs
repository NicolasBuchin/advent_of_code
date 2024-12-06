use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let t = 1000;
    let mut avg = 0;

    (0..t).for_each(|_| {
        let now = Instant::now();

        let count = guard_gallivant(&input);

        avg += now.elapsed().as_nanos();
        assert_eq!(count, 6204);
    });

    avg /= t;

    println!("avg elapsed time: {:.2?}ns", avg);
}

fn guard_gallivant(input: &str) -> u32 {
    let (walls_x, walls_y, mut guard_position, edge) = parse_puzzle(input);

    let mut steps_count = 0;

    let mut direction = Direction::N;

    loop {
        match direction {
            Direction::N => {
                if let Some(v) = walls_x.get(&guard_position.x) {
                    for &y in v.iter().rev() {
                        if y < guard_position.y {
                            steps_count += guard_position.y - y;
                            direction = Direction::E;
                            break;
                        }
                    }
                } else {
                    return steps_count;
                }
            }
            Direction::E => todo!(),
            Direction::S => todo!(),
            Direction::W => todo!(),
        }

        // d
    }
}

struct Position {
    x: u32,
    y: u32,
}

enum Direction {
    N,
    E,
    S,
    W,
}

fn parse_puzzle(input: &str) -> (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>, Position, Position) {
    let mut walls_x = HashMap::new();
    let mut walls_y = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    let mut guard_position = Position { x: 0, y: 0 };

    input.bytes().for_each(|b| match b {
        b'\n' => {
            y += 1;
            x = 0;
        }
        b'#' => {
            walls_x.entry(x).or_insert_with(Vec::new).push(y);
            walls_y.entry(y).or_insert_with(Vec::new).push(x);
            x += 1;
        }
        b'^' => {
            guard_position.x = x;
            guard_position.y = y;
            x += 1;
        }
        _ => x += 1,
    });

    (walls_x, walls_y, guard_position, Position { x, y })
}
