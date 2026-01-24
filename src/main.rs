use std::env;

fn main() {
    let word = match env::args().nth(1) {
        Some(w) => w,
        None => {
            eprintln!("Usage: anagram-gen <word>");
            std::process::exit(2);
        }
    };

    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    let sorted: String = chars.into_iter().collect();

    println!("{}", sorted);
}
