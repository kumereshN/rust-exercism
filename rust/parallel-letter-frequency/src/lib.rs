use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let vec_chars: Vec<char> = input
        .iter()
        .flat_map(|&c|
            c
                .chars()
                .flat_map(|c|
                    c.to_lowercase().filter(|&c| c.is_ascii_alphabetic())
                ))
        .collect();

    let count_map: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    let chunk_size = vec_chars.len() / worker_count;
    let chunks: Vec<&[char]> = vec_chars.chunks(chunk_size).collect();

    for chunk in chunks {
        let count_map_clone = Arc::clone(&count_map);
        let handle = thread::spawn(move || {
            let mut local_map = count_map_clone.lock().unwrap();
            for &c in chunk {
                let counter = local_map.entry(c).or_insert(0);
                *counter += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_map = Arc::try_unwrap(count_map).unwrap().into_inner().unwrap();
    final_map
}
