use std::collections::HashSet;
use std::ops::Div;
use std::process::Output;

const BOOK_PRICE: u32 = 800;

enum BookDiscount {
    DiscountOfTwo = 95,
    DiscountOfThree = 90,
    DiscountOfFour = 80,
    DiscountOfFive = 75
}

impl Div for BookDiscount {
    type Output = f32;

    fn div(self, rhs: BookDiscount) -> Self::Output {
        (self / rhs) as f32
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let total_books_in_basket = books.len();

    match total_books_in_basket {
        0 => 0,
        1 => BOOK_PRICE,
        _ => {
            let books_basket_hashset = books.iter().collect::<HashSet<&u32>>();
            let total_price_of_books = (total_books_in_basket as u32 * BOOK_PRICE) as f32;
            if books_basket_hashset.len() == total_books_in_basket {
                match total_books_in_basket {
                    2 => ((BookDiscount::DiscountOfTwo.div(100)) * (total_price_of_books)) as u32
                }

            } else {
                0
            }

        }
    }
    // todo!("Find the lowest price of the bookbasket with books {books:?}")
}
