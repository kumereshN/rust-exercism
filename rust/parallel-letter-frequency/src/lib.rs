use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]|{
        input
            .iter()
            .flat_map(|&c| c.chars().flat_map(|c| c.to_lowercase().filter(|&c| c.is_alphabetic())))
            .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
        };

    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);
            let chunks = input.chunks(input.len() / worker_count + 1);
            for chunk in chunks {
                handles.push(s.spawn(|| counter(chunk)))
            }
            handles
                .into_iter()
                .fold(HashMap::new(), |mut acc, c| {
                    let res = c.join().unwrap();
                    for (&k, &v) in res.iter() {
                        *acc.entry(k).or_default() += v;
                    }
                    acc
                })
        }),
    }
}