use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Handle edge cases: empty input or no workers
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    // Calculate chunk size, rounding up to ensure all strings are processed
    let chunk_size = input.len().div_ceil(worker_count);

    // Spawn threads to process chunks in parallel
    let handles: Vec<_> = input
        .chunks(chunk_size) // Split input into chunks
        .map(|chunk| {
            // Join chunk strings into single String to satisfy 'static lifetime for thread
            let text = chunk.join("");
            std::thread::spawn(move || {
                let mut local_map: HashMap<char, usize> = HashMap::new();
                // Count only alphabetic characters, converting to lowercase
                for ch in text.chars().filter(|c| c.is_alphabetic()) {
                    // Handle Unicode properly: some chars expand when lowercased
                    for lower in ch.to_lowercase() {
                        *local_map.entry(lower).or_insert(0) += 1;
                    }
                }
                local_map
            })
        })
        .collect();

    // Merge results from all threads
    let mut freq_map: HashMap<char, usize> = HashMap::new();
    for handle in handles {
        let local_map = handle.join().unwrap();
        // Add each thread's counts to the final frequency map
        for (ch, count) in local_map {
            *freq_map.entry(ch).or_insert(0) += count;
        }
    }
    freq_map
}
