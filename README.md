# Anagram Gen

A tiny CLI for jumbled-word prompts and simple anagram lookup.

Usage:

- Prompt mode (prints a jumbled word):

  `cargo run`

  (or `cargo run --bin anagram_gen`)

- `prompt` subcommand (same as no args):

  `cargo run -- prompt`

  In prompt mode, you can type guesses and use TAB to reshuffle the current jumble.

- `solve` subcommand: search the bundled corpus for single-word anagrams

  `cargo run -- solve <word-or-phrase>`

  For phrases with spaces, wrap in quotes:

  `cargo run -- solve "due sit"`

Example:

  `cargo run -- solve zebra`

Or build and run the binary:

```bash
  cargo build --release
  ./target/release/anagram_gen solve zebra
```

## Development

- Install `cargo-make` (recommended):

  ```bash
  cargo install cargo-make
  ```

- Common tasks (from the project root):

  - `cargo make check` — format, lint (clippy), and run tests
  - `cargo make run-prompt` — run prompt mode (prints a jumbled word)
  - `cargo make run-solve` — run the `solve orange` example

- You can also run pre-commit hooks directly:

  ```bash
  pre-commit run --all-files
  ```

These commands standardize developer workflows across machines and CI.