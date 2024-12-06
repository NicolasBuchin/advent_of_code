mod matrix;
use matrix::Matrix;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Default, PartialEq, Debug)]
enum Path {
    Turn(Direction),
    Wall,
    #[default]
    Empty,
}

pub fn guard_gallivant(input: &str) -> usize {
    let (map, (mut x, mut y)) = parse_puzzle(input);
    let (ox, oy) = (x, y);

    let mut steps = HashSet::new();
    let mut direction = Direction::N;

    loop {
        steps.insert((x, y));
        if is_edge(x, y, map.width(), map.height(), &direction) {
            break;
        } else {
            let (mut next_x, mut next_y) = next_position(x, y, &direction);
            while map[next_y][next_x] == Path::Wall {
                direction = next_direction(&direction);
                (next_x, next_y) = next_position(x, y, &direction);
            }
            (x, y) = (next_x, next_y);
        }
    }

    steps.remove(&(ox, oy));

    steps
        .par_iter()
        .map(|(x, y)| {
            let mut new_map = map.clone();
            new_map[*x][*y] = Path::Wall;
            is_looping(new_map, ox, oy)
        })
        .sum()
}

fn parse_puzzle(input: &str) -> (Matrix<Path>, (usize, usize)) {
    let mut map = Vec::with_capacity(130 * 130);

    let mut width = 0;
    let mut width_found = false;

    let mut guard_position = (0, 0);

    input.bytes().for_each(|b| {
        match b {
            b'\n' => width_found = true,
            b'#' => map.push(Path::Wall),
            b'^' => {
                guard_position = (map.len().rem_euclid(width), map.len().div_euclid(width));
                map.push(Path::Empty)
            }
            _ => map.push(Path::Empty),
        }
        if !width_found {
            width += 1;
        }
    });

    let map = Matrix::make(map.clone(), width, map.len().div_euclid(width));

    (map, guard_position)
}

fn next_direction(d: &Direction) -> Direction {
    match d {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

fn next_position(x: usize, y: usize, d: &Direction) -> (usize, usize) {
    match d {
        Direction::N => (x, y - 1),
        Direction::E => (x + 1, y),
        Direction::S => (x, y + 1),
        Direction::W => (x - 1, y),
    }
}

fn is_edge(x: usize, y: usize, width: usize, height: usize, d: &Direction) -> bool {
    match d {
        Direction::N => y == 0,
        Direction::E => x == width - 1,
        Direction::S => y == height - 1,
        Direction::W => x == 0,
    }
}

fn is_looping(mut map: Matrix<Path>, mut x: usize, mut y: usize) -> usize {
    let mut direction = Direction::N;
    loop {
        if is_edge(x, y, map.width(), map.height(), &direction) {
            return 0;
        } else {
            let (mut next_x, mut next_y) = next_position(x, y, &direction);
            while map[next_y][next_x] == Path::Wall {
                direction = next_direction(&direction);
                if map[y][x] == Path::Turn(direction) {
                    return 1;
                }
                if map[y][x] == Path::Empty {
                    map[y][x] = Path::Turn(direction);
                }
                (next_x, next_y) = next_position(x, y, &direction);
            }
            (x, y) = (next_x, next_y);
        }
    }
}
