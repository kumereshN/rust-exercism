use std::collections::{HashMap, HashSet};
const NUCLEOTIDES: &[char;4] = &['A', 'C', 'G', 'T'];

fn find_invalid_char(slice: &str) -> Option<char> {
    slice.chars().find(|c| !NUCLEOTIDES.contains(c))
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    dna.matches()
    match (NUCLEOTIDES.contains(&nucleotide),
           (find_invalid_char(dna))
    ) {
        (true, None) => {
            Ok(dna
                .chars()
                .filter(|&c| c == nucleotide)
                .count()
            )
        }
        (false, _) => {
            Err(nucleotide)
        }
        (_, Some(c)) => {
            Err(c)
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res: HashMap<char, usize> = NUCLEOTIDES.iter().map(|&c| (c,0)).collect();
    let unique_chars = dna.chars().collect::<HashSet<_>>();

    for c in unique_chars.iter() {
        match count(*c, dna) {
            Ok(total_count) => {
                res
                    .entry(*c)
                    .and_modify(|e| {*e += total_count})
                    .or_insert(0);
            },
            Err(c) => {
                return Err(c)
            }
        }
    }
    Ok(res)
}
