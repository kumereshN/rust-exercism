pub fn actions(n: u8) -> Vec<&'static str> {
    let mut res = (0..8)
        .rev()
        .filter_map(|i| {
            let bit_wise_ops = n & (1 << i);
            if bit_wise_ops != 0 {
                match i {
                    0 => Some("wink"),
                    1 => Some("double blink"),
                    2 => Some("close your eyes"),
                    3 => Some("jump"),
                    4 => Some("reverse"),
                    _ => None
                }
            } else {
                None
            }
        })
        .rev()
        .collect::<Vec<&'static str>>();

    if res.contains(&"reverse"){
        res.pop();
        res.into_iter().rev().collect::<Vec<&'static str>>()
    } else {
        res
    }
}
