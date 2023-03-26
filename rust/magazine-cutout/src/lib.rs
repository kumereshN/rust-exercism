// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = magazine.iter().fold(HashMap::new(), |mut map, str| {
        *map.entry(str).or_insert(0) += 1;
        map
    });

    let notes_map = note.iter().fold(HashMap::new(), |mut map, str| {
        *map.entry(str).or_insert(0) += 1;
        map
    });

    notes_map.iter()
             .all(|(word, count)| {magazine_map.get(word).unwrap_or(&0) >= count})
}
