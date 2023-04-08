# Rust

Let's have a look at Rust.

Getting started with "Rust by Example" at https://doc.rust-lang.org/rust-by-example/

## Helpful commands

- `cargo new`: Create a new cargo project. Use `cargo new --vcs none` to not create a git repo.
- `cargo run`: Build and run the project.
- `cargo build`: Build the project.

- `Cargo.toml`: The manifest file. It contains the project name, version, authors, dependencies, etc. Like package.json in nodejs.

- `rustc --version`: Check version of rust.
- `rustup update`: Update rust.

## Formatting

The Rust toolchain includes a formatter, [rustfmt](https://github.com/rust-lang/rustfmt). Make sure VS Code is configured to use the rustfmt from the toolchain. If you have for example Prettier used by default, you can add the following to your settings.json:

```json
"[rust]": {
	"editor.defaultFormatter": "rust-lang.rust-analyzer"
},
```
