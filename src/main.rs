use std::collections::HashSet;
use std::fs::{self};
use std::time::Instant;

use matrix::Matrix;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod matrix;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let t = 1000;
    let mut avg = 0;

    (0..t).for_each(|_| {
        let now = Instant::now();

        let count = guard_gallivant(&input);

        avg += now.elapsed().as_nanos();
        assert_eq!(count, 1480);
    });

    avg /= t;

    println!("avg elapsed time: {:.2?}ns", avg);
}

#[derive(Clone, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn guard_gallivant(input: &str) -> usize {
    let (map, (mut x, mut y)) = parse_puzzle(input);
    let (ox, oy) = (x, y);

    let mut steps = HashSet::new();

    let mut direction = Direction::N;

    loop {
        steps.insert((x, y));
        match direction {
            Direction::N => {
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

    steps.remove(&(ox, oy));

    steps
        .par_iter()
        .map(|(wx, wy)| {
            let mut new_map = map.clone();
            new_map[*wy][*wx] = true;
            if is_looping(new_map, ox, oy) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[derive(Clone, Default, PartialEq)]
enum Path {
    Turn(Direction),
    Continue,
    #[default]
    Untaken,
}

fn is_looping(map: Matrix<bool>, mut x: usize, mut y: usize) -> bool {
    let mut steps = Matrix::<Path>::new(map.width(), map.height());

    let mut direction = Direction::N;

    loop {
        if steps[y][x] == Path::Untaken {
            steps[y][x] = Path::Continue;
        };
        match direction {
            Direction::N => {
                if y == 0 {
                    return false;
                }
                if map[y - 1][x] {
                    if steps[y][x] == Path::Turn(Direction::E) {
                        return true;
                    }
                    match steps[y][x] {
                        Path::Turn(Direction::E) => {
                            return true;
                        }
                        Path::Continue | Path::Untaken => {
                            steps[y][x] = Path::Turn(Direction::E);
                        }
                        _ => (),
                    }
                    direction = Direction::E;
                } else {
                    y -= 1;
                }
            }
            Direction::E => {
                if x == map.width() - 1 {
                    return false;
                }
                if map[y][x + 1] {
                    match steps[y][x] {
                        Path::Turn(Direction::S) => {
                            return true;
                        }
                        Path::Continue | Path::Untaken => {
                            steps[y][x] = Path::Turn(Direction::S);
                        }
                        _ => (),
                    }
                    direction = Direction::S;
                } else {
                    x += 1;
                }
            }
            Direction::S => {
                if y == map.height() - 1 {
                    return false;
                }
                if map[y + 1][x] {
                    match steps[y][x] {
                        Path::Turn(Direction::W) => {
                            return true;
                        }
                        Path::Continue | Path::Untaken => {
                            steps[y][x] = Path::Turn(Direction::W);
                        }
                        _ => (),
                    }
                    direction = Direction::W;
                } else {
                    y += 1;
                }
            }
            Direction::W => {
                if x == 0 {
                    return false;
                }
                if map[y][x - 1] {
                    match steps[y][x] {
                        Path::Turn(Direction::N) => {
                            return true;
                        }
                        Path::Continue | Path::Untaken => {
                            steps[y][x] = Path::Turn(Direction::N);
                        }
                        _ => (),
                    }
                    direction = Direction::N;
                } else {
                    x -= 1;
                }
            }
        }
    }
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
