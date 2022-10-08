/*
test bench_large_parallel   ... bench:     114,406 ns/iter (+/- 12,091)
test bench_large_sequential ... bench:     444,719 ns/iter (+/- 59,682)
test bench_small_parallel   ... bench:      55,430 ns/iter (+/- 4,629)
test bench_small_sequential ... bench:      15,842 ns/iter (+/- 2,190)
test bench_tiny_parallel    ... bench:      33,638 ns/iter (+/- 8,013)
test bench_tiny_sequential  ... bench:          98 ns/iter (+/- 41)
*/

use crossbeam_utils::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    // Naive way of concurrency, split up based on x number of workers to count concurrently by splitting the strings to multiple vectors.
    let chunk_size = 1.max(input.len() / worker_count);
    let chunks = input.chunks(chunk_size);
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    thread::scope(|s| {
        let mut threads = Vec::new();
        for chunk in chunks {
            let thread_handle = s.spawn(move |_| process_chunk(chunk));
            threads.push(thread_handle);
        }

        for thread_handle in threads {
            if let Ok(counts) = thread_handle.join() {
                // Merge counts
                for (c, count) in counts.iter() {
                    if let Some(old_count) = char_counts.get_mut(c) {
                        *old_count = *old_count + count;
                    } else {
                        char_counts.insert(*c, *count);
                    }
                }
            } else {
                panic!("Error!")
            }
        }
    })
    .unwrap();
    char_counts
}

fn process_chunk(chunk: &[&str]) -> HashMap<char, usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for &input_s in chunk {
        for c in input_s.chars() {
            if !c.is_alphabetic() {
                continue;
            }
            count_char(&mut char_counts, &c.to_ascii_lowercase());
        }
    }
    char_counts
}

fn count_char(char_counts: &mut HashMap<char, usize>, c: &char) {
    if let Some(new_countc) = char_counts.get_mut(c) {
        *new_countc = *new_countc + 1;
    } else {
        char_counts.insert(*c, 1);
    }
}
