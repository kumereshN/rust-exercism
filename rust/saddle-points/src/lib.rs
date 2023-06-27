pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    for i in 0..input.len() {
        let row_max = input[i].iter().max().unwrap_or(&0);
        for (j, &val) in input[i].iter().enumerate() {
            if val == *row_max {
                let col_min = (0..input.len())
                    .map(|x| input[x][j])
                    .min()
                    .unwrap();
                if val == col_min {
                    saddle_points.push((i, j));
                }
            }
        }
    }
    saddle_points
}
