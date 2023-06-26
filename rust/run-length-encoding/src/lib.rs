fn push_char(count: u8, ch: char, mut res: String) -> String {
    match count {
        1 => {
            res.push(ch);
        }
        _ => {
            res.push_str(count.to_string().as_str());
            res.push(ch);
        }
    }
    res
}

pub fn encode(source: &str) -> String {
    // https://www.pythonpool.com/run-length-encoding-python/

    let mut res = String::new();

    if source.is_empty() {
        return res
    }

    let mut ch = source.chars().next().unwrap();
    let remaining_chars = source.chars().skip(1);
    let mut count: u8 = 1;

    for c in remaining_chars {
        if ch == c {
            count += 1;
        }
        else {
            res = push_char(count, ch, res);
            ch = c;
            count = 1;
        }
    }
    res = push_char(count, ch, res);
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
