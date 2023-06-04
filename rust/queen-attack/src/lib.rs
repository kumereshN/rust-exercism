use std::cmp::Ordering;

#[derive(Debug)]
pub struct ChessPosition{
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            ((0..=7), (0..=7)) => Some(
                Self {
                    rank,
                    file
                }
            ),
            _ => None
        }
    }

}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    fn add_pos(&self) -> i32 {
        self.0.rank + self.0.file
    }

    fn subtract_pos(&self) -> i32 {
        self.0.rank - self.0.file
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (self_rank, self_file) = (&self.0.rank, &self.0.file);
        let (other_rank, other_file) = (other.0.rank, other.0.file);

        match (self_rank.cmp(&other_rank), self_file.cmp(&other_file)) {
            (Ordering::Equal, _) => true,
            (_, Ordering::Equal) => true,
            _ => {
                let self_sum = self.add_pos();
                let self_subtract = self.subtract_pos();

                let other_sum = other.add_pos();
                let other_subtract = other.subtract_pos();

                match (self_sum.cmp(&other_sum), self_subtract.cmp(&other_subtract))  {
                    (Ordering::Equal, _) => true,
                    (_, Ordering::Equal) => true,
                    (_, _) => false
                }
            }
        }
    }
}
