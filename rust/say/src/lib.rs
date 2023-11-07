use std::collections::HashMap;

pub struct numbers {
    one: String
}


pub fn encode(n: u64) -> String {
    let len_of_number = n.to_string().len();
    let n_string = n.to_string();

    let ones_number_map: HashMap<&str, &str> = HashMap::from([
        ("0", "zero"),
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
    ]);

    match len_of_number{
        1 => { ones_number_map.get(n_string.as_str()).unwrap().to_string()},
        _ => panic!("Something went wrong")
    }
}
