const BELOW_20: [&str; 20] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen"];
const TENS: [&str; 10] = ["","","twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
pub fn encode(n: u64) -> String {
    match n {
        0..=19 => BELOW_20[n as usize].to_string(),
        20..=99 => {
            let ten = TENS[(n / 10) as usize];
            let rest = n % 10;
            if rest == 0 {
                ten.to_string()
            } else {
                format!("{}-{}", ten, BELOW_20[rest as usize])
            }
        },
        _ => {
            let (power_10, text) = match n {
                100..=999 => (100, "hundred"),
                1_000..=999_999 => (1_000, "thousand"),
                1_000_000..=999_999_999 => (1_000_000, "million"),
                1_000_000_000..=999_999_999_999 => (1_000_000_000, "billion"),
                1_000_000_000_000..=999_999_999_999_999 => (1_000_000_000_000, "trillion"),
                1_000_000_000_000_000..=999_999_999_999_999_999 => (1_000_000_000_000_000, "quadrillion"),
                _ => (1_000_000_000_000_000_000, "quintillion")
            };
            let high = encode(n / power_10);
            let low = encode(n % power_10);
            format!("{} {} {}", high, text, low).replace(" zero", "")
        },
    }
}
