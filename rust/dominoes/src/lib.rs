use std::cmp;
use std::cmp::Ordering;
use std::collections::HashSet;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    // todo!("From the given input '{input:?}' construct a proper dominoes chain or return None if it is not possible.");
    let mut cloned_input = input.to_vec();
    if input.is_empty() {
        return Some(cloned_input)
    }
    let n_input = input.len();
    match n_input {
        1 => {
            if input.iter().all(|(i1, i2)| i1 == i2) {
                Some(cloned_input)
            } else {
                None
            }
        },
        _ => {
            // Some sort of sorting needs to be done to match the current last number and the next current number
            // Look at the Joplin notes
            cloned_input.sort_by(|&(i1,i2), (j1, _)| {
                let cmp = i2.cmp(j1);
                if cmp == std::cmp::Ordering::Equal {
                    i1.cmp(j1)
                } else {
                    cmp.then_with(|| Ordering::Less)
                }
            });
            Some(cloned_input)
        }
    }
}
