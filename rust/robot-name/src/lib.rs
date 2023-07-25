use rand::{thread_rng};
use rand::distributions::{Alphanumeric, Distribution, DistString, Uniform};
pub struct Robot(String);

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}

impl Robot {
    pub fn new() -> Self {
        let step = Uniform::new_inclusive(0, 9);
        let mut rng = thread_rng();
        let chars = Alphanumeric.sample_string(&mut rng, 2).chars().map(|c| c.to_uppercase().to_string()).collect::<String>();
        let numbers = step.sample_iter(&mut rng).take(3).map(|n| n.to_string()).collect::<String>();
        Self(format!("{chars}{numbers}"))
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn reset_name(&mut self) {
        unimplemented!("Assign a new unique name to the robot.");
    }
}
