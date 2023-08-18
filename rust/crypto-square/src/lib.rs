pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        "".to_string()
    }
    else {
        let normalized_chars = input
                                        .to_ascii_lowercase()
                                        .chars()
                                        .filter(|&c| c.is_alphanumeric())
                                        .collect::<Vec<char>>();
        let len_of_input = normalized_chars.len();
        let width = (len_of_input as f64).sqrt().ceil() as usize;

        (0..width)
            .map(|i| {
                normalized_chars.chunks(width)
                    .map(|v| v.get(i).unwrap_or(&' '))
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
