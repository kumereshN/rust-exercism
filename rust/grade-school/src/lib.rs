// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

use std::collections::{BTreeMap, BTreeSet};

#[allow(clippy::new_without_default)]
pub struct School {
    grade_student_map: BTreeMap<u32,BTreeSet<String>>
}

impl School {
    pub fn new() -> School{
        Self {
            grade_student_map: BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: & str) {
        let students = self.grade_student_map
            .entry(grade)
            .or_default();
        students.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_student_map.keys().copied().collect::<Vec<u32>>()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grade_student_map
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .cloned()
            .collect()
    }
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}