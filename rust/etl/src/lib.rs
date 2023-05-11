use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res: BTreeMap<char, i32> = BTreeMap::new();

    for (points, vec_char) in h{
        let lower_case_char: Vec<_> = vec_char
            .iter()
            .map(|&c| {
                c.to_ascii_lowercase()
        })
            .collect();

        for char in lower_case_char{
            res.entry(char).or_insert(*points);
        }
    }
    res
}
