use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const BOOK_PRICE: u32 = 800;

enum BookDiscount {
    Two,
    Three,
    Four,
    Five
}

impl BookDiscount {
    pub fn get_discount(self) -> f32 {
        match self {
            BookDiscount::Two => 0.95,
            BookDiscount::Three => 0.90,
            BookDiscount::Four => 0.80,
            BookDiscount::Five => 0.75
        }
    }
}

fn are_books_unique_in_basket(books: &[u32], unique_books: HashSet<&u32>) -> bool {
    unique_books.len() == books.len()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let total_books_in_basket = books.len();
    let books_basket_hashset = books.iter().collect::<HashSet<&u32>>();

    match total_books_in_basket {
        0 => 0,
        1 => BOOK_PRICE,
        _ => {
            let total_price_of_books = (total_books_in_basket as u32 * BOOK_PRICE) as f32;
            if are_books_unique_in_basket(books, books_basket_hashset) {
                match total_books_in_basket {
                    2 => (BookDiscount::Two.get_discount() * (total_price_of_books)) as u32,
                    3 => (BookDiscount::Three.get_discount() * (total_price_of_books)) as u32,
                    4 => (BookDiscount::Four.get_discount() * (total_price_of_books)) as u32,
                    5 => (BookDiscount::Five.get_discount() * (total_price_of_books)) as u32,
                    _ => panic!("Unique books are greater than 5")

                }

            } else {
                let total_count_of_books = books
                    .iter()
                    .fold(HashMap::new(), |mut acc, book| {
                        acc
                            .entry(*book)
                            .and_modify(|counter| *counter += 1u32)
                            .or_insert(1);
                        acc
                    });
                let group_by_total_count_of_books: HashMap<u32, Vec<u32>> = total_count_of_books
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, books| {
                        acc
                            .entry(books.1)
                            .and_modify(|v| v.push(books.0))
                            .or_insert(vec![books.0]);
                        acc
                    });
                10
            }

        }
    }
    // todo!("Find the lowest price of the bookbasket with books {books:?}")
}
