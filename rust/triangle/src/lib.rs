pub struct Triangle<T>
{
    sides: [T; 3]
}

impl<T> Triangle<T>
    where T: Copy + std::ops::Add<Output = T> + PartialOrd
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sides = sides;
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sides[0] + sides[1] > sides[2] {
            return Some(Self { sides })
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
