#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CustomSet<T>
{
    set: Vec<T>
}

impl<T> CustomSet<T>
    where T: PartialEq + Ord + Clone
{
    pub fn new(_input: &[T]) -> Self {
        let mut set = _input.to_vec();
        set.sort_unstable();
        set.dedup();
        Self {
            set
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.set.extend([_element]);
        self.set.sort_unstable();
        self.set.dedup()
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.set
            .iter()
            .all(|n| _other.contains(n))
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
        self
            .set
            .iter()
            .filter_map(|n| match _other.contains(n) {
                true => Some(n),
                false => None
            })
            .collect()
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
