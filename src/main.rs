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
    Solve {
        /// Word or phrase to solve
        word: String,
    },
}

fn run_interactive_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let answer = anagram_gen::pick_random_choice(anagram_gen::WORDS)
        .ok_or("corpus has no eligible prompt words")?;
    let scrambled = loop {
        let cand = anagram_gen::shuffle_word(&answer);
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

    println!("{}", formatted_prompt);
    println!("Type your guess (or 'give up' to reveal the answer)");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut guess = String::new();
        let nread = io::stdin().read_line(&mut guess)?;
        if nread == 0 {
            println!("EOF received, the word was: {}", answer);
            break;
        }
        let guess = guess.trim().to_lowercase();
        if guess.is_empty() {
            println!("Please type a guess or 'give up'.");
            continue;
        }

        if guess == "give up" || guess == "giveup" || guess == "quit" || guess == "exit" {
            println!("Give up! The word was: {}", answer);
            break;
        }

        if guess == answer {
            println!("🎉 Correct! The word is {}", answer);
            break;
        }

        println!("Nope, try again or type 'give up'.");
    }

    Ok(())
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
