use std::collections::HashSet;

pub fn refine(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = String::from(word).to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}

pub fn anagrams_for<'a>(origin_word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let origin_word = origin_word.to_lowercase();
    let word = refine(&origin_word);
    possible_anagrams
        .iter()
        .cloned()
        .filter(|x| x.to_lowercase() != origin_word && refine(x) == word)
        .collect::<HashSet<&'a str>>()
}
