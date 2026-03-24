use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

/// Bundled word list (replace with full top-10k later)
pub const WORDS: &str = include_str!("../assets/words.txt");

/// Canonicalize a string: keep letters only, lowercase, sorted
pub fn canonical(s: &str) -> String {
    let mut chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

/// Find single-word anagrams of `target` in the bundled corpus.
/// Returns a list of matching corpus words (deduplicated, original order preserved).
pub fn find_anagrams(words: &str, target: &str) -> Vec<String> {
    let target_can = canonical(target);
    let mut seen = HashSet::new();
    let mut matches = Vec::new();
    for line in words.lines() {
        let w = line.trim();
        if w.is_empty() || w.starts_with('#') {
            continue;
        }
        if canonical(w) == target_can && seen.insert(w.to_string()) {
            matches.push(w.to_string());
        }
    }
    matches
}

/// Return filtered choices from the bundled corpus.
pub fn filtered_choices(words: &str) -> Vec<&str> {
    words
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        // basic interestingness filter: alphabetic and length between 5 and 10
        .filter(|s| s.len() >= 5 && s.len() <= 10 && s.chars().all(|c| c.is_ascii_alphabetic()))
        .collect()
}

/// Pick a random word from the filtered choices and return a jumbled version.
pub fn random_jumbled(words: &str) -> Option<String> {
    let choices = filtered_choices(words);
    let mut rng = thread_rng();
    let word = choices.choose(&mut rng)?;
    let mut chars: Vec<char> = word.chars().collect();
    chars.shuffle(&mut rng);
    Some(chars.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_strips_and_sorts() {
        // "Panama!" -> letters p a n a m a -> lowercase -> a a a m n p -> "aaamnp"
        assert_eq!(canonical("Panama!"), "aaamnp");
    }

    #[test]
    fn find_anagrams_finds_orange() {
        let matches = find_anagrams(WORDS, "orange");
        assert!(
            matches.iter().any(|w| w.eq_ignore_ascii_case("orange")),
            "expected 'orange' in matches: {:?}",
            matches
        );
    }
}
