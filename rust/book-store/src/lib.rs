use std::collections::{HashMap, HashSet};

// First iteration: Total sum of books * min no of copies
// (you're adding copies not multiplying the sum of books), largest copy's value
// Second iteration: largest copy / min no of copy's value + largest copy's value * largest copy (Same here, you making copies)

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

pub fn lowest_price(books: &[u32]) -> u32 {
    let total_books_in_basket = books.len();
    let books_basket_hashset = books.iter().collect::<HashSet<&u32>>();

    match total_books_in_basket {
        0 => 0,
        1 => BOOK_PRICE,
        _ => {
                let total_price_of_books = (total_books_in_basket as u32 * BOOK_PRICE) as f32;
                let total_count_of_books = books
                    .iter()
                    .fold(HashMap::new(), |mut acc, book| {
                        acc
                            .entry(*book)
                            .and_modify(|counter| *counter += 1u32)
                            .or_insert(1);
                        acc
                    });
                let group_by_total_count_of_books: HashMap<u32, u32> = total_count_of_books
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, books| {
                        acc
                            .entry(books.1)
                            .and_modify(|c| *c += 1u32)
                            .or_insert(1);
                        acc
                    });
                let mut res: Vec<Vec<u32>> = vec![];
                let total_number_of_books = books_basket_hashset.len() as u32;
                let min_no_copy = *group_by_total_count_of_books.iter().min_by_key(|n| n.0).unwrap().0;
                let min_no_copy_value = *group_by_total_count_of_books.iter().min_by_key(|n| n.0).unwrap().1;
                let max_copy = *group_by_total_count_of_books.iter().max_by_key(|n| n.0).unwrap().0;
                let max_copy_value = *group_by_total_count_of_books.iter().max_by_key(|n| n.0).unwrap().1;

                let group_by_total_count_of_books_len = group_by_total_count_of_books.len();

                match group_by_total_count_of_books_len {
                    1 => {
                        match min_no_copy_value {
                            1 => return total_price_of_books as u32,
                            2 => return (BookDiscount::Two.get_discount() * (total_price_of_books)) as u32,
                            3 => return (BookDiscount::Three.get_discount() * (total_price_of_books)) as u32,
                            4 => return (BookDiscount::Four.get_discount() * (total_price_of_books)) as u32,
                            5 => return (BookDiscount::Five.get_discount() * (total_price_of_books)) as u32,
                            _ => panic!("Something went wrong")
                        };
                    },
                    2 => {
                        let mut first_iter = vec![total_number_of_books; min_no_copy as usize];
                        first_iter.push(max_copy_value);
                        res.push(first_iter);

                        let second_iter = vec![(max_copy_value / min_no_copy_value) + max_copy_value; max_copy as usize];
                        res.push(second_iter);
                    },
                    _ => panic!("Something went wrong")
                }
                // If group by group_by_total_count_of_books is only 1, then divide the group evenly
                // If not follow up with first_iter and second_iter
                10
            }
        }
    }
    // todo!("Find the lowest price of the bookbasket with books {books:?}")
