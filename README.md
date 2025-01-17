# stats-cards

🦀 Blazing fast stats cards for everything

## Available cards

Current list of available cards:

- [Wakatime languages (compact with progress)](https://github.com/anuraghazra/github-readme-stats)

List of planned cards:

- [WIP] [Github Streak](https://github.com/DenverCoder1/github-readme-streak-stats)
- [WIP] [Wakatime Streak](https://github.com/DenverCoder1/github-readme-streak-stats)
- [WIP] [Github stats](https://github.com/anuraghazra/github-readme-stats)
- [WIP] [Github languages (compact with progress)](https://github.com/anuraghazra/github-readme-stats)
- [WIP] [GitHub Extra Pins](https://github.com/anuraghazra/github-readme-stats)
- [WIP] [GitHub Gist Pins](https://github.com/anuraghazra/github-readme-stats)
- [WIP] Github Activity Graph

\* links contain design examples

## Themes

Available themes:

- [Catpuccin Macchiato](https://github.com/catppuccin/catppuccin) (default)

## How to run

To run your own instance:

1. Install [Rust 1.75+](https://www.rust-lang.org/learn/get-started)

2. (Optional) Run for developing:

   2.1. Install cargo watch:

   ```bash
   cargo install cargo-watch
   ```

   2.2. Run live server:

   ```bash
       cargo watch -x run
   ```

3. Run for Production:

   3.1. Build:

   ```bash
   cargo build --release
   ```

   3.2. Run a stats-cards file from `./target/release` folder:

   ```bash
   ./stats-cards
   ```

Written with ❤️ & 🦀
