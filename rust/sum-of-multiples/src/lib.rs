use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let unique_valid_factors = (0..limit)
        .filter(|&num|
            factors
            .iter()
            .any(|&factor| factor != 0 && num % factor == 0))
        .collect::<HashSet<u32>>();

    unique_valid_factors.iter().sum()
}