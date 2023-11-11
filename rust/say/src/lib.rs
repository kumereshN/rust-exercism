use std::collections::HashMap;

pub fn encode(n: u64) -> String {
    let len_of_number = n.to_string().len();
    let vec_char = n
        .to_string()
        .chars()
        .collect::<Vec<char>>();

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


    match len_of_number {
        1 => {
            let first_char = vec_char.first().unwrap();
            ones_number_map.get(first_char).unwrap().to_string()
        },
        2 => {
            let concat_n = vec_char.iter().map(|&c|c.to_string()).collect::<Vec<String>>().join("");
            let first_char = concat_n.chars().next().unwrap();
            let last_char = concat_n.chars().last().unwrap();

            match first_char {
                '1' => {
                    tens_number_map.get(concat_n.as_str()).unwrap().to_string()
                },
                '2'..='9' => {
                    if last_char == '0' {
                        twenty_to_ninety_nine_map.get(concat_n.as_str()).unwrap().to_string()
                    } else {
                        concat_n
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
        _ => panic!("Something went wrong")
    }
}
