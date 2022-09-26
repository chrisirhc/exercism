use std::collections::HashMap;

fn get_word_counts<'a>(words: &[&'a str]) -> HashMap<&'a str, u32> {
    let mut counts = HashMap::new();

    for &w in words.iter() {
        let prev_count = match counts.get(w) {
            Some(&count) => count,
            None => 0,
        };
        counts.insert(w, prev_count + 1);
    }
    counts
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_word_counts = get_word_counts(magazine);
    let note_word_counts = get_word_counts(note);

    for (word, required_count) in note_word_counts.iter() {
        let magazine_word_count_result = magazine_word_counts.get(word);
        match magazine_word_count_result {
            Some(magazine_word_count) if magazine_word_count >= required_count => {
                continue;
            }
            _ => {
                return false;
            }
        }
    }
    true
}
