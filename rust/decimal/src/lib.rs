use num_bigint::BigInt;
use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, Eq)]
pub struct Decimal{
    int: BigInt,
    shift: usize,
    // implement your type here
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal>
    {
        let mut int = String::new();
        let mut shift = 0;
        for (i, c) in input.chars().enumerate() {
            if c == '.' {
                shift = input.len() - 1 - i;
            } else {
                int.push(c);
            }
        }
        let int = BigInt::parse_bytes(int.as_bytes(), 10)?;
        Some(Self { int, shift})
        // todo!("Create a new decimal with a value of {input}")
    }

    fn upshift(&self, shift: usize) -> Self {
        if self.shift >= shift {
            panic!("invalid upshift");
        }
        let shift_delta = shift - self.shift;
        Self {
            int: self.int.clone() * BigInt::from(10).pow(shift_delta as u32),
            shift,
        }
    }

    fn normalize_shifts(lhs: Self, rhs: Self) -> (Self, Self) {
        let left_shift = lhs.shift;
        let right_shift = rhs.shift;
        match lhs.shift.cmp(&rhs.shift) {
            Ordering::Equal => (lhs, rhs),
            Ordering::Less => (lhs.upshift(right_shift), rhs),
            Ordering::Greater => (lhs, rhs.upshift(left_shift))
        }
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (mut lhs, rhs) = Self::normalize_shifts(self, rhs);
        lhs.int += rhs.int;
        lhs
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (mut lhs, rhs) = Self::normalize_shifts(self, rhs);
        lhs.int -= rhs.int;
        lhs
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (mut lhs, rhs) = Self::normalize_shifts(self, rhs);
        lhs.int *= rhs.int;
        lhs.shift += rhs.shift;
        lhs
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs) = Self::normalize_shifts(self.clone(), other.clone());
        lhs.shift == rhs.shift && lhs.int == rhs.int
    }
}


impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (lhs, rhs) = Self::normalize_shifts(self.clone(), other.clone());
        lhs.int.partial_cmp(&rhs.int)
    }
}

// See https://exercism.org/tracks/rust/eNpxercises/decimal/solutions/vuongkienthanh