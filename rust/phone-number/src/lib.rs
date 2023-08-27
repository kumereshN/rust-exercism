use std::iter::Peekable;
use std::str::Chars;

const PHONE_NUMBER_WITHOUT_COUNTRY_CODE: usize = 10;
const PHONE_NUMBER_WITH_COUNTRY_CODE: usize = 11;

fn is_valid_area_code(phone_no: &mut Peekable<Chars>) -> bool {
    let first_char_area_code = phone_no.peek().unwrap().to_digit(10);
    matches!(first_char_area_code, Some((2..=9)))
}

fn is_valid_exchange_code(phone_no: Vec<char>) -> bool {
    let first_char_exchange_code = phone_no.get(3).unwrap().to_digit(10);
    matches!(first_char_exchange_code, Some((2..=9)))
}

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
        PHONE_NUMBER_WITHOUT_COUNTRY_CODE => {
            let mut clean_user_number_peekable = clean_user_number.chars().peekable();
            let clean_user_number_vec = clean_user_number.chars().collect::<Vec<char>>();
            match (is_valid_area_code(&mut clean_user_number_peekable), is_valid_exchange_code(clean_user_number_vec)) {
                (true, true) => Some(clean_user_number),
                (_,_) => None
            }
        },
        PHONE_NUMBER_WITH_COUNTRY_CODE => {
            let mut clean_user_number_iter = clean_user_number.chars();
            let country_code = clean_user_number_iter.next().unwrap();
            let mut remaining_user_number_peekable = clean_user_number_iter.clone().peekable();
            match country_code {
                '1' => {
                    match (is_valid_area_code(&mut remaining_user_number_peekable), is_valid_exchange_code(clean_user_number_iter.collect::<Vec<char>>())) {
                        (true, true) => {
                            Some(remaining_user_number_peekable.collect::<String>())
                        },
                        (_,_) => None
                    }
                },
                _ => None
            }
        },
        _ => None
    }
}
