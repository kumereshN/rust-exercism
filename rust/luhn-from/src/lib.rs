pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.0.len() < 2 {
            return false
        }

        let vec_of_nums = self
            .0
            .chars()
            .rev()
            .filter(|c| c.is_alphanumeric())
            .enumerate()
            .map(|(i, c)| {
                if c.is_alphabetic() {
                    return None
                }
                let digit = c.to_digit(10).unwrap();
                match i % 2 == 0 {
                    true => {
                        Some(digit)
                    },
                    false => {
                        let multiple = digit * 2;
                        if multiple > 9 {
                            Some(multiple.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
                        } else {
                            Some(multiple)
                        }
                    }
                }
            })
            .collect::<Vec<Option<u32>>>();

        if vec_of_nums.iter().all(|n| n.is_some()) {
            vec_of_nums.iter().map(|c| c.unwrap()).sum::<u32>() % 10 == 0
        } else {
            false
        }
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
