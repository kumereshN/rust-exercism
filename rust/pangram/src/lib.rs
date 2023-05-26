use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let all = ('a'..='z').collect::<HashSet<char>>();
    let used = sentence.to_lowercase().chars().collect::<HashSet<char>>();
    all.is_subset(&used)
}
