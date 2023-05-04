use std::cmp::Ordering;

pub fn find<T: AsRef<[U]>, U: Ord>(array: T, key: U) -> Option<usize> {
    let array = array.as_ref();

    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
            Ordering::Equal => Some(mid),
            Ordering::Less => find(&array[..mid], key),
            Ordering::Greater => find(&array[mid + 1..], key).map(|i| i + mid + 1),
        }
    }

