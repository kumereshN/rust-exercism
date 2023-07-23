// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot(i32,i32,Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self(x, y, d)
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.2 {
            Direction::North => self.2 = Direction::East,
            Direction::East => self.2 = Direction::South,
            Direction::South => self.2 = Direction::West,
            Direction::West => self.2 = Direction::North
        }
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.2 {
            Direction::North => self.2 = Direction::West,
            Direction::East => self.2 = Direction::North,
            Direction::South => self.2 = Direction::East,
            Direction::West => self.2 = Direction::South
        }
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.2 {
            Direction::North => self.1 += 1,
            Direction::South => self.1 -= 1,
            Direction::East =>  self.0 += 1,
            Direction::West => self.0 -= 1
        }

        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instructions| {
            match instructions {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => robot
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
