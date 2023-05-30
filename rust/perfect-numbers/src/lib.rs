use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn find_factors_of_num(num: u64) -> HashSet<u64> {
    let mut factor_set = (1..2).collect::<HashSet<u64>>();

    for n in 2..=num {
        let divisior = num / n;
        if factor_set.contains(&n) || factor_set.contains(&divisior) {
            break
        }
        else if num % n == 0 {
            factor_set.insert(n);
            factor_set.insert(divisior);
        }
    }
    factor_set
}

pub fn classify(num: u64) -> Option<Classification> {
    // To find the factors of the number: If we iterate through the numbers and find the iterating number inside the factor tuple, then we end as we've found all the factors.

    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        2.. => {
            let aliquot_sum = find_factors_of_num(num).iter().sum::<u64>();
            match aliquot_sum {
                _ if aliquot_sum == num => Some(Classification::Perfect),
                _ if aliquot_sum > num => Some(Classification::Abundant),
                _ => Some(Classification::Deficient)
            }
        }
    }

}
