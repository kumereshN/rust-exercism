/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.chars().count() != s2.chars().count() {
        None
    } else {
        s1
            .chars()
            .zip(
                s2
                    .chars()
            )
            .map(|t| {
                if t.0 != t.1 {
                    Some(1)
                } else {
                    Some(0)
                }
            })
            .sum()
    }
}
