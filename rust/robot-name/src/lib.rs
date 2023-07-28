use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};

// The lazy_static crate is used to ensure that the counter is initialized lazily and only once,
// even if multiple instances of the Robot struct are created.
lazy_static! {
    static ref COUNTER: AtomicUsize = AtomicUsize::new(0);
}

pub struct Robot{
    name: String,
}

fn next_count() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn next_name() -> String {
    let count = next_count();
    let alp = (count / 1000) as u32;
    let first_char = char::from_u32('A' as u32 + alp / 26).unwrap();
    let second_char = char::from_u32('A' as u32 + alp % 26).unwrap();
    // Obtain the last 3 digits
    let serial = count % 1000;

    format!("{}{}{:0>3}",
            first_char,
            second_char,
            serial
    )
}

impl Default for Robot {
    fn default() -> Self {
        Robot::new()
    }
}

impl Robot {
    pub fn new() -> Self {
        Robot { name : next_name()}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = next_name();
    }
}
