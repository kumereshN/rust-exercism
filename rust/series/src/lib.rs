pub fn series(digits: &str, len: usize) -> Vec<String> {
   let len_digits = digits.len() + 1;

    match len {
        0 => {
            vec!["".to_string(); len_digits]
        }
        _ => {
            digits
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .windows(len)
                .map(|w|w.concat())
                .collect()
        }
    }
}
