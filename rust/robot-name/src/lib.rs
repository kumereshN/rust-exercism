use std::collections::HashSet;
use rand::{thread_rng};
use rand::distributions::{Distribution, Uniform};
pub struct Robot{
    robot_id: String,
    seen: HashSet<String>
}

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}


impl Robot {

    pub fn new() -> Self {
       let robot = Robot::create_robot();

       while robot.seen.contains(robot.robot_id) {
           
       }
    }

    pub fn create_robot() -> Self {
        let step = Uniform::new_inclusive(0, 9);
        let chars = Uniform::new_inclusive('A', 'Z');
        let mut seen = HashSet::new();

        let mut rng = thread_rng();

        let chars = chars.sample_iter(&mut rng).take(2).map(|c| c.to_uppercase().to_string()).collect::<String>();
        let numbers = step.sample_iter(&mut rng).take(3).map(|n| n.to_string()).collect::<String>();
        let robot_id = format!("{chars}{numbers}");

        seen.insert(robot_id.clone());
        Self{robot_id, seen}
    }

    pub fn name(&self) -> &str {
        self.robot_id.as_str()
    }

    pub fn reset_name(&mut self) {
        self.robot_id = Robot::new().robot_id;
    }

    pub fn is_duplicated(&self) -> bool {
        let seen: HashSet<String> = HashSet::new();
        seen.contains(self.robot_id.as_str())
    }
}
