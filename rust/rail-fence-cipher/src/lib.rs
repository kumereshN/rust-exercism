use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
pub struct RailFence{
    rails: u32
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(text, true).collect();
        self.fence(graphemes.len())
            .iter()
            .map(|&i| graphemes[i])
            .collect()
        }

    pub fn decode(&self, cipher: &str) -> String {
        let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(cipher, true).collect();
        let width = graphemes.len();
        let indexes = self.fence(width);
        let lookup: HashMap<_, &str> = indexes.iter().zip(graphemes).collect();
        (0..width).map(|i| lookup[&i]).collect()
    }

    fn fence(&self, length: usize) -> Vec<usize> {
        let mut table: Vec<(u32, usize)> = (0..self.rails)
            .chain((1..self.rails - 1).rev())
            .cycle()
            .zip(0..length)
            .collect();
        table.sort_unstable();
        table.iter().map(|(_, c)| *c).collect()
    }
}
