

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    /*
    This code defines a function called to_bytes that takes a slice of u32 values as input and returns a vector of u8 values.

The function starts by creating an empty vector called res to store the resulting bytes.

Next, it iterates over the input values in reverse order using the iter().rev() method. For each value ``, it performs the following steps:

It pushes the least significant 7 bits of num (obtained by performing a bitwise AND with 0x7F) onto the res vector using the push method. This ensures that only the lowest 7 bits are retained, discarding any higher bits.

It then right-shifts num by 7 bitsnum >> 7) and assigns the result back to n. This effectively removes the 7 least significant bits from num`.

It enters a while loop that continues as long as n is not zero. Inside the loop, it performs the following steps:

It pushes the least significant 7 bits of n (obtained by performing a bitwise AND with 0x7F) onto the res vector, but with the most significant bit set 1 (| 0x80). This sets the highest bit of each byte to indicate that more bytes will follow.

It right-shifts n by 7 bits (n >>= 7) to remove the 7 least significant bits and continue processing the remaining bits.

After iterating over all the input values, the resulting bytes in the res vector are in reverse order. To correct this, the code calls the reverse method on the res vector to reverse the order of the bytes.

Finally, the res vector is returned as the result of the function.

In summary, this code converts a slice of u32 values into a vector of u8 values by encoding each u32 value as a series of bytes. The most significant bit of each byte is used to indicate whether more bytes are needed to represent the original u32 value.
    */
    let mut res = vec![];
    values.iter().rev().for_each(|&num| {
        res.push(num as u8 & 0x7F);
        let mut n = num >> 7;
        while n != 0 {
            res.push((n as u8 & 0x7F) | 0x80);
            n >>= 7;
        }
    });
    res.reverse();
    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    /*
    This code defines a function from_bytes that takes a slice of bytes (&[u8]) as input and returns a Result containing a vector of 32-bit unsigned integers (Vec<u32>) or an Error.

Let's break down the code step by step:

The function initializes an empty vector res to store the resulting numbers.
It creates a mutable iterator iter over the input bytes and makes it peekable. A peekable iterator allows us to look at the next element without consuming it.
It declares a variable number and initializes it to 0, which will be used to accumulate the decoded numbers.
The code enters a loop that iterates over each byte in the input slice using the while let Some(byte) = iter.next() pattern. This pattern consumes the iterator and assigns the current value to byte, exiting the loop when there are no more elements.
Inside the loop, the code checks if the number has enough leading zeros (using number.leading_zeros() >= 7) to accommodate another 7 bits. This check ensures that the number doesn't overflow when shifting and adding the next 7 bits.
If the check passes, the code shifts the existing number left by 7 bits (number << 7) and bitwise ORs it with the lower 7 bits of the current byte ((byte & 0x7F) as u32). This operation effectively appends the 7 bits from the current byte to the number.
If the check fails, indicating an overflow, the code returns an Err variant with the Error::Overflow value.
After updating the number, the code checks if the current byte's most significant bit is not set (byte & 0x80 == 0). If this condition is true, it means that the current number complete, and it can be added to the result vector res.
If the condition is true, the code appends the current number to res, resets number to 0, and continues to the next iteration of the loop.
If the condition is false, indicating that the current number is not complete yet, the code checks if there are no more bytes in the iterator (iter.peek().is_none()). If this condition is true, it means that the input bytes ended before a complete number was decoded, so the code returns an Err variant with the Error::IncompleteNumber value.
Finally, if the loop completes without encountering any errors, the code returns an Ok variant containing the resulting vector res.
In summary, this code decodes a sequence of bytes into a vector of 32-bit unsigned integers by applying a specific encoding scheme where each number is represented using a variable number of bytes. The most significant bit of each byte indicates whether the number continues in the next byte, and the lower 7 bits contribute to the number's value.
    */
    let mut res = vec![];
    let mut iter = bytes.iter().peekable();
    let mut number = 0u32;

    while let Some(byte) = iter.next() {
        number = if number.leading_zeros() >= 7 {
            (number << 7) | (byte & 0x7F) as u32
        } else {
            return Err(Error::Overflow)
        };
        if byte & 0x80 == 0 {
            res.push(number);
            number = 0;
            continue;
        } else if iter.peek().is_none() {
            return  Err(Error::IncompleteNumber)
        }
    }
    Ok(res)
}
