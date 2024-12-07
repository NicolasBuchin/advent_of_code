use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn bridge_repair(input: &str) -> usize {
    let mut test_numbers = [0usize; 60_000];
    let mut test_numbers_len = 0;

    let mut digits = 0;
    let mut digits_mul = 1;
    let mut target = 0;

    let mut solution_count = 0;

    input.bytes().for_each(|b| {
        if b.is_ascii_digit() {
            digits = digits * 10 + b as usize - 0x30;
            digits_mul *= 10;
        } else if b == b':' {
            target = digits;
            digits = 0;
            digits_mul = 1;
        } else if b == b' ' {
            if digits != 0 {
                if test_numbers_len == 0 {
                    test_numbers[0] = digits;
                    test_numbers_len = 1;
                } else {
                    (0..test_numbers_len).for_each(|i| {
                        let x = test_numbers[i];
                        test_numbers[i] = digits + x;
                        test_numbers[test_numbers_len] = digits * x;
                        test_numbers_len += 1;
                        test_numbers[test_numbers_len] = x * digits_mul + digits;
                        test_numbers_len += 1;
                    });
                }
            }
            digits = 0;
            digits_mul = 1;
        } else if b == b'\n' {
            for &n in &test_numbers[0..test_numbers_len] {
                if n + digits == target || n * digits == target || n * digits_mul + digits == target {
                    solution_count += target;
                    break;
                }
            }
            digits = 0;
            target = 0;
            test_numbers_len = 0;
        }
    });

    solution_count
}

pub fn bridge_repair_par(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| {
            let mut test_numbers = [0usize; 60_000];
            let mut test_numbers_len = 0;

            let mut digits = 0;
            let mut digits_mul = 1;
            let mut target = 0;

            let mut solution_count = 0;

            l.bytes().for_each(|b| {
                if b.is_ascii_digit() {
                    digits = digits * 10 + b as usize - 0x30;
                    digits_mul *= 10;
                } else if b == b':' {
                    target = digits;
                    digits = 0;
                    digits_mul = 1;
                } else if b == b' ' {
                    if digits != 0 {
                        if test_numbers_len == 0 {
                            test_numbers[0] = digits;
                            test_numbers_len = 1;
                        } else {
                            (0..test_numbers_len).for_each(|i| {
                                let x = test_numbers[i];
                                test_numbers[i] = digits + x;
                                test_numbers[test_numbers_len] = digits * x;
                                test_numbers_len += 1;
                                test_numbers[test_numbers_len] = x * digits_mul + digits;
                                test_numbers_len += 1;
                            });
                        }
                    }
                    digits = 0;
                    digits_mul = 1;
                }
            });
            for &n in &test_numbers[0..test_numbers_len] {
                if n + digits == target || n * digits == target || n * digits_mul + digits == target {
                    solution_count += target;
                    break;
                }
            }
            solution_count
        })
        .sum()
}
