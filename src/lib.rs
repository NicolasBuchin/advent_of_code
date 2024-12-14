const SIZE: (i32, i32) = (101, 103);
const DURATION: i32 = 100;

#[derive(Default, Clone, Copy, Debug)]
struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

pub fn claw_contraption(input: &str) -> usize {
    let robots = parse(input);

    let mut quadrants = [0; 4];

    for mut robot in robots {
        robot.position.0 += robot.velocity.0 * DURATION;
        robot.position.1 += robot.velocity.1 * DURATION;

        robot.position.0 = robot.position.0.rem_euclid(SIZE.0);
        robot.position.1 = robot.position.1.rem_euclid(SIZE.1);

        if robot.position.0 < SIZE.0 / 2 && robot.position.1 < SIZE.1 / 2 {
            quadrants[0] += 1;
        } else if robot.position.0 > SIZE.0 / 2 && robot.position.1 < SIZE.1 / 2 {
            quadrants[1] += 1;
        } else if robot.position.0 < SIZE.0 / 2 && robot.position.1 > SIZE.1 / 2 {
            quadrants[2] += 1;
        } else if robot.position.0 > SIZE.0 / 2 && robot.position.1 > SIZE.1 / 2 {
            quadrants[3] += 1;
        }

        // println!("{:?}", robot);
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
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
