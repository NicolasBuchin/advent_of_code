use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn bridge_repair_par(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| {
            let mut test_numbers = [(0usize, 0usize); 100];
            let mut test_numbers_len = 0;

            let (digits, digits_mul, target) = l
                .bytes()
                .scan((0, 1, 0), |state, b| {
                    let (digits, digits_mul, target) = state;
                    if b.is_ascii_digit() {
                        *digits = *digits * 10 + b as usize - 0x30;
                        *digits_mul *= 10;
                    } else if b == b':' {
                        *target = *digits;
                        *digits = 0;
                        *digits_mul = 1;
                    } else if b == b' ' && *digits != 0 {
                        test_numbers[test_numbers_len] = (*digits_mul, *digits);
                        test_numbers_len += 1;
                        *digits = 0;
                        *digits_mul = 1;
                    }
                    Some((*digits, *digits_mul, *target))
                })
                .last()
                .unwrap();

            test_numbers[test_numbers_len] = (digits_mul, digits);
            test_numbers_len += 1;

            if find_solution(&test_numbers, target, test_numbers_len) {
                return target;
            }
            0
        })
        .sum()
}

fn find_solution(test_numbers: &[(usize, usize)], target: usize, i: usize) -> bool {
    if target == 0 {
        return true;
    }
    if i == 0 {
        return false;
    }
    let (digits_mul, digits) = test_numbers[i - 1];
    (target.rem_euclid(digits) == 0 && find_solution(test_numbers, target.div_euclid(digits), i - 1))
        || (target.rem_euclid(digits_mul) == digits
            && find_solution(test_numbers, target.div_euclid(digits_mul), i - 1))
        || (target >= digits && find_solution(test_numbers, target - digits, i - 1))
}
