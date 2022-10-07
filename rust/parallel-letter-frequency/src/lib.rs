use std::collections::HashMap;
use std::sync::mpsc;
use crossbeam_utils::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Naive way of concurrency, split up based on x number of workers to count concurrently by splitting the strings to multiple vectors.
    let (tx, rx) = mpsc::channel();
    let chunk_size = 1.max(input.len() / worker_count);
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    let chunks = input.chunks(chunk_size);
    thread::scope(|s| {
        for chunk in chunks {
            let tx_c = tx.clone();
            s.spawn(move |_| {
                for &input_s in chunk {
                    for c in input_s.chars() {
                        if !c.is_alphabetic() {
                            continue;
                        }
                        tx_c.send(c.to_ascii_lowercase()).unwrap();
                    }
                }
            });
        }
    }).unwrap();

    // Make sure to drop the main tx reference
    drop(tx);

    for c in rx {
        if let Some(new_countc) = char_counts.get_mut(&c) {
            *new_countc = *new_countc + 1;
        } else {
            char_counts.insert(c, 1);
        }
    }

    char_counts
}
