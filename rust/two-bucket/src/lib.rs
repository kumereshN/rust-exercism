use std::cmp::{Ordering};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub struct BucketCapacity<'a> {
    bucket_name: &'a Bucket,
    capacity: u8,
    water_remaining: u8,
}


impl<'a> BucketCapacity<'a> {
    fn new(bucket_name: &'a Bucket, capacity: u8) -> BucketCapacity {
        Self {
            bucket_name,
            capacity,
            water_remaining: 0,
        }
    }

    fn fill_bucket(bucket: &'a mut BucketCapacity) {
        let water_to_pour = bucket.capacity.saturating_sub(bucket.water_remaining);
        bucket.water_remaining += water_to_pour;
    }

    fn empty_bucket(bucket: &'a mut BucketCapacity) {
        bucket.water_remaining = 0
    }

    fn is_full_bucket(bucket: &'a mut BucketCapacity) -> bool { bucket.water_remaining == bucket.capacity }

    fn is_empty_bucket(bucket: &'a mut BucketCapacity) -> bool { bucket.water_remaining == 0}

    fn goal_bucket(from_bucket: &'a mut BucketCapacity, to_bucket: &'a mut BucketCapacity, goal: u8) -> &'a Bucket {
        if from_bucket.water_remaining == goal {
            from_bucket.bucket_name
        }
        else {
            to_bucket.bucket_name
        }
    }

    fn pour_from_one_bucket_to_another(from_bucket: &'a mut BucketCapacity, to_bucket: &'a mut BucketCapacity) {
        let max_water_to_pour = to_bucket.capacity.saturating_sub(to_bucket.water_remaining);
        if from_bucket.water_remaining + to_bucket.water_remaining > to_bucket.capacity {
            from_bucket.water_remaining -= max_water_to_pour;
            to_bucket.water_remaining += max_water_to_pour;
        } else {
            to_bucket.water_remaining += from_bucket.water_remaining;
            from_bucket.water_remaining = 0;
        }
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
        let mut start_bucket_struct = BucketCapacity::new(start_bucket, if start_bucket == &Bucket::One {capacity_1} else {capacity_2});
        let mut other_bucket_struct = BucketCapacity::new(if start_bucket == &Bucket::Two {&Bucket::One} else {&Bucket::Two}, if start_bucket == &Bucket::One {capacity_2} else {capacity_1});

        let mut moves: u8 = 0;
        BucketCapacity::fill_bucket(& mut start_bucket_struct);
        moves +=1;

        while start_bucket_struct.water_remaining != goal && other_bucket_struct.water_remaining != goal {

            if BucketCapacity::is_empty_bucket(&mut start_bucket_struct) {
                BucketCapacity::fill_bucket(& mut start_bucket_struct);
                moves += 1;
            }

            match BucketCapacity::is_full_bucket(&mut other_bucket_struct) {
                true => {
                    BucketCapacity::empty_bucket(&mut other_bucket_struct);
                    moves += 1;
                },
                false => {
                    BucketCapacity::pour_from_one_bucket_to_another(&mut start_bucket_struct, &mut other_bucket_struct);
                    moves += 1;
                }
            }
        }

        Some(
            BucketStats {
                moves,
                goal_bucket: *BucketCapacity::goal_bucket(&mut start_bucket_struct, &mut other_bucket_struct, goal),
                other_bucket: if start_bucket_struct.water_remaining == goal { other_bucket_struct.water_remaining } else { start_bucket_struct.water_remaining }
            }
        )
    }