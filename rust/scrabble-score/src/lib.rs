use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let letter_score_hmap: HashMap<Vec<char>, u64> = HashMap::from([
        (vec!['a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't'], 1),
        (vec!['d', 'g'], 2),
        (vec!['b', 'c', 'm', 'p'], 3),
        (vec!['f', 'h', 'v', 'w', 'y'], 4),
        (vec!['k'], 5),
        (vec!['j', 'x'], 8),
        (vec!['q', 'z'], 10)
    ]);

    word
        .to_ascii_lowercase()
        .chars()
        .flat_map(|c| {
            letter_score_hmap
                .iter()
                .map(move |(vec_letter, score)| {
                    if vec_letter.contains(&c) {
                        score
                    }
                    else {
                        &0
                    }
                })
        })
        .sum::<u64>()
}
