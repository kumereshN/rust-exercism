use std::collections::HashMap;

const LARGEST_ALPHABET_INDEX: i8 = 25;

fn get_transpose_char(c: char, key: i8) -> char {
    let alphabets_hmap: HashMap<char, usize> = ('a'..='z').enumerate().map(|(i, c)| (c, i)).collect();
    let char_idx = *alphabets_hmap.get(&c).unwrap() as i8;
    let mut transpose_idx = char_idx + key;

    if transpose_idx > LARGEST_ALPHABET_INDEX {
        transpose_idx = transpose_idx % LARGEST_ALPHABET_INDEX - 1;
    }
    if transpose_idx.is_negative() {
        transpose_idx = transpose_idx.rem_euclid(LARGEST_ALPHABET_INDEX) + 1;
    }


    *alphabets_hmap
        .iter()
        .find(|(&_c, &i)| {
            i == transpose_idx as usize
        })
        .unwrap()
        .0
}

pub fn rotate(input: &str, key: i8) -> String {

    input
        .chars()
        .map(|c| {
            match (c.is_alphabetic(), c.is_uppercase()) {
                (true, true) => {
                    get_transpose_char(c.to_ascii_lowercase(), key).to_ascii_uppercase()
                },
                (true, false) => {
                    get_transpose_char(c, key)
                }
                (_, _) => {
                    c
                }
            }

        })
        .collect::<String>()
    }
