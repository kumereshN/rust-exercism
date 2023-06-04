use std::collections::HashSet;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
const ADD_AY: &str = "ay";

fn is_vowel(input: &str, first_char: char) -> bool {
    input.starts_with("xr")  || input.starts_with("yt") || VOWELS.contains(&first_char)
}

fn is_consonant(input_char: char) -> bool {
    let consonants = ('a'..='z').filter(|c| c != &'y' && !VOWELS.contains(c)).collect::<HashSet<char>>();
    consonants.contains(&input_char)
}

fn convert_str_to_pig_latin_str(input: &str) -> String {

    let first_char = input.chars().take(1).next().unwrap();

    match is_vowel(input, first_char) {
        true => format!("{input}{ADD_AY}"),
        false => {
            let find_qu_vec = input.split_inclusive("qu").collect::<Vec<_>>();
            let find_qu_substr = find_qu_vec[0];
            match find_qu_substr.contains("qu") {
                true => {
                    let remaining_chars = find_qu_vec[1];
                    format!("{remaining_chars}{find_qu_substr}{ADD_AY}")
                },
                false => {
                    if first_char == 'y'{
                        let remaining_chars = input.chars().skip(1).collect::<String>();
                        return format!("{remaining_chars}y{ADD_AY}")
                    }

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

pub fn translate(input: &str) -> String {
    if input.contains(' '){
        input
            .split_ascii_whitespace()
            .map(|w| {
                convert_str_to_pig_latin_str(w)
            })
            .collect::<Vec<String>>()
            .join(" ")
    } else {
        convert_str_to_pig_latin_str(input)
    }

}
