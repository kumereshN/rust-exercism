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

fn has_punctuation(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_punctuation())
}

pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();

    if phrase.is_empty() {
        "".to_string();
    } else {
        phrase
            .split_ascii_whitespace()
            .for_each(|s| {
                match (is_all_uppercase(s), has_punctuation(s)) {
                    (true, true) => s
                        .chars()
                        .filter(|&c| c.is_alphabetic())
                        .for_each(|c| acronym.push(c)),
                    (true, false) => acronym.push(s.chars().next().unwrap()),
                    (false,true) => s
                        .split(|c: char| c.is_ascii_punctuation())
                        .filter(|s| !s.is_empty() && s.len() > 1)
                        .for_each(|c| {
                            let first_char = capitalize_first_char(c).chars().next().unwrap();
                            acronym.push(first_char)
                        }),
                    (_,_) => {
                        let s = capitalize_first_char(s);
                        s
                            .chars()
                            .filter(|&c| c.is_uppercase())
                            .for_each(|c| acronym.push(c))
                    }

                }
            })
    }
    acronym
}
