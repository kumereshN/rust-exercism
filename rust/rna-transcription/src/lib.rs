use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const VALID_DNA_CHARS: &[char] = &['A', 'C', 'G', 'T'];
const VALID_RNA_CHARS: &[char] = &['A', 'C', 'G', 'U'];


impl  Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, ch) in dna.chars().enumerate() {
            if !VALID_DNA_CHARS.contains(&ch) {
                return Err(i)
            }
        }
        Ok(Dna(dna.parse().unwrap()))
    }

    pub fn into_rna(self) -> Rna {
        let map_dna_to_rna: HashMap<char, char> = HashMap::from([
            ('G', 'C'),
            ('C', 'G'),
            ('T', 'A'),
            ('A', 'U')
        ]);

        let rna_str = self
            .0
            .chars()
            .map(|c| {
                map_dna_to_rna.get(&c).unwrap()
            })
            .collect::<String>();

        Rna(rna_str)
    }
}

impl  Rna {
    pub fn new(rna: & str) -> Result<Rna, usize> {
        for (i, ch) in rna.chars().enumerate() {
            if !VALID_RNA_CHARS.contains(&ch) {
                return Err(i)
            }
        }
        Ok(Rna(rna.parse().unwrap()))
    }
}
