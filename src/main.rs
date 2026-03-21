use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::thread_rng;

// Bundled word list (replace with full top-10k later)
const WORDS: &str = include_str!("../assets/words.txt");

/// Anagram Gen — small CLI for jumbled-word prompts and simple solves
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a jumbled random prompt (same as running with no args)
    Prompt,
    /// Print the canonical sorted-letter form for a word
    Solve {
        /// Word to solve
        word: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Prompt) | None => prompt_mode(),
        Some(Commands::Solve { word }) => {
            let matches = find_anagrams(&word);
            if matches.is_empty() {
                println!("No anagrams found for: {}", word);
            } else {
                for m in matches {
                    println!("{}", m);
                }
            }
        }
    }
}

fn prompt_mode() {
    let choices: Vec<&str> = WORDS
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        // basic interestingness filter: alphabetic and length between 5 and 10
        .filter(|s| s.len() >= 5 && s.len() <= 10 && s.chars().all(|c| c.is_ascii_alphabetic()))
        .collect();

    let mut rng = thread_rng();
    let word = match choices.choose(&mut rng) {
        Some(&w) => w.to_string(),
        None => {
            eprintln!("No words available in bundled list (assets/words.txt)");
            std::process::exit(1);
        }
    };

    let mut chars: Vec<char> = word.chars().collect();
    chars.shuffle(&mut rng);
    let jumbled: String = chars.into_iter().collect();
    println!("{}", jumbled);
}

// helper `print_sorted` removed — use `find_anagrams` for solve functionality

fn canonical(s: &str) -> String {
    let mut chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn find_anagrams(target: &str) -> Vec<String> {
    let target_can = canonical(target);
    let mut matches = Vec::new();
    for line in WORDS.lines() {
        let w = line.trim();
        if w.is_empty() || w.starts_with('#') {
            continue;
        }
        // sanitize corpus word and compare canonical forms
        if canonical(w) == target_can {
            matches.push(w.to_string());
        }
    }
    matches
}
