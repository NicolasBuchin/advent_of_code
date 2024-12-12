pub fn garden_groups(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut islands = [0usize; 26];
    let mut previous_line = [b'/'; 140];
    let mut current_line = [b'/'; 140];

    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;

    while j < bytes.len() {
        if bytes[j] == b'\n' {
            previous_line = current_line;
            current_line = [b'/'; 140];
            i = 0;
            j += 1;
            continue;
        }
        let value = bytes[j] - b'A';
        if previous_line[i] == value && i > 0 && current_line[i - 1] == value {
            // idk
        } else if previous_line[i] == value || (i > 0 && current_line[i - 1] == value) {
            islands[value as usize] += 2;
        } else {
            sum += islands[value as usize] * value as usize;
            println!("{} : {}", char::from(value + b'A'), islands[value as usize]);
            islands[value as usize] = 4;
        }
        current_line[i] = value;
        i += 1;
        j += 1;
    }

    for x in j - i..j {
        sum += islands[x] * x;
        println!("{} : {}", char::from(x as u8 + b'A'), islands[x]);
    }

    sum
}
