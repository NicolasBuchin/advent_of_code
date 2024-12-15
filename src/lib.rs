use std::collections::HashSet;

pub fn warehouse_woes(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = (0, 0);

    let (mut x, mut y) = (0, 0);
    let mut width = 0;
    let mut i = 0;

    loop {
        let b = bytes[i];
        match b {
            b'\n' => {
                if x == 0 {
                    i += 1;
                    break;
                }
                width = x;
                x = 0;
                y += 1;
            }
            b'#' => {
                walls.insert((x, y));
                x += 1;
            }
            b'O' => {
                boxes.insert((x, y));
                x += 1;
            }
            b'@' => {
                robot = (x, y);
                x += 1;
            }
            b'.' => {
                x += 1;
            }
            _ => (),
        }
        i += 1;
    }

    // show_puzzle(&robot, &walls, &boxes, width, y);

    while i < bytes.len() {
        let b = bytes[i];
        match b {
            b'^' => try_move_robot(&mut robot, Direction::N, &walls, &mut boxes),
            b'v' => try_move_robot(&mut robot, Direction::S, &walls, &mut boxes),
            b'<' => try_move_robot(&mut robot, Direction::W, &walls, &mut boxes),
            b'>' => try_move_robot(&mut robot, Direction::E, &walls, &mut boxes),
            _ => (),
        }
        i += 1;

        // if b == b'\n' {
        //     continue;
        // }
        // println!("move = {}", char::from(b));
        // show_puzzle(&robot, &walls, &boxes, width, y);
        // println!();
    }

    boxes.iter().map(|(i, j)| i + j * 100).sum()
}

enum Direction {
    N,
    S,
    W,
    E,
}

fn try_move_robot(
    robot: &mut (usize, usize),
    direction: Direction,
    walls: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
) {
    let next = next_position(*robot, &direction);
    if walls.contains(&next) {
        return;
    }
    if boxes.contains(&next) {
        if try_move_box(next, direction, walls, boxes) {
            *robot = next;
        }
    } else {
        *robot = next;
    }
}

fn try_move_box(
    box_position: (usize, usize),
    direction: Direction,
    walls: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
) -> bool {
    let next = next_position(box_position, &direction);
    if walls.contains(&next) {
        false
    } else if boxes.contains(&next) {
        let moved = try_move_box(next, direction, walls, boxes);
        if moved {
            boxes.remove(&box_position);
            boxes.insert(next);
        }
        moved
    } else {
        boxes.remove(&box_position);
        boxes.insert(next);
        true
    }
}

fn next_position(position: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::N => (position.0, position.1 - 1),
        Direction::S => (position.0, position.1 + 1),
        Direction::W => (position.0 - 1, position.1),
        Direction::E => (position.0 + 1, position.1),
    }
}

fn show_puzzle(
    robot: &(usize, usize),
    walls: &HashSet<(usize, usize)>,
    boxes: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for j in 0..height {
        for i in 0..width {
            let position = (i, j);
            if walls.contains(&position) {
                print!("#");
            } else if boxes.contains(&position) {
                print!("O");
            } else if position == *robot {
                print!("@");
            } else {
                print!(".")
            }
        }
        println!();
    }
}
