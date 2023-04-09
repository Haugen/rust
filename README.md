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

## Section notes

### [1. Hello World](./hello-world)

https://doc.rust-lang.org/rust-by-example/hello.html

First lesson focused on Formatted print. How to print to the console using `println!()`. How to pass in arguments to the macro, using `{}` as a placeholder. Using `{:?}` to `Debug` instead of `Display`. There are many more [formatting traits](https://doc.rust-lang.org/std/fmt/#traits).

Custom structs can be printed with `{:?}` or `{:#?}` by adding `#[derive(Debug)]` to the line above its declaration. This is a [derive macro](https://doc.rust-lang.org/rust-by-example/trait/derive.html). For `Display`, the struct needs to implement the `Display` trait manually (as shown in the example).

Still don't understand

- struct

### [2. Primitives](./primitives)

https://doc.rust-lang.org/rust-by-example/primitives.html

u8, u16 etc, are unsigned integers. i8, i16 etc, are signed integers. The difference is that unsigned integers can not hold negative values. For example u8 can store values from 0 to 255. i8 can store values from -128 to 127.

Further detailed introduction to all the types. More fine grade typing than in TypeScript. Lots of different types for numbers and floats. Tuples are like fixed length arrays of values, where each position in the tuple have a specific type?

An array is a collection of objects **of the same type `T`**. Slices are similar to arrays, but their length is not known at compile time.

Still don't understand

- Shadowing?

### 3. Custom types
