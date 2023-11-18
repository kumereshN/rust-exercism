use std::cmp::max;

pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn knap_sack(max_w:u32, wt: Vec<u32>, val: Vec<u32>, n: usize) -> u32 {
    let mut dp = vec![0u32; max_w as usize];

    for i in 1..=n {
        for w in (1..=max_w).rev() {
            if wt[i-1] <= w {
                dp[w as usize - 1] = max(dp[w as usize - 1], dp[w as usize - wt[i-1] as usize] + val[i-1])
            }
        }
    }
    dp[max_w as usize - 1]
}

pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    let weight = _items.iter().map(|i| i.weight).collect::<Vec<u32>>();
    let profit = _items.iter().map(|i| i.value).collect::<Vec<u32>>();
    let n = profit.len();

    knap_sack(_max_weight, weight, profit, n)
}
