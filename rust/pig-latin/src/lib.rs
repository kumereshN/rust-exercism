use std::collections::HashSet;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
const ADD_AY: &str = "ay";

fn is_vowel(input: &str) -> bool {
    let first_two_chars = input.chars().take(2).collect::<String>();
    let first_char = input.chars().take(1).next().unwrap();

    first_two_chars == "xr" || first_two_chars == "yt" || VOWELS.contains(&first_char)
}

fn is_consonant(input_char: char) -> bool {
    let mut consonants = ('a'..='z').filter(|c| !VOWELS.contains(c)).collect::<HashSet<char>>();
    consonants.remove(&'y');
    consonants.contains(&input_char)
}

pub fn translate(input: &str) -> String {

    match is_vowel(input) {
        true => format!("{input}{ADD_AY}"),
        false => {
            let find_qu_vec = input.split_inclusive("qu").collect::<Vec<_>>();
            let find_qu = find_qu_vec[0];
            match find_qu.contains("qu") {
                true => {
                    let remaining_chars = find_qu_vec[1];
                    format!("{remaining_chars}{find_qu}{ADD_AY}")
                },
                false => {
                    let consonant_clusters = input
                        .chars()
                        .take_while(|&c| { is_consonant(c) })
                        .collect::<String>();

                    let remaining_chars = input
                        .chars()
                        .skip_while(|&c| consonant_clusters.contains(c))
                        .collect::<String>();

                    format!("{remaining_chars}{consonant_clusters}{ADD_AY}")
                }
            }
        }
    }
}
