use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use crossbeam_utils::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Naive way of concurrency, split up based on x number of workers to count concurrently by splitting the strings to multiple vectors.
    let (tx, rx) = channel();
    let chunk_size = 1.max(input.len() / worker_count);

    let chunks = input.chunks(chunk_size);
    thread::scope(|s| {
        for chunk in chunks {
            let tx_c = tx.clone();
            s.spawn(move |_| {
                process_chunk(chunk, tx_c);
            });
        }
    }).unwrap();

    // Make sure to drop the main tx reference otherwise it wait forever for new tx.
    drop(tx);
    count_chars_from_channel(rx)
}

fn process_chunk(chunk: &[&str], tx_c: Sender<char>) {
    for &input_s in chunk {
        for c in input_s.chars() {
            if !c.is_alphabetic() {
                continue;
            }
            tx_c.send(c.to_ascii_lowercase()).unwrap();
        }
    }
}

fn count_chars_from_channel(chan: Receiver<char>) -> HashMap<char, usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for c in chan {
        if let Some(new_countc) = char_counts.get_mut(&c) {
            *new_countc = *new_countc + 1;
        } else {
            char_counts.insert(c, 1);
        }
    }
    char_counts
}
