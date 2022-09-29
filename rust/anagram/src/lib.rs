use std::collections::{HashMap, HashSet};

fn count_letters(word: &str) -> HashMap<char, u8> {
    let mut letter_counts = HashMap::new();
    for c in word.chars() {
        let count = match letter_counts.get(&c) {
            Some(&count) => count,
            None => 0,
        };
        letter_counts.insert(c, count + 1);
    }
    letter_counts
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let count_for_word = count_letters(&lowercase_word);
    let mut matched_words = HashSet::new();
    for &candidate in possible_anagrams.iter() {
        let lowercase_candidate = candidate.to_lowercase();
        if lowercase_word.eq(&lowercase_candidate) {
            continue;
        }
        let count_for_candidates = count_letters(&lowercase_candidate);
        if count_for_word.eq(&count_for_candidates) {
            matched_words.insert(candidate);
        }
    }
    matched_words
}
