use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

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
    // Use precomputed map for O(1) lookup when available. The map is built from `words`.
    let target_can = canonical(target);
    let map = build_map(words);
    match map.get(&target_can) {
        Some(vec) => vec.clone(),
        None => Vec::new(),
    }
}

/// Build or return a cached canonical->words map for the provided corpus.
fn build_map(words: &str) -> HashMap<String, Vec<String>> {
    // Note: we don't memoize per-corpus string value; we build a fresh map each call for now.
    // For binary use with the bundled `WORDS` constant, we expose a lazy cached map below.
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen = HashSet::new();
    for line in words.lines() {
        let w = line.trim();
        if w.is_empty() || w.starts_with('#') {
            continue;
        }
        if seen.insert(w.to_string()) {
            let can = canonical(w);
            map.entry(can).or_default().push(w.to_string());
        }
    }
    map
}

/// Cached map built from the bundled `WORDS` constant for fast lookups.
pub static BUNDLED_MAP: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| {
    let m = build_map(WORDS);
    Mutex::new(m)
});

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

    #[test]
    fn bundled_map_contains_orange() {
        let can = canonical("orange");
        let map = BUNDLED_MAP.lock().unwrap();
        let entry = map
            .get(&can)
            .expect("expected canonical key in BUNDLED_MAP");
        assert!(entry.iter().any(|w| w.eq_ignore_ascii_case("orange")));
    }

    #[test]
    fn find_anagrams_matches_bundled_map() {
        let input = "orange";
        let by_fn = find_anagrams(WORDS, input);
        let map = BUNDLED_MAP.lock().unwrap();
        let by_map = map.get(&canonical(input)).cloned().unwrap_or_default();
        // Compare sets (order may differ)
        let set_fn: std::collections::HashSet<_> = by_fn.into_iter().collect();
        let set_map: std::collections::HashSet<_> = by_map.into_iter().collect();
        assert_eq!(set_fn, set_map);
    }
}
