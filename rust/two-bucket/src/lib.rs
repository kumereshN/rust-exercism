use std::cmp::max;

#[derive(PartialEq, Eq, Debug)]
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

    fn fill_bucket(bucket: &'a mut BucketCapacity, water_to_fill: u8) {
        bucket.water_remaining += water_to_fill;
    }

    fn empty_bucket(bucket: &'a mut BucketCapacity) {
        bucket.water_remaining = 0
    }

    fn pour_from_one_bucket_to_another(from_bucket: &'a mut BucketCapacity, to_bucket: &'a mut BucketCapacity, water_to_pour: u8) {
        let max_water_to_pour = to_bucket.capacity.saturating_sub(from_bucket.water_remaining);
        if to_bucket.water_remaining + water_to_pour > to_bucket.capacity {
            from_bucket.water_remaining -= max_water_to_pour;
            to_bucket.water_remaining += max_water_to_pour;
        } else {
            from_bucket.water_remaining -= water_to_pour;
            to_bucket.water_remaining += water_to_pour;
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
        let mut bucket_1 = BucketCapacity::new(&Bucket::One, capacity_1);
        let mut bucket_2 = BucketCapacity::new(&Bucket::Two, capacity_2);

        let mut moves = 0u8;

        match start_bucket {
            Bucket::One => {
                while bucket_1.water_remaining.ne(&goal) && bucket_2.water_remaining.ne(&goal) {
                    BucketCapacity::fill_bucket(&mut bucket_1, capacity_1);
                    moves += 1;
                    BucketCapacity::pour_from_one_bucket_to_another(&mut bucket_1, &mut bucket_2, capacity_1);
                    moves += 1;
                }
            }

            Bucket::Two => {
                BucketCapacity::fill_bucket(&mut bucket_2, capacity_2);
            }
        }

        Some(
            BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: bucket_2.water_remaining
            }
        )
    }