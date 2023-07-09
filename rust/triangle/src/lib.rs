use itertools::Itertools;

fn sum_of_any_two_sides_eq_or_greater_than_remaining_one_side(sides: &[u64; 3]) -> bool {
    let permutations = sides.iter().permutations(3);
    permutations
        .map(|x| {
            let total_sum = x[0] + x[1];
            total_sum >= *x[2]
        })
        .all(|b| b)
}

pub struct Triangle{
    sides: [u64; 3]
}

impl Triangle {

    fn new(sides: [u64; 3]) -> Triangle {
        Triangle {
            sides
        }
    }

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides;
        sides.sort_unstable();
        if sides.iter().all(|&n| n > 0) && sum_of_any_two_sides_eq_or_greater_than_remaining_one_side(&sides) {
            let t = Triangle::new(sides);
            return Some(t)
        }
        None
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.windows(2).all(|n| n[0] == n[1])
    }

    pub fn is_scalene(&self) -> bool {
        self.sides.windows(2).all(|n| n[0] != n[1])
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides.windows(2).any(|n| n[0] == n[1])
    }
}
