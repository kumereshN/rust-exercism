use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut hm = HashMap::new();

    candidate
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .for_each(|c| {
            let lower_c  = c.to_ascii_lowercase();
            hm
                .entry(lower_c)
                .and_modify(|e| {*e += 1})
                .or_insert(0);
        });

    hm
        .values()
        .all(|&v| v == 1)
}
