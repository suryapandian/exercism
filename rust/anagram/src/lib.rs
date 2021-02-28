use std::collections::HashSet;

const NUM_UNICODE_CHARS: usize = 143_859;

fn count_every_char(word: &str) -> Vec<i32> {
    let mut characters = vec![0; NUM_UNICODE_CHARS];
    for letter in word.to_lowercase().chars() {
        let index = letter as usize;
        if index > NUM_UNICODE_CHARS {
            continue;
        }
        characters[index] += 1;
    }
    characters
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let original_count = count_every_char(word);
    let mut result = HashSet::<&'a str>::new();

    for possible_anagram in possible_anagrams {
        if word.to_lowercase() == (*possible_anagram).to_lowercase() {
            continue;
        }

        let count = count_every_char(possible_anagram);
        if original_count == count {
            result.insert(possible_anagram);
        }
    }
    result
}
