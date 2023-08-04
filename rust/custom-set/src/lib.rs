use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<'a, T> {
    set: &'a [T]
}

impl<'a, T> CustomSet<'a, T>
    where T:cmp::PartialEq
{
    pub fn new(_input: &'a [T]) -> Self {
        Self {
            set: _input,
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        unimplemented!();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        match (self.set.is_empty(), _other.set.is_empty()) {
            (true, true) => true,
            (true, false) => true,
            (false, true) => false,
            (_, _) => {
                self.set
                    .iter()
                    .all(|n| _other.contains(n))
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.set.len() == 0
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {

        self.set
            .iter()
            .all(|n| !_other.contains(n))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
