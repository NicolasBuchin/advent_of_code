pub fn code_chronicle(input: &str) -> usize {
    let (locks, keys) = parse(input);
    solve(locks, keys)
}

fn solve(locks: Vec<[u8; 5]>, keys: Vec<[u8; 5]>) -> usize {
    let mut solutions = 0;
    for lock in &locks {
        for key in &keys {
            let mut fits = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    fits = false;
                    break;
                }
            }
            if fits {
                solutions += 1;
            }
        }
    }
    solutions
}

fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let bytes = input.as_bytes();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut i = 0;
    while i < bytes.len() {
        let is_lock = bytes[i] == b'#';
        i += 6;
        let mut nums = [0, 0, 0, 0, 0];
        for _ in 0..5 {
            for n in &mut nums {
                if bytes[i] == b'#' {
                    *n += 1;
                }
                i += 1;
            }
            i += 1;
        }
        i += 7;
        if is_lock {
            locks.push(nums);
        } else {
            keys.push(nums);
        }
    }

    (locks, keys)
}
