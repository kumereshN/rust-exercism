pub fn number(user_number: &str) -> Option<String> {
    let clean_user_number = user_number
        .split(|c: char| c.is_ascii_whitespace() || c == '-' || c == '.')
        .flat_map(|s| {
            s
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>()
        })
        .collect::<String>();

    match clean_user_number.len() {
        10 => {
          Some(clean_user_number)
        },
        11 => {
            let mut clean_user_number_iter = clean_user_number.chars();
            let first_char = clean_user_number_iter.next();
            match first_char {
                Some('1') => {
                    Some(clean_user_number_iter.collect::<String>())
                },
                _ => None
            }
        },
        _ => None
    }

}
