use std::iter;

pub struct PascalsTriangle{
    triangle: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        match row_count {
            0 => Self{
                triangle: vec![]
            },
            1 => Self{
                triangle: vec![vec![1]]
            },
            2 => Self{
                triangle: vec![vec![1], vec![1, 1]]
            },
            3.. => {
                let mut starting_triangle = Self {
                    triangle: vec![vec![1], vec![1, 1]]
                };
                let remaining_count = row_count - 2;

                for _ in 0..remaining_count{
                    let last_vec = starting_triangle.triangle.last().unwrap();
                    let total_sum = last_vec
                        .windows(2)
                        .map(|n| {
                            n.iter().sum::<u32>()
                        })
                        .collect::<Vec<_>>();

                    let res = iter::once(&1)
                        .chain(
                            total_sum
                                .iter()
                        )
                        .chain(
                            iter::once(&1)
                        )
                        .copied()
                        .collect::<Vec<_>>();

                    starting_triangle.triangle.push(res);
                }

                starting_triangle
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
