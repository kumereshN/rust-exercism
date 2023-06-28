pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    for (r, row) in input.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if row.iter().all(|x| x <= val) && input.iter().all(|x| x[c] >= *val) {
                res.push((r,c));
            }
        }
    }
    res
}
