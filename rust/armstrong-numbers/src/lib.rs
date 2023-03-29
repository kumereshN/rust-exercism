pub fn is_armstrong_number(num: u32) -> bool {
    let len_num = num.to_string().len() as u32;
    let total_sum = num
        .to_string()
        .chars()
        .map(|w| {
        w.to_digit(10).unwrap().pow(len_num) as u64
    })
        .sum::<u64>();
    num as u64 == total_sum
}