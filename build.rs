use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct CanonicalMap(HashMap<String, Vec<String>>);

fn canonical(word: &str) -> String {
    let mut chars: Vec<char> = word
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=assets/words.txt");

    let text = fs::read_to_string("assets/words.txt")?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen = HashSet::new();

    for line in text.lines() {
        let word = line.trim();
        if word.is_empty() || word.starts_with('#') {
            continue;
        }

        if !seen.insert(word.to_string()) {
            continue;
        }

        let key = canonical(word);
        map.entry(key).or_default().push(word.to_string());
    }

    let bundle = CanonicalMap(map);
    let bytes = bincode::serde::encode_to_vec(&bundle, bincode::config::standard())
        .expect("failed to serialize bundled map");

    let out_path = Path::new("assets").join("bundled_map.bin");
    let mut out_file = fs::File::create(&out_path)?;
    out_file.write_all(&bytes)?;
    Ok(())
}
