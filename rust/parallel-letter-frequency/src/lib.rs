use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Naive way of concurrency, split up based on x number of workers to count concurrently by splitting the strings to multiple vectors.
    let input_arc = Arc::new(input);
    let (tx, rx) = mpsc::channel();
    let chunk_size = 1.max(input.len() / worker_count);

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    thread::scope(|scope| {
        let chunks = input_arc.chunks(chunk_size);
        for chunk in chunks {
            let tx_c = tx.clone();
            scope.spawn(move || {
                for &input_s in chunk {
                    for c in input_s.chars() {
                        print!("{:}", c);
                        tx_c.send(c).unwrap();
                    }
                }
            });
        }

        // Make sure to drop the main tx reference
        drop(tx);

        for c in rx {
            if let Some(new_countc) = char_counts.get_mut(&c) {
                *new_countc = *new_countc + 1;
            } else {
                char_counts.insert(c, 1);
            }
        }

        println!("xyz z");
    });

    char_counts
}

// 5 / 5, means chunks of 1
// 5 / 6 still means chunks of 1: 1, 1, 1,
// 6 / 2 means chunks of 3
// 6 / 4 means chunks of 1, so 1, 1, 1, 3 or 2,2,2,0 . Max parallelism, so always do ceil?
// 6 / 5: 2, 2, 2, 0, 0 ? Or 1, 1, 1, 1, 2 ? 6 / 5 -> 1
