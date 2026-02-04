use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    let chunk_size = (input.len() + worker_count - 1) / worker_count;

    crossbeam::scope(|scope| {
        let handles: Vec<_> = input
            .chunks(chunk_size)
            .map(|chunk| {
                scope.spawn(move |_| {
                    let mut local_map: HashMap<char, usize> = HashMap::new();
                    for &line in chunk {
                        for ch in line.chars().filter(|c| c.is_alphabetic()) {
                            for lower in ch.to_lowercase() {
                                *local_map.entry(lower).or_insert(0) += 1;
                            }
                        }
                    }
                    local_map
                })
            })
            .collect();

        let mut freq_map: HashMap<char, usize> = HashMap::new();
        for handle in handles {
            let local_map = handle.join().unwrap();
            for (ch, count) in local_map {
                *freq_map.entry(ch).or_insert(0) += count;
            }
        }
        freq_map
    })
    .unwrap()
}
