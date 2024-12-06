use std::fs::{self};
use std::time::Instant;

use matrix::Matrix;

mod matrix;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let t = 1000;
    let mut avg = 0;

    (0..t).for_each(|_| {
        let now = Instant::now();

        let count = guard_gallivant(&input);

        avg += now.elapsed().as_nanos();
        assert_eq!(count, 4580);
    });

    avg /= t;

    println!("avg elapsed time: {:.2?}ns", avg);
}

fn guard_gallivant(input: &str) -> usize {
    let (map, (mut x, mut y)) = parse_puzzle(input);

    let mut steps = Matrix::new(map.width(), map.height());

    let mut direction = Direction::N;

    loop {
        match direction {
            Direction::N => {
                steps[y][x] = true;
                if y == 0 {
                    break;
                }
                if map[y - 1][x] {
                    direction = Direction::E;
                } else {
                    y -= 1;
                }
            }
            Direction::E => {
                steps[y][x] = true;
                if x == map.width() - 1 {
                    break;
                }
                if map[y][x + 1] {
                    direction = Direction::S;
                } else {
                    x += 1;
                }
            }
            Direction::S => {
                steps[y][x] = true;
                if y == map.height() - 1 {
                    break;
                }
                if map[y + 1][x] {
                    direction = Direction::W;
                } else {
                    y += 1;
                }
            }
            Direction::W => {
                steps[y][x] = true;
                if x == 0 {
                    break;
                }
                if map[y][x - 1] {
                    direction = Direction::N;
                } else {
                    x -= 1;
                }
            }
        }
    }

    count_steps(&steps.data)
}

fn count_steps(steps: &[bool]) -> usize {
    steps.iter().filter(|&b| *b).count()
}

enum Direction {
    N,
    E,
    S,
    W,
}

fn parse_puzzle(input: &str) -> (Matrix<bool>, (usize, usize)) {
    let mut map = Vec::with_capacity(130 * 130);

    let mut width = 0;
    let mut width_found = false;

    let mut guard_position = (0, 0);

    input.bytes().for_each(|b| {
        match b {
            b'\n' => width_found = true,
            b'#' => map.push(true),
            b'^' => {
                if width_found {
                    guard_position = (map.len().rem_euclid(width), map.len().div_euclid(width));
                } else {
                    guard_position = (0, map.len());
                }
                map.push(false)
            }
            _ => map.push(false),
        }
        if !width_found {
            width += 1;
        }
    });

    let map = Matrix::make(map.clone(), width, map.len().div_euclid(width));

    (map, guard_position)
}
