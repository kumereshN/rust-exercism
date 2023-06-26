pub fn encode(source: &str) -> String {
    let mut res = String::new();
    let mut remainder = source;

    while let Some(c) = remainder.chars().next() {
        let count = remainder.chars().take_while(|&next| next == c).count();

        match count {
            1 => res.push(c),
            _ => res.push_str(&format!("{}{}", count, c)),
        }

        remainder = &remainder[count * c.len_utf8()..]
        }
    res
    }

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let iter = source.chars();
    let mut count = String::new();

    for ch in iter {
        if ch.is_ascii_digit() {
            count.push(ch);
        } else {
            let total_count = count.parse::<usize>().unwrap_or(1);
            let repeated_ch = std::iter::repeat(ch).take(total_count).collect::<String>();
            res.push_str(&repeated_ch);
            count = String::new();
        }
    }
    res
}
