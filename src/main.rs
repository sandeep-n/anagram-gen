use clap::{Parser, Subcommand};

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
    /// Find single-word anagrams in the bundled corpus
    Solve {
        /// Word or phrase to solve
        word: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Prompt) | None => match anagram_gen::random_jumbled(anagram_gen::WORDS) {
            Some(j) => println!("{}", j),
            None => {
                eprintln!("No words available in bundled list (assets/words.txt)");
                std::process::exit(1);
            }
        },
        Some(Commands::Solve { word }) => {
            let matches = anagram_gen::find_anagrams(anagram_gen::WORDS, &word);
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
