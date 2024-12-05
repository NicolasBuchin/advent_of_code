use std::collections::HashMap;
use std::fs::{self};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let t = 1;
    let mut avg = 0;

    (0..t).for_each(|_| {
        let now = Instant::now();

        let count = print_queue(&input);

        avg += now.elapsed().as_nanos();
        println!("count = {}", count)
        // assert_eq!(count, 1831);
    });

    avg /= t;

    println!("avg elapsed time: {:.2?}ns", avg);
}

enum State {
    Rules,
    Evaluation,
}

fn print_queue(input: &str) -> usize {
    let mut order = HashMap::<i32, i32>::new();
    let mut state = State::Rules;
    let mut next_state = false;

    let mut key: i32 = 0;
    let mut target = Vec::new();

    let mut count = 0;

    input.bytes().for_each(|b| match state {
        State::Rules => {
            if b.is_ascii_digit() {
                target.push(b);
            } else {
                match b {
                    b'|' => {
                        key = target.iter().fold(0i32, |acc, &digit| acc * 10 + (digit - b'0') as i32);
                        target.clear();
                    }
                    b'\n' => {
                        if next_state {
                            state = State::Evaluation
                        } else {
                            let value = target.iter().fold(0i32, |acc, &digit| acc * 10 + (digit - b'0') as i32);
                            order.insert(key, value);
                            target.clear();
                            next_state = true;
                        }
                    }
                    _ => (),
                }
            }
        }
        State::Evaluation => {
            if b.is_ascii_digit() {
                target.push(b);
            } else {
                match b {
                    b',' => {
                        ();
                    }
                    b'\n' => {
                        count += 1;
                    }
                    _ => (),
                }
            }
        }
    });

    count
}
