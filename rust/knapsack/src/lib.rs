
#[derive(Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}


pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    let mut items = _items.to_vec();
    let mut max_val = 0;

    while let Some(item) = items.pop() {
        let val = if item.weight <= _max_weight {
            item.value + maximum_value(_max_weight - item.weight, &items)
        } else {
            0
        };

        max_val = if val > max_val { val } else { max_val }
    }
    max_val
}
