use once_cell::sync::Lazy;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

const MIN_INTERESTING_WORD_LEN: usize = 5;
const MAX_INTERESTING_WORD_LEN: usize = 10;

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

/// Determine whether a single corpus word is eligible for prompt mode.
pub fn is_interesting_word(word: &str) -> bool {
    let word = word.trim();
    let len = word.len();
    (MIN_INTERESTING_WORD_LEN..=MAX_INTERESTING_WORD_LEN).contains(&len)
        && word.chars().all(|c| c.is_ascii_alphabetic())
}

/// Convert a text line from the corpus into a word candidate.
pub fn line_to_word(line: &str) -> Option<&str> {
    let candidate = line.trim();
    if candidate.is_empty() || candidate.starts_with('#') {
        None
    } else {
        Some(candidate)
    }
}

/// Find single-word anagrams of `target` in the bundled corpus.
/// Returns a list of matching corpus words (deduplicated, original order preserved).
pub fn find_anagrams(words: &str, target: &str) -> Vec<String> {
    let target_can = canonical(target);
    let map = build_map(words);
    map.get(&target_can).cloned().unwrap_or_default()
}

/// Find single-word anagrams of `target` using a prebuilt canonical map.
pub fn find_anagrams_from_map(map: &HashMap<String, Vec<String>>, target: &str) -> Vec<String> {
    let target_can = canonical(target);
    map.get(&target_can).cloned().unwrap_or_default()
}

/// Build a canonical->words map for the provided corpus.
fn build_map(words: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen = HashSet::new();
    for word in words.lines().filter_map(line_to_word) {
        if seen.insert(word.to_string()) {
            map.entry(canonical(word))
                .or_default()
                .push(word.to_string());
        }
    }
    map
}

const BUNDLED_MAP_BYTES: &[u8] = include_bytes!("../assets/bundled_map.bin");

/// Cached map built from the bundled corpus asset for fast lookups.
pub static BUNDLED_MAP: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| {
    let (map, _) =
        bincode::serde::decode_from_slice(BUNDLED_MAP_BYTES, bincode::config::standard())
            .expect("failed to deserialize bundled map");
    Mutex::new(map)
});

/// Return filtered choices from the bundled corpus.
pub fn filtered_choices(words: &str) -> Vec<&str> {
    words
        .lines()
        .filter_map(line_to_word)
        .filter(|word| is_interesting_word(word))
        .collect()
}

/// Pick a random cleaned choice word from the corpus.
pub fn pick_random_choice(words: &str) -> Option<String> {
    filtered_choices(words)
        .choose(&mut thread_rng())
        .map(|word| word.to_string())
}

/// Pick a random prompt answer from the prebuilt canonical map.
pub fn pick_random_prompt_word(map: &HashMap<String, Vec<String>>) -> Option<String> {
    let mut rng = thread_rng();
    let buckets: Vec<_> = map
        .values()
        .filter(|values| values.iter().any(|word| is_interesting_word(word)))
        .collect();
    let chosen_bucket = buckets.choose(&mut rng)?;
    chosen_bucket
        .iter()
        .filter(|word| is_interesting_word(word))
        .choose(&mut rng)
        .cloned()
}

/// Shuffle letters of a word.
pub fn shuffle(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
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
