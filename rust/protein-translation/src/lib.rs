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
        rna
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map_while(|c| {
                let codon = c.iter().collect::<String>();
                let codon_name = self.name_for(codon.as_str());

                match codon_name == Option::from("stop codon") {
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
