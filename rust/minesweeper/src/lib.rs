use std::str;
const MINE_BYTE: u8 = b'*';
fn count_mines(minefield: &[&str], x: usize, y: usize) -> usize {
    (y.saturating_sub(1)..=y + 1)
        .filter_map(|y| minefield.get(y))
        .flat_map(|line| (x.saturating_sub(1)..=x + 1).filter_map(move |x| line.as_bytes().get(x)))
        .filter(|&&c| c == MINE_BYTE)
        .count()
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = vec![];
    for (i, row) in minefield.iter().enumerate() {
        let mut new_row = String::from("");
        for (j, item) in row.as_bytes().iter().enumerate() {
            if *item == MINE_BYTE {
                new_row.push(*item as char);
            } else {
                let new_num = count_mines(minefield, j, i);
                new_row.push(if (new_num) > 0 {
                    char::from_digit(new_num as u32, 10).unwrap()
                } else {
                    ' '
                })
            }
        }
        res.push(new_row);
    }
    res
}