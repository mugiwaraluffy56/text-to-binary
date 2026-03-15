# Contributing

## Setup

```
git clone https://github.com/yourname/binrs
cd binrs
cargo build
```

## Running tests

```
cargo test
cargo clippy
```

## Adding a command

1. Add logic to the relevant module in `src/` (or create a new one)
2. Add the `Command` variant to the enum in `src/main.rs`
3. Add a match arm in `run()` to dispatch it
4. Add it to the command table in `README.md`

## Code style

- No comments anywhere
- No unwrap in public-facing paths, use `?` and return `Result<_, String>`
- All error messages lowercase, descriptive
- Keep modules focused: encode/decode/ops/inspect/stats/checksum/etc.

## Submitting changes

Open a pull request against `main`. Keep commits small and scoped to one thing.
