use std::collections::HashSet;
// https://www.geeksforgeeks.org/pythagorean-triplet-given-sum/
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res: HashSet<[u32; 3]> = HashSet::new();
    for i in 1..=sum/3 {
        for j in i + 1..=sum/2 {
            let k = sum - i - j;
            if i * i + j * j == k * k {
                res.insert([i,j,k]);
            }
        }
    }
    res
    // todo!("Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c");
}
