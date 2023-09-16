use std::cmp::max;
use std::collections::HashSet;
use std::ops::Div;
// https://www.geeksforgeeks.org/pythagorean-triplet-given-sum/
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res: HashSet<[u32; 3]> = HashSet::new();
    let min_range = (sum as f32 / 2f32).sqrt().ceil() as u32;
    let max_range = ((sum as f32).sqrt() + 1f32).floor() as u32;

    for m in min_range..max_range {
        let a = (sum as i32 - sum.pow(2) as i32) as f32 / (2 * m.pow(2)) as f32;
        if a.fract() == 0.0 {
            let b = sum - m.pow(2);
            let c = sum - a as u32 - b;
            res.insert([a as u32, b, c]);
        }
    }
    res
    // todo!("Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c");
}
