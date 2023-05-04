pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right{
        let mut mid = (left + right) / 2;
        let mid_no = *array.get(mid).unwrap();

        if mid_no == key {
            return Some(mid)
        }
        else if mid_no > key {
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }
    return None
}
