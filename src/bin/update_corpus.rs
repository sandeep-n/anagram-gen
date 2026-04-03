use anyhow::{Context, Result};
use std::fs;
use std::io::Write;

const MAX_WORDS: usize = 10_000;

/// Strip non-ASCII-alphabetic characters, lowercase, and return `None` if the result is empty.
fn clean_word(word: &str) -> Option<String> {
    let cleaned: String = word
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

/// Small helper to download and clean the corpus into `assets/words.txt`.
///
/// Usage:
///   cargo run --bin update_corpus
fn main() -> Result<()> {
    let url = "https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2016/en/en_50k.txt";
    println!("Downloading corpus from {}...", url);

    let resp = ureq::get(url)
        .call()
        .map_err(|e| anyhow::anyhow!("request failed: {}", e))?;
    if resp.status() != 200 {
        anyhow::bail!("request failed: {}", resp.status());
    }
    let body = resp
        .into_string()
        .map_err(|e| anyhow::anyhow!("failed to read response body: {}", e))?;

    // Process lines: take first MAX_WORDS lines, extract first column, clean, skip empties.
    // Format assumed: "word frequency" per line.
    let words: Vec<String> = body
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(MAX_WORDS)
        .filter_map(|line| line.split_whitespace().next().and_then(clean_word))
        .collect();

    fs::create_dir_all("assets").context("failed to create assets dir")?;
    let path = "assets/words.txt";
    let mut file = fs::File::create(path).context("failed to create assets/words.txt")?;
    for word in &words {
        writeln!(file, "{}", word)?;
    }
    println!("Wrote {} words to {}", words.len(), path);
    Ok(())
}
