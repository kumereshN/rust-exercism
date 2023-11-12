use std::collections::HashMap;

pub fn double_digits_to_word(n_string: String,
               ones_number_map: &HashMap<char, &str>,
               tens_number_map: &HashMap<&str, &str>,
               twenty_to_ninety_nine_map: &HashMap<&str, &str>) -> String {

    let first_char = n_string.chars().next().unwrap();
    let last_char = n_string.chars().last().unwrap();

    match first_char {
        '1' => {
            tens_number_map.get(n_string.as_str()).unwrap().to_string()
        },
        '2'..='9' => {
            if last_char == '0' {
                twenty_to_ninety_nine_map.get(n_string.as_str()).unwrap().to_string()
            } else {
                n_string
                    .chars()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i,c) | {
                        if i == 0 {
                            let first_str = *twenty_to_ninety_nine_map.get(format!("{}0",c).as_str()).unwrap();
                            acc.push_str(format!("{}-", first_str).as_str())
                        } else {
                            let last_str = *ones_number_map.get(&c).unwrap();
                            acc.push_str(last_str)
                        }
                        acc
                    })
            }
        },
        _ => {
            panic!("Error")
        }
    }
}

pub fn len_to_magnitude<'a>(n: usize) -> &'a str {
    match n {
        3 => "hundred",
        4..=6 => "thousand",
        7..=9 => "million",
        10..=12 => "billion",
        _ => panic!("error")
    }
}

pub fn encode(n: u64) -> String {

    let ones_number_map: HashMap<char, &str> = HashMap::from([
        ('0', "zero"),
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
    ]);

    let tens_number_map: HashMap<&str, &str> = HashMap::from([
        ("10", "ten"),
        ("11", "eleven"),
        ("12", "twelve"),
        ("13", "thirteen"),
        ("14", "fourteen"),
        ("15", "fifteen"),
        ("16", "sixteen"),
        ("17", "seventeen"),
        ("18", "eighteen"),
        ("19", "nineteen"),
    ]);

    let twenty_to_ninety_nine_map: HashMap<&str, &str> = HashMap::from([
        ("20", "twenty"),
        ("30", "thirty"),
        ("40", "forty"),
        ("50", "fifty"),
        ("60", "sixty"),
        ("70", "seventy"),
        ("80", "eighty"),
        ("90", "ninety")
    ]);


    let n_string = n.to_string();
    let first_char = n_string.chars().next().unwrap();
    let first_digit_to_word = ones_number_map.get(&first_char).unwrap();

    let len_of_n_string = n_string.len();


    match len_of_n_string {
        1 => {
            ones_number_map.get(&n_string.chars().next().unwrap()).unwrap().to_string()
        },
        2 => {
            double_digits_to_word(n_string, &ones_number_map, &tens_number_map, &twenty_to_ninety_nine_map)
        },
        6 | 9 | 12 => {
            n_string
                .chars()
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|c| c.iter().collect::<String>())
                .fold(String::new(), |mut acc, c| {
                    let digit_to_word = encode(c.parse::<u64>().unwrap());
                    let magnitude = len_to_magnitude(len_of_n_string);
                    let res = format!("{} {}", digit_to_word, magnitude);
                    acc.push_str(res.as_str());
                    acc
                })
        },
        // For digits greater than 6 and modulo 3 yields 0, have to do a different method
        3..=12 => {
            let magnitude = len_to_magnitude(len_of_n_string);
            let remaining_digits = &n_string[1..].parse::<u64>().unwrap();
            if remaining_digits > &0 {
                format!("{} {} {}",
                        first_digit_to_word,
                        magnitude,
                        encode(*remaining_digits)
                )
            } else {
                format!("{} {}",
                        first_digit_to_word,
                        magnitude
                )
            }
        },
        _ => {
            panic!("Something went wrong")
        }
    }
}
