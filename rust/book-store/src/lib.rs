use std::collections::HashMap;

const PRICES: &[u32] = &[800, 1520, 2160, 2560, 3000];

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    let mut piles: Vec<u32> = books.iter().fold(HashMap::<u32, u32>::new(), |mut h, i| {
        *h.entry(*i).or_insert(0) += 1;
        h
    }).values().cloned().collect();

    piles.sort_by(|a, b| b.cmp(a));
    for i in 0..(piles.len() - 1) {
        piles[i] -= piles[i + 1];
    }
    if piles.len() >= 5 {
        // Get the minimum pile of book between 3rd most common and 5th most common book
        let n = piles[2].min(piles[4]);
        // Remove the minimum pile from both 3rd most common and 5th most common book
        piles[2] -= n;
        piles[4] -= n;
        // Add the removed piles to the 4th most common pile to form as many sets of 5 books as possible
        piles[3] += 2 * n;
    }
    piles.iter().zip(PRICES.iter()).fold(0, |sum, (n, p)| sum + n * p)
}