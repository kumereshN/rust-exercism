use std::str;

pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    grouped_pairs: Vec<(&'a str, &'a str )>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.grouped_pairs
            .iter()
            .find_map(|(c, n)| {
                if *c == codon {
                    Some(*n)
                } else {
                    None
                }
            })
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        const CODON_LENGTH: usize = 3;
        const STOP_MARK: &str = "stop codon";

        rna
            .as_bytes()
            .chunks(CODON_LENGTH)
            .map(str::from_utf8)
            .map_while(|c| {
                let codon_name = self.name_for(c.unwrap());

                match codon_name == Option::from(STOP_MARK) {
                    true => None,
                    false => Some(codon_name)
                }
            })
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        grouped_pairs: pairs
    }
}
