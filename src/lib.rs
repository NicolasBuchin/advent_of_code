pub fn bridge_repair(input: &str) -> usize {
    let mut test_numbers = [0usize; 1024];
    let mut test_numbers_len = 0;

    let mut digits = 0;
    let mut target = 0;

    let mut solution_count = 0;

    input.bytes().for_each(|b| {
        if b.is_ascii_digit() {
            digits = digits * 10 + b as usize - 0x30;
        } else if b == b':' {
            target = digits;
            digits = 0;
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
                    });
                }
            }
            digits = 0;
        } else if b == b'\n' {
            for &n in &test_numbers[0..test_numbers_len] {
                if n + digits == target || n * digits == target {
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
