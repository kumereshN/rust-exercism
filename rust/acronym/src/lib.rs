fn capitalize_first_char(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let capitalized = first.to_uppercase();
            let rest: String = chars.collect();
            format!("{}{}", capitalized, rest)
        }
    }
}

fn is_all_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn has_puntuation(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic())
}

pub fn abbreviate(phrase: &str) -> String {
    let mut accronym = String::new();

    if phrase.is_empty() {
        "".to_string();
    } else {
        phrase
            .split_ascii_whitespace()
            .for_each(|s| {
                if s.chars().all(|c| c.is_uppercase()) {
                    if s.chars().any(|c| c.is_alphabetic()){

                    } else {
                        let s: char = s.chars().next().unwrap();
                        accronym.push(s)
                    }

                } else {
                    let s = capitalize_first_char(s);
                    s
                        .chars()
                        .filter(|&c| c.is_uppercase())
                        .for_each(|c| accronym.push(c))
                }
            })
    }
    accronym
}
