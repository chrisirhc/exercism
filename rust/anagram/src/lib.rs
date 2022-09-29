use std::collections::{HashMap, HashSet};

fn count_letters(word: &str) -> HashMap<char, u8> {
    let mut letter_counts = HashMap::new();
    for c in word.chars() {
        let lowercase_c = c.to_ascii_lowercase();
        let count = match letter_counts.get(&lowercase_c) {
            Some(count) => count + 1,
            None => 0,
        };
        letter_counts.insert(lowercase_c, count);
    }
    letter_counts
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let count_for_word = count_letters(word);
    let mut matched_words = HashSet::new();
    for &candidate in possible_anagrams.iter() {
        if word.eq_ignore_ascii_case(candidate) {
            continue;
        }
        let count_for_candidates = count_letters(candidate);
        if count_for_word.eq(&count_for_candidates) {
            matched_words.insert(candidate);
        }
    }
    matched_words
}
