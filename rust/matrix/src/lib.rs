pub struct Matrix {
    rows: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>
    // Implement your Matrix struct
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|c| c
                .split_whitespace()
                .map(|n| {
                    n.parse::<u32>().unwrap()
                })
                .collect::<Vec<u32>>()
            )
            .collect::<Vec<Vec<u32>>>();
        
        let len_of_matrix = rows.first().unwrap().len();
        let columns = (0..len_of_matrix)
            .map(|i| {
                rows
                    .iter()
                    .map(|v| {
                        *v.get(i).unwrap()
                    })
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<_>>();
        
        Self {
            rows,
            columns
        }

        // todo!("Create new method to store the {input}")
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no-1).cloned()
        // todo!("Return the row at {row_no} (1-indexed) or None if the number is invalid")
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.columns.get(col_no-1).cloned()
        // todo!("Return the column at {col_no} (1-indexed) or None if the number is invalid")
    }
}
