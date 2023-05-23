use std::collections::HashMap;
const NUCLEOTIDES: &[char;4]= &['A', 'C', 'G', 'T'];



pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts = nucleotide_counts(dna)?;
    counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = NUCLEOTIDES.iter().map(|&n| (n, 0)).collect();

    for c in dna.chars() {
        map.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }
    Ok(map)
}
