#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const VALID_DNA_CHARS: [char; 4] = ['G', 'C', 'T', 'A'];
const VALID_RNA_CHARS: [char; 4] = ['C', 'G', 'A', 'U'];

fn validate(s: &str, chars: [char; 4]) -> Result<String, usize> {
    match s.chars().position(|c| !chars.contains(&c)) {
        Some(x) => Err(x),
        None => Ok(s.to_string())
    }
}

fn transcribe(nucleotide: char) -> char {
    VALID_RNA_CHARS[VALID_DNA_CHARS.iter().position(|&c| c == nucleotide).unwrap()]
}

impl  Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate(dna, VALID_DNA_CHARS).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.chars().map(transcribe).collect())
    }
}

impl  Rna {
    pub fn new(rna: & str) -> Result<Rna, usize> {
        validate(rna, VALID_RNA_CHARS).map(Rna)
    }
}
