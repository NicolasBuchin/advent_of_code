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
                walls.insert((x + 1, y));
                x += 2;
            }
            b'O' => {
                boxes.insert((x, y));
                x += 2;
            }
            b'@' => {
                robot = (x, y);
                x += 2;
            }
            b'.' => {
                x += 2;
            }
            _ => (),
        }
        i += 1;
    }

    // show_puzzle(robot, &walls, &boxes, width, y);

    while i < bytes.len() {
        let b = bytes[i];
        match b {
            b'^' => try_move_robot(&mut robot, &Direction::N, &walls, &mut boxes),
            b'v' => try_move_robot(&mut robot, &Direction::S, &walls, &mut boxes),
            b'<' => try_move_robot(&mut robot, &Direction::W, &walls, &mut boxes),
            b'>' => try_move_robot(&mut robot, &Direction::E, &walls, &mut boxes),
            _ => (),
        }
        i += 1;

        // if b == b'\n' {
        //     continue;
        // }
        // println!("move = {}", char::from(b));
        // show_puzzle(robot, &walls, &boxes, width, y);
        // println!();
    }

    // show_puzzle(robot, &walls, &boxes, width, y);

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
    direction: &Direction,
    walls: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
) {
    let next = next_position(*robot, direction);
    let next_left = (next.0 - 1, next.1);
    if walls.contains(&next) {
        return;
    }
    match *direction {
        Direction::N => {
            if can_move_box(next, direction, walls, boxes) && can_move_box(next_left, direction, walls, boxes) {
                move_box(next, direction, boxes);
                move_box(next_left, direction, boxes);
                *robot = next;
            }
        }
        Direction::S => {
            if can_move_box(next, direction, walls, boxes) && can_move_box(next_left, direction, walls, boxes) {
                move_box(next, direction, boxes);
                move_box(next_left, direction, boxes);
                *robot = next;
            }
        }
        Direction::W => {
            if can_move_box(next, direction, walls, boxes) && can_move_box(next_left, direction, walls, boxes) {
                move_box(next, direction, boxes);
                move_box(next_left, direction, boxes);
                *robot = next;
            }
        }
        Direction::E => {
            if can_move_box(next, direction, walls, boxes) {
                move_box(next, direction, boxes);
                *robot = next;
            }
        }
    }
}

fn can_move_box(
    box_position: (usize, usize),
    direction: &Direction,
    walls: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
) -> bool {
    if !boxes.contains(&box_position) {
        return true;
    }
    let next = next_position(box_position, direction);
    let next_left = (next.0 - 1, next.1);
    let next_right = (next.0 + 1, next.1);
    if walls.contains(&next) || walls.contains(&next_right) {
        return false;
    }
    match *direction {
        Direction::N => {
            can_move_box(next, direction, walls, boxes)
                && can_move_box(next_left, direction, walls, boxes)
                && can_move_box(next_right, direction, walls, boxes)
        }
        Direction::S => {
            can_move_box(next, direction, walls, boxes)
                && can_move_box(next_left, direction, walls, boxes)
                && can_move_box(next_right, direction, walls, boxes)
        }
        Direction::W => can_move_box(next, direction, walls, boxes) && can_move_box(next_left, direction, walls, boxes),
        Direction::E => {
            can_move_box(next, direction, walls, boxes) && can_move_box(next_right, direction, walls, boxes)
        }
    }
}

fn move_box(box_position: (usize, usize), direction: &Direction, boxes: &mut HashSet<(usize, usize)>) {
    if !boxes.contains(&box_position) {
        return;
    }
    let next = next_position(box_position, direction);
    let next_left = (next.0 - 1, next.1);
    let next_right = (next.0 + 1, next.1);
    match *direction {
        Direction::N => {
            move_box(next, direction, boxes);
            move_box(next_left, direction, boxes);
            move_box(next_right, direction, boxes);
        }
        Direction::S => {
            move_box(next, direction, boxes);
            move_box(next_left, direction, boxes);
            move_box(next_right, direction, boxes);
        }
        Direction::W => {
            move_box(next, direction, boxes);
            move_box(next_left, direction, boxes);
        }
        Direction::E => {
            move_box(next, direction, boxes);
            move_box(next_right, direction, boxes);
        }
    }
    boxes.remove(&box_position);
    boxes.insert(next);
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
    robot: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    boxes: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) {
    for j in 0..height {
        for i in 0..width {
            if walls.contains(&(i, j)) {
                print!("#");
            } else if boxes.contains(&(i, j)) {
                print!("[");
            } else if i > 0 && boxes.contains(&(i - 1, j)) {
                print!("]");
            } else if (i, j) == robot {
                print!("@");
            } else {
                print!(".")
            }
        }
        println!();
    }
}
