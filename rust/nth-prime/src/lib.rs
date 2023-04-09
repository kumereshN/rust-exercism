pub fn is_prime(n: u32) -> bool {
    let sqrt = (n as f32).sqrt() as u32;
    (2..=sqrt)
        .all(|num| n % num != 0)
}

pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|&n| is_prime(n))
        .nth(n as usize)
        .unwrap_or(0)
}
