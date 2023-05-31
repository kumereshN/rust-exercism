use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else{
        let aliquot_sum = (1..num).filter(|&f| num % f == 0).sum::<u64>();
        match aliquot_sum.cmp(&num) {
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Greater => Some(Classification::Abundant)
        }
    }
}
