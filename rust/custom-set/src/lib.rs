#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T>
    where T: PartialEq + Ord + Clone
{
    pub fn new(_input: &[T]) -> Self {
        let mut set = _input.to_vec();
        set.sort_unstable();
        set.dedup();
        Self(set)
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.0.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.0.push(_element);
        self.0.sort_unstable();
        self.0.dedup()
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.0
            .iter()
            .all(|n| _other.contains(n))
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.0
            .iter()
            .all(|n| !_other.contains(n))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        Self(self.0
                .clone()
                .into_iter()
                .filter(|n|
                    _other.contains(n))
                .collect::<Vec<_>>()
        )
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        Self(self.0
            .clone()
            .into_iter()
            .filter(|n|
                _other.contains(n))
            .collect::<Vec<_>>()
        )
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut set = self.0.clone();

        set.extend(_other.0.clone());
        set.sort_unstable();
        set.dedup();

        Self(
            set
        )
    }
}
