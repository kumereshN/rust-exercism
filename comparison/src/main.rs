#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    _first_list == _second_list
}

impl PartialEq for sublist {
    fn eq(&self, other: &Self) -> bool {
        let mut counter = 0;
        for element in self{
            if other.contains(element) {
                counter += 1;
            }
        }
        if self.len() == counter {
            true
        }
        else {
            false
        }
    }
}


fn main() {
    let s = &[1,2,3];
    let t = &[1,2,3,4];
    let res = sublist(s, t);
    println!("{}",res)
}
