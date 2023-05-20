/// Determines whether the supplied string is a valid ISBN number
const NUMERIC_X: usize = 10;
const LEN_ISBN: usize = 10;

pub fn is_valid_isbn(isbn: &str) -> bool {
    let remove_dash_isbn: String = isbn
        .chars()
        .filter(|&c| c.is_ascii_digit() || c == 'X')
        .collect();

    if remove_dash_isbn.is_empty() {
        return false
    }

    let check_for_index_of_x = remove_dash_isbn
        .find('X')
        .unwrap_or(remove_dash_isbn.len()-1);

    if remove_dash_isbn.len() == LEN_ISBN && check_for_index_of_x == remove_dash_isbn.len()-1{
            let total_sum = remove_dash_isbn
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    match c.to_digit(10) {
                        Some(digit) => {
                            (i + 1) * digit as usize
                        },
                        None => {
                            (i + 1) * NUMERIC_X
                        }
                    }
                })
                .sum::<usize>();
            total_sum % 11 == 0
        }
        else {
            false
        }
    }
