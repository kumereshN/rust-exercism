use std::collections::HashSet;
// https://www.geeksforgeeks.org/pythagorean-triplet-given-sum/
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    /*let mut res: HashSet<[u32; 3]> = HashSet::new();
    for i in 1..=sum/3 {
        for j in i + 1..=sum/2 {
            let k = sum - i - j;
            if i * i + j * j == k * k {
                res.insert([i,j,k]);
            }
        }
    }
    res*/

    (1..=sum/3)
        .flat_map(|i| {
            (i+1..=sum/2)
                .filter_map(|j| {
                    let k = sum - i - j;
                    if i.pow(2) + j.pow(2) == k.pow(2) {
                        Some([i,j,k])
                    } else {
                        None
                    }
                })
                .collect::<HashSet<[u32;3]>>()
        })
        .collect::<HashSet<[u32;3]>>()
}
