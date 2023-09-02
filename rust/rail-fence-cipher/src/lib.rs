pub struct RailFence{
    rails: u32,
    vec_of_chars: Vec<Vec<char>>
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails,
            vec_of_chars: vec![vec!['.']; rails as usize]
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let rails = self.rails;
        let mut vec_of_chars = self.vec_of_chars
            .iter()
            .map(|c| c.repeat(text.len()))
            .collect::<Vec<Vec<char>>>();

        let first_half_iter = 0..=rails-1;
        let second_half_iter = (0..=rails-1).rev().map(|c| c.saturating_sub(1)).take((rails.saturating_sub(2)) as usize);
        let mut combined_iter = first_half_iter.chain(second_half_iter).cycle();

        for (col_idx, c) in text.chars().enumerate() {
            let row_idx = combined_iter.next().unwrap() as usize;
            vec_of_chars[row_idx][col_idx] = c;
        }

        vec_of_chars
            .iter()
            .flat_map(|v| {
                v.iter().filter(|c| c.is_alphanumeric())
            })
            .collect::<String>()
        }

    pub fn decode(&self, cipher: &str) -> String {
        let rails = self.rails;
        let mut vec_of_chars = self.vec_of_chars
            .iter()
            .map(|c| c.repeat(cipher.len()))
            .collect::<Vec<Vec<char>>>();

        let first_half_iter = 0..=rails-1;
        let second_half_iter = (0..=rails-1).rev().map(|c| c.saturating_sub(1)).take((rails.saturating_sub(2)) as usize);
        let mut combined_iter = first_half_iter.chain(second_half_iter).cycle();

        for col_idx in 0..cipher.len() {
            let row_idx = combined_iter.next().unwrap() as usize;
            vec_of_chars[row_idx][col_idx] = '*';
        }

        let mut cipher_idx_range = 0..cipher.len();

        let ziz_zag_vec = vec_of_chars
            .iter()
            .map(|v| {
                v
                    .iter()
                    .map(|&c| {
                    if c == '*' {
                        let cipher_idx = cipher_idx_range.next().unwrap();
                        cipher.chars().nth(cipher_idx).unwrap()
                    } else {
                        '.'
                    }
                })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        let mut res = String::new();

        for col_idx in 0..cipher.len() {
            for row_idx in 0..rails {
                let ch = ziz_zag_vec[row_idx as usize][col_idx];
                if ch != '.' {
                    res.push(ch);
                }
            }
        }

        res
    }
}
