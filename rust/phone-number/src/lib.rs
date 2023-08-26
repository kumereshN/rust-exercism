pub fn number(user_number: &str) -> Option<String> {
    let clean_user_number_vec = user_number
        .split(|c: char| c.is_ascii_whitespace() || c == '-' || c == '.')
        .flat_map(|s| {
            s
                .chars()
                .filter_map(|c| {
                    match c.is_numeric() {
                        true => Some(Some(c)),
                        false => None
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Option<char>>>();

    let is_all_numeric_digits = clean_user_number_vec.iter().all(|c| c.is_some());

    match (clean_user_number_vec.len(), is_all_numeric_digits) {
        (10, true) => {
          Some(clean_user_number_vec.iter().flatten().collect::<String>())
        },
        (11, true) => {
            let clean_user_number = clean_user_number_vec.iter().flatten().collect::<String>();
            let mut clean_user_number_iter = clean_user_number.chars();
            let country_code = clean_user_number_iter.next().unwrap();
            match country_code {
                '1' => {
                    let mut remaining_user_number_peekable = clean_user_number_iter.peekable();
                    let first_char_area_code = remaining_user_number_peekable.peek().unwrap().to_digit(10);
                    match first_char_area_code {
                        Some((2..=9)) => {
                            Some(remaining_user_number_peekable.collect::<String>())
                        },
                        _ => None
                    }

                },
                _ => None
            }
        },
        (_,_) => None
    }

}
