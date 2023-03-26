/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0;count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fib: Vec<u8> = vec![];

    for n in 0..=4 {
        match n {
            0..=1 => fib.push(1),
            _ => fib.push(fib.iter().rev().take(2).sum())
        }
    }
    fib


}
