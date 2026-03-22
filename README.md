# Anagram Gen

A tiny CLI for jumbled-word prompts and simple anagram lookup.

Usage:

- Prompt mode (prints a jumbled word):

  `cargo run`

- `prompt` subcommand (same as no args):

  `cargo run -- prompt`

- `solve` subcommand: search the bundled corpus for single-word anagrams

  `cargo run -- solve <word-or-phrase>`

Example:

  `cargo run -- solve zebra`

Or build and run the binary:

  cargo build --release
  ./target/release/anagram_gen solve zebra
