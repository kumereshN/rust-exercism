pub struct Matrix {
    data: Vec<Vec<u32>>
    // Implement your Matrix struct
}

impl Matrix {
    const PARSE_ERROR: &'static str = "Unable to parse character";
    pub fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|c| c
                .split_whitespace()
                .map(|n| {
                    n.parse::<u32>().expect(Self::PARSE_ERROR)
                })
                .collect::<Vec<u32>>()
            )
            .collect::<Vec<Vec<u32>>>();
        
        Self {
            data
        }

        // todo!("Create new method to store the {input}")
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.data
            .get(row_no-1)
            .cloned()
        // todo!("Return the row at {row_no} (1-indexed) or None if the number is invalid")
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.data
            .iter()
            .map(|row| row.get(col_no-1).cloned())
            .collect()
        // todo!("Return the column at {col_no} (1-indexed) or None if the number is invalid")
    }
}
