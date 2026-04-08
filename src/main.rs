use clap::{Parser, Subcommand};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, queue, style::Print};
use std::io::{self, Stdout, Write};

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
    let mut stdout = io::stdout();

    println!(
        "Enter guesses for each prompt. Tab reshuffles, 'give up' reveals the answer, 'quit'/'exit' leaves, Ctrl+D exits."
    );

    terminal::enable_raw_mode()?;
    let result = (|| {
        loop {
            let answer = anagram_gen::pick_random_prompt_word(&map)
                .ok_or("corpus has no eligible prompt words")?;
            let mut scrambled = loop {
                let cand = anagram_gen::shuffle(&answer);
                if cand != answer {
                    break cand;
                }
            };
            let mut formatted_prompt = format_prompt(&scrambled);
            let mut guess = String::new();

            draw_new_prompt(&mut stdout, &formatted_prompt, &guess)?;

            loop {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read()?
                {
                    match code {
                        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                            print!("\r\nThe word was: {}\r\n", answer.to_uppercase());
                            return Ok(());
                        }
                        KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
                            print!("\r\nThe word was: {}\r\n", answer.to_uppercase());
                            return Ok(());
                        }
                        KeyCode::Tab => {
                            scrambled = loop {
                                let cand = anagram_gen::shuffle(&answer);
                                if cand != answer {
                                    break cand;
                                }
                            };
                            formatted_prompt = format_prompt(&scrambled);
                            render_prompt(&mut stdout, &formatted_prompt, &guess)?;
                        }
                        KeyCode::Enter => {
                            let normalized = guess.trim().to_lowercase();
                            if normalized.is_empty() {
                                continue;
                            }
                            if normalized == "quit" || normalized == "exit" {
                                print!("\r\nThe word was: {}\r\n", answer.to_uppercase());
                                return Ok(());
                            }
                            if normalized == "give up" || normalized == "giveup" {
                                print!("\r\nThe word was: {}\r\n", answer.to_uppercase());
                                break;
                            }
                            if normalized == answer {
                                print!("\r\n🎉 Correct!\r\n");
                                break;
                            }
                            print!("\r\nNope, try again or type 'give up'.\r\n");
                            guess.clear();
                            render_prompt(&mut stdout, &formatted_prompt, &guess)?;
                        }
                        KeyCode::Backspace => {
                            guess.pop();
                            render_prompt(&mut stdout, &formatted_prompt, &guess)?;
                        }
                        KeyCode::Char(c) => {
                            guess.push(c);
                            render_prompt(&mut stdout, &formatted_prompt, &guess)?;
                        }
                        _ => {}
                    }
                }
            }
        }
    })();

    terminal::disable_raw_mode()?;
    result
}

fn format_prompt(scrambled: &str) -> String {
    scrambled
        .to_uppercase()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("·")
}

fn draw_new_prompt(
    stdout: &mut Stdout,
    prompt: &str,
    guess: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        Print("\r\n"),
        Print(prompt),
        Print("\r\n> "),
        Print(guess)
    )?;
    stdout.flush()?;
    Ok(())
}

fn render_prompt(
    stdout: &mut Stdout,
    prompt: &str,
    guess: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    queue!(
        stdout,
        cursor::MoveUp(1),
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
        Print(prompt),
        cursor::MoveDown(1),
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
        Print(format!("> {}", guess))
    )?;
    stdout.flush()?;
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
