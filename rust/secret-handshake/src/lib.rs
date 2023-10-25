pub fn actions(n: u8) -> Vec<&'static str> {
    let mut res: Vec<&'static str> = vec![];
    let binary_number = format!("{:b}", 31);
    match binary_number.as_str() {
        "1" => res.push("wink"),
        _ => panic!("Something went wrong")
    }
    (0..8)
        .rev()
        .map(|i| {
            if (n & (1 << i)) != 0 {

            }
        })
    res
}
