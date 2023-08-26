use std::iter::Peekable;
use std::str::Chars;

fn is_valid_area_code(phone_no: &mut Peekable<Chars>) -> bool {
    let first_char_area_code = phone_no.peek().unwrap().to_digit(10);
    matches!(first_char_area_code, Some((2..=9)))
}

fn is_valid_exchange_code(phone_no: Vec<char>) -> bool {
    let first_char_exchange_code = phone_no.get(3).unwrap().to_digit(10);
    matches!(first_char_exchange_code, Some((2..=9)))
}

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
    let clean_user_number = clean_user_number_vec.iter().flatten().collect::<String>();

    match (clean_user_number.len(), is_all_numeric_digits) {
        (10, true) => {
            let mut clean_user_number_peekable = clean_user_number.chars().peekable();
            match (is_valid_area_code(&mut clean_user_number_peekable), is_valid_exchange_code(clean_user_number.chars().collect::<Vec<char>>())) {
                (true, true) => Some(clean_user_number),
                (_,_) => None
            }
        },
        (11, true) => {
            let mut clean_user_number_iter = clean_user_number.chars();
            let country_code = clean_user_number_iter.next().unwrap();
            let mut remaining_user_number_peekable = clean_user_number_iter.clone().peekable();
            match country_code {
                '1' => {
                    match (is_valid_area_code(&mut remaining_user_number_peekable), is_valid_exchange_code(clean_user_number_iter.clone().collect::<Vec<char>>())) {
                        (true, true) => {
                            Some(clean_user_number_iter.collect::<String>())
                        },
                        (_,_) => None
                    }
                },
                _ => None
            }
        },
        (_,_) => None
    }

}
