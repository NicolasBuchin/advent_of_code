use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn bridge_repair_par(input: &str) -> usize {
    input
        .par_lines()
        .map(|l| {
            let bytes = l.as_bytes();
            let mut i = 0;

            let target = {
                let mut t = 0;
                while bytes[i] != b':' {
                    t = 10 * t + bytes[i] as usize - 0x30;
                    i += 1;
                }
                t
            };

            i += 2;

            let (test_numbers, test_numbers_mul, test_numbers_len) = {
                let mut test_numbers = [0usize; 12];
                let mut test_numbers_mul = [0usize; 12];
                let mut test_numbers_len = 0;
                bytes[i..]
                    .split(|&b| b == b' ')
                    .map(|chunk| {
                        chunk
                            .iter()
                            .fold((0, 1), |(d, dm), &b| (d * 10 + (b as usize - 0x30), dm * 10))
                    })
                    .enumerate()
                    .for_each(|(idx, (digits, digits_mul))| {
                        test_numbers[idx] = digits;
                        test_numbers_mul[idx] = digits_mul;
                        test_numbers_len += 1;
                    });
                (test_numbers, test_numbers_mul, test_numbers_len)
            };

            if find_solution(&test_numbers, &test_numbers_mul, target, test_numbers_len) {
                return target;
            }
            0
        })
        .sum()
}

fn find_solution(test_numbers: &[usize], test_numbers_mul: &[usize], target: usize, i: usize) -> bool {
    if target == 0 {
        return true;
    }
    if i == 0 {
        return false;
    }
    let digits = test_numbers[i - 1];
    let digits_mul = test_numbers_mul[i - 1];
    (target.rem_euclid(digits) == 0 && find_solution(test_numbers, test_numbers_mul, target.div_euclid(digits), i - 1))
        || (target.rem_euclid(digits_mul) == digits
            && find_solution(test_numbers, test_numbers_mul, target.div_euclid(digits_mul), i - 1))
        || (target >= digits && find_solution(test_numbers, test_numbers_mul, target - digits, i - 1))
}
