pub fn disk_fragmenter(input: &str) -> usize {
    let mut memory: Vec<(usize, u8, usize)> = Vec::new();
    let mut open: Vec<(usize, u8)> = Vec::new();

    let mut biggest_open_size = 0;

    {
        let mut idx = 0;
        let bytes = input.as_bytes();
        let mut i = 0;

        loop {
            memory.push((idx, bytes[i] - 0x30, i >> 1));
            idx += bytes[i] as usize - 0x30;

            if i == input.len() - 1 {
                break;
            }

            let open_size = bytes[i + 1] - 0x30;
            open.push((idx, open_size));
            idx += bytes[i + 1] as usize - 0x30;

            if open_size > biggest_open_size {
                biggest_open_size = open_size;
            }

            i += 2;
        }
    }

    let mut sum = 0;

    for i in (0..memory.len()).rev() {
        let (mut memory_position, memory_size, v) = memory[i];
        if memory_size > biggest_open_size {
            continue;
        }
        for j in 0..open.len() {
            let (mut open_position, mut open_size) = open[j];
            if open_position > memory_position {
                break;
            }
            if open_size >= memory_size {
                memory_position = open_position;
                open_size -= memory_size;
                open_position += memory_size as usize;
                open[j] = (open_position, open_size);
                break;
            }
        }
        memory[i] = (memory_position, memory_size, v);
        let size = memory_size as usize;
        let value = v as usize;
        let sum_of_indices = size * memory_position + ((size * (size - 1)) >> 1);
        sum += sum_of_indices * value;
    }

    sum
}
