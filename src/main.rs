use clap::{Parser, Subcommand};
use std::io::{self, Write};

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
    ///
    /// Words and phrases are both accepted; for phrases, wrap in quotes:
    /// `cargo run -- solve "fun day"`.
    Solve {
        /// Word or phrase to solve (use quotes for spaces)
        word: String,
    },
}

fn run_interactive_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let map = anagram_gen::BUNDLED_MAP.lock().unwrap();

    println!(
        "Enter guesses for each prompt. Type 'give up' to reveal the answer, 'quit'/'exit' to leave, or Ctrl+D to exit."
    );

    loop {
        let answer = anagram_gen::pick_random_prompt_word(&map)
            .ok_or("corpus has no eligible prompt words")?;
        let scrambled = loop {
            let cand = anagram_gen::shuffle(&answer);
            if cand != answer {
                break cand;
            }
        };

        let formatted_prompt = scrambled
            .to_uppercase()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("·");

        println!("\n{}", formatted_prompt);

        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut guess = String::new();
            let nread = io::stdin().read_line(&mut guess)?;
            if nread == 0 {
                println!("\nGoodbye.");
                return Ok(());
            }
            let guess = guess.trim().to_lowercase();
            if guess.is_empty() {
                continue;
            }

            if guess == "quit" || guess == "exit" {
                println!("Exiting prompt mode.");
                return Ok(());
            }

            if guess == "give up" || guess == "giveup" {
                println!("The word was: {}", answer.to_uppercase());
                break;
            }

            if guess == answer {
                println!("🎉 Correct!");
                break;
            }

            println!("Nope, try again or type 'give up'.");
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Prompt) | None => {
            if let Err(e) = run_interactive_prompt() {
                eprintln!("Prompt failed: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Solve { word }) => {
            let map = anagram_gen::BUNDLED_MAP.lock().unwrap();
            let matches = anagram_gen::find_anagrams_from_map(&map, &word);
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
