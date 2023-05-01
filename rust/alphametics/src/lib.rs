use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Get all the unique alphabetic chars from input
    let charset: HashSet<char> = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();

    let (alphametic, not_zero) = parse(input);

    // Get all possible permutations (0 to 9) based on charset
    let perms = (0..=9).permutations(charset.len());

    for perm in perms {
        // Map each char to each permutation pair
        let solution: HashMap<char, u8> = charset.iter().cloned().zip(perm.into_iter()).collect();
        if check(&alphametic, &not_zero, &solution) {
            return Some(solution);
        }
    }
    None
}

fn parse(input: &str) -> (Vec<String>, HashSet<char>) {
    let split_addends_sum: Vec<&str> = input.split("==").collect();
    let mut split: Vec<&str> = split_addends_sum.first().unwrap().split('+').collect();
    split.push(split_addends_sum.get(1).unwrap());
    let parsed: Vec<String> = split.iter().map(|s| s.trim().to_string()).collect();
    // Gets the first char in each string to check if it's a 0 in their numeric format, since leading zeros are not allowed
    let not_zero: HashSet<char> = parsed.iter().map(|s| s.chars().next().unwrap()).collect();
    (parsed, not_zero)
}

fn check(alphametic: &[String], not_zero: &HashSet<char>, solution: &HashMap<char, u8>) -> bool {
    if not_zero.iter().any(|c| solution.get(c).unwrap() == &0) {
        return false;
    }
    let sum: Vec<u64> = alphametic.iter().map(|s| to_number(s, solution)).collect();
    // Example: The sum of I + BB in their numeric form compared against ILL in numeric form
    sum[0..sum.len() - 1].iter().sum::<u64>() == sum[sum.len() - 1]
}

fn to_number(string: &str, solution: &HashMap<char, u8>) -> u64 {
    let num_str: String = string
        .chars()
        .map(|c| solution.get(&c).unwrap().to_string())
        .collect();
    num_str
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Something happened while parsing: {}", num_str))
}