# Fuzzer

## Installation

```bash
cargo binstall cargo-fuzz
```

## Run

Run fuzzer for the parser, for 15 minutes.

```bash
rustup default nightly
# parser
cargo +nightly fuzz run --sanitizer none parser -- -only_ascii=1 -max_total_time=900 -timeout=5
# css parser
cargo +nightly fuzz run --sanitizer none css_parser -- -only_ascii=1 -max_total_time=900 -timeout=5
```
