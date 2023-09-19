const BOOK_PRICE: u32 = 800;

enum Discount {
    DiscountOfTwo,
    DiscountOfThree,
    DiscountOfFour,
    DiscountOfFive
}
pub fn lowest_price(books: &[u32]) -> u32 {
    match books.len() {
        0 => 0,
        1 => BOOK_PRICE,
        _ => books.len() as u32 * BOOK_PRICE
    }
    // todo!("Find the lowest price of the bookbasket with books {books:?}")
}
