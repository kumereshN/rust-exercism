fn calculate_rows_cols(input_len: usize) -> (usize, usize) {
    let (mut r, mut c) = (1_usize, 1_usize);
    while r * c <= input_len {
        if r * c == input_len {
            return (r, c)
        }
        r += 1_usize;
        c += 1_usize;
    }
    (r-1, c)
}

pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        "".to_string()
    }
    else {
        let normalized_chars = input
                                        .to_ascii_lowercase()
                                        .chars()
                                        .filter(|&c| c.is_alphabetic())
                                        .collect::<Vec<char>>();
        let len_of_input = normalized_chars.len();
        let (r, c) = calculate_rows_cols(len_of_input);

        let chunks_iter = normalized_chars.chunks_exact(c);
        let mut vec_of_slice_chars = vec![];


        if !chunks_iter.remainder().is_empty() {
                let padding_length = c - chunks_iter.remainder().len();
                let chunks_remainder = chunks_iter.remainder().iter().collect::<String>();
                let padded_string = format!("{:<width$}", chunks_remainder, width = chunks_iter.remainder().len() + padding_length);
                let padding_vec_char = padded_string.chars().collect::<Vec<char>>();
                vec_of_slice_chars.push(padding_vec_char)
        }


        for s in chunks_iter.rev() {
            vec_of_slice_chars.insert(0,s.to_vec())
        }

        let mut res = vec![];

        for col in 0..c {
            for row in 0..r {
                res.push(vec_of_slice_chars[row][col])
            }
        }

        res
            .chunks(r)
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
            .to_string()
    }
}
