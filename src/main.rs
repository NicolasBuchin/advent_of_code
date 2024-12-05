use std::fs::{self};
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let t = 1000;
    let mut avg = 0;

    (0..t).for_each(|_| {
        let now = Instant::now();

        let count = print_queue(&input);

        avg += now.elapsed().as_nanos();
        assert_eq!(count, 4905);
    });

    avg /= t;

    println!("avg elapsed time: {:.2?}ns", avg);
}

fn print_queue(input: &str) -> i32 {
    let bytes = &input.bytes().collect::<Vec<_>>();
    let mut rules = [0u128; 100];

    let mut i = 0;

    loop {
        let key = bytes_to_word(bytes[i + 3], bytes[i + 4]);
        let value: u128 = 1u128 << bytes_to_word(bytes[i], bytes[i + 1]);

        rules[key] |= value;
        i += 6;

        if bytes[i - 1] == b'\n' && bytes[i] == b'\n' {
            i += 1;
            break;
        }
    }

    let mut passed_flag = 0u128;
    let mut ok = true;
    let mut count = 0;
    let mut index = i;

    while i < bytes.len() {
        if !ok {
            i += 3;
            if bytes[i - 1] == b'\n' {
                passed_flag = 0;
                index = i;
                ok = true;
            }
        } else {
            let word = bytes_to_word(bytes[i], bytes[i + 1]);
            let flag = rules[word];
            if flag & passed_flag != passed_flag {
                ok = false;
            }
            passed_flag |= 1u128 << word;
            i += 3;

            if bytes[i - 1] == b'\n' {
                if ok {
                    let p = index + (i - index).div_euclid(2);
                    count += bytes_to_word(bytes[p - 1], bytes[p]) as i32;
                }
                passed_flag = 0;
                index = i;
                ok = true;
            }
        }
    }
    count
}

fn bytes_to_word(d: u8, u: u8) -> usize {
    ((d - 48) * 10 + u - 48) as usize
}
