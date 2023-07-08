pub struct Triangle{
    sides: [u64; 3]
}

impl Triangle {

    pub fn new(sides: [u64; 3]) -> Triangle {
        Triangle {
            sides
        }
    }

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides;
        sides.sort_unstable();
        if sides.iter().all(|&n| n > 0) {
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
