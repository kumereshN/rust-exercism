const ONE_STR: &str = "1";
pub fn egg_count(display_value: u32) -> usize {
    let binary_string = format!("{:b}", display_value);
    binary_string
        .matches(ONE_STR)
        .count()
    // todo!("count the eggs in {display_value}")
}
