use std::collections::HashSet;

const SIZE: (i32, i32) = (101, 103);
const DURATION: usize = 10000;

#[derive(Default, Clone, Copy, Debug)]
struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

pub fn claw_contraption(input: &str) -> usize {
    let mut robots = parse(input);

    let mut robots_set = HashSet::new();

    for d in 0..DURATION {
        for robot in &mut robots {
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;

            robot.position.0 = robot.position.0.rem_euclid(SIZE.0);
            robot.position.1 = robot.position.1.rem_euclid(SIZE.1);

            robots_set.insert((robot.position.0, robot.position.1));
        }

        if has_tree(&robots_set) {
            println!("AT: {}", d);
            show(&robots_set);
            return d + 1;
        }

        robots_set.clear();
    }

    0
}

fn has_tree(robots_set: &HashSet<(i32, i32)>) -> bool {
    for (x, y) in robots_set {
        let mut found = true;
        for i in -2..=2 {
            for j in -2..=2 {
                if !robots_set.contains(&(x + i, y + j)) {
                    found = false;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

fn show(robots_set: &HashSet<(i32, i32)>) {
    for i in 0..SIZE.0 {
        for j in 0..SIZE.1 {
            if robots_set.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse(input: &str) -> Vec<Robot> {
    let bytes = input.as_bytes();
    let mut robots = Vec::new();

    let mut i = 2;

    let mut number = 0;
    let mut positive = true;

    while i < bytes.len() {
        let mut robot = Robot::default();

        if bytes[i] == b'-' {
            positive = false;
            i += 1;
        }
        while bytes[i] != b',' {
            number *= 10;
            number += bytes[i] as i32 - 0x30;
            i += 1;
        }

        if positive {
            robot.position.0 = number
        } else {
            robot.position.0 = -number
        }

        i += 1;

        number = 0;
        positive = true;

        if bytes[i] == b'-' {
            positive = false;
            i += 1;
        }
        while bytes[i] != b' ' {
            number *= 10;
            number += bytes[i] as i32 - 0x30;
            i += 1;
        }

        if positive {
            robot.position.1 = number
        } else {
            robot.position.1 = -number
        }

        i += 3;

        number = 0;
        positive = true;

        if bytes[i] == b'-' {
            positive = false;
            i += 1;
        }
        while bytes[i] != b',' {
            number *= 10;
            number += bytes[i] as i32 - 0x30;
            i += 1;
        }

        if positive {
            robot.velocity.0 = number
        } else {
            robot.velocity.0 = -number
        }

        i += 1;

        number = 0;
        positive = true;

        if bytes[i] == b'-' {
            positive = false;
            i += 1;
        }
        while bytes[i] != b'\n' {
            number *= 10;
            number += bytes[i] as i32 - 0x30;
            i += 1;
        }

        if positive {
            robot.velocity.1 = number
        } else {
            robot.velocity.1 = -number
        }

        i += 3;
        number = 0;
        positive = true;

        robots.push(robot);
    }

    robots
}
