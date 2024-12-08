pub fn resonant_collinearity(input: &str) -> usize {
    let mut input_clone = input.to_owned();
    let bytes: &mut [u8] = unsafe { input_clone.as_bytes_mut() };

    let line_size = {
        let mut i = 0;
        while bytes[i] != b'\n' {
            i += 1;
        }
        i + 1
    };

    // [frequency] -> ([positions], index of last position)
    let mut antenna_table = [([0usize; 4], 0usize); (b'z' - b'0') as usize + 1];
    bytes.iter().enumerate().for_each(|(i, &byte)| {
        if byte == b'.' || byte == b'\n' {
            return;
        }
        let b = (byte - b'0') as usize;
        antenna_table[b].0[antenna_table[b].1] = i;
        antenna_table[b].1 += 1;
    });

    let mut antinode_count = 0;
    antenna_table.iter().for_each(|(positions, size)| {
        if *size != 0 {
            (0..*size).for_each(|i| {
                let antenna1 = positions[i];
                let antenna1_line = antenna1 / line_size;

                ((i + 1)..*size).for_each(|j| {
                    let antenna2 = positions[j];
                    let antenna2_line = antenna2 / line_size;
                    let difference = antenna2 - antenna1;
                    let line_difference = antenna2_line - antenna1_line;
                    {
                        let mut difference2 = 0;
                        let mut line_difference2 = 0;
                        while difference2 <= antenna1 && line_difference2 <= antenna1_line {
                            if bytes[antenna1 - difference2] != b'#'
                                && bytes[antenna1 - difference2] != b'\n'
                                && (antenna1 - difference2) / line_size == antenna1_line - line_difference2
                            {
                                antinode_count += 1;
                                bytes[antenna1 - difference2] = b'#';
                            }
                            difference2 += difference;
                            line_difference2 += line_difference;
                        }
                    }
                    {
                        let mut difference2 = 0;
                        let mut line_difference2 = 0;
                        while antenna2 + difference2 < bytes.len() {
                            if bytes[antenna2 + difference2] != b'#'
                                && bytes[antenna2 + difference2] != b'\n'
                                && (antenna2 + difference2) / line_size == antenna2_line + line_difference2
                            {
                                antinode_count += 1;
                                bytes[antenna2 + difference2] = b'#';
                            }
                            difference2 += difference;
                            line_difference2 += line_difference;
                        }
                    }
                });
            });
        }
    });
    antinode_count
}
