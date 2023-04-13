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

### [1. Hello World](./hello-world/src/main.rs)

https://doc.rust-lang.org/rust-by-example/hello.html

First lesson focused on Formatted print. How to print to the console using `println!()`. How to pass in arguments to the macro, using `{}` as a placeholder. Using `{:?}` to `Debug` instead of `Display`. There are many more [formatting traits](https://doc.rust-lang.org/std/fmt/#traits).

Custom structs can be printed with `{:?}` or `{:#?}` by adding `#[derive(Debug)]` to the line above its declaration. This is a [derive macro](https://doc.rust-lang.org/rust-by-example/trait/derive.html). For `Display`, the struct needs to implement the `Display` trait manually (as shown in the example).

### [2. Primitives](./primitives/src/main.rs)

https://doc.rust-lang.org/rust-by-example/primitives.html

`u8`, `u16` etc, are unsigned integers. `i8`, `i16` etc, are signed integers. The difference is that unsigned integers can not hold negative values. For example `u8` can store values from 0 to 255. `i8` can store values from -128 to 127.

Are the signed and unsigned integer types getting deprecated? https://doc.rust-lang.org/std/#modules

Further detailed introduction to all the types. More fine grade typing than in TypeScript. Lots of different types for integers and floats. Tuples are like fixed length arrays of values, where each position in the tuple have a specific type?

An array is a collection of objects **of the same type `T`**. Slices are similar to arrays, but their length is not known at compile time.

### [3. Custom types](./custom-types/src/main.rs)

https://doc.rust-lang.org/rust-by-example/custom_types.html

Rust custom data types are formed mainly through the two keywords:

- `struct`: define a structure
- `enum`: define an enumeration

Constants can also be created via the const and static keywords.

- `const`: An unchangeable value (the common case).
- `static`: A possibly `mut`able variable with `static` lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is `unsafe`.

struct defines data structures. Can be tuples, single units, or objects like C structs with named fields. Enums are way different than in TypeScript. The [linked list example](https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html) is a bit mind bending. Enums can have methods? const is the most straight forward, while static is a bit more complex.

### [4. Variable Bindings](./variable-bindings/src/main.rs)

https://doc.rust-lang.org/rust-by-example/variable_bindings.html

Finally, the `mut` keyword. Simple example is that all variable bindings are **immutable by default**. Prefixing with the `mut` keyword makes a binding mutable.

Scope is similar to TS. Variables have scope within {} blocks. Shadowing occurs when a new variable is declared with the same name as a previous variable. The new variable _shadows_ the previous variable. The first variable is no longer accessible. When shadowing a mutable variable, it's frozen in the current scope.

### [5. Types](./types/src/main.rs)

https://doc.rust-lang.org/rust-by-example/types.html

Rust provides several mechanisms to change or define the type of primitive and user defined types.

- Casting between primitive types. Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.
- Specifying the desired type of literals. Numeric literals can be type annotated by adding the type as a suffix.
- Using type inference. The type inference engine is pretty smart. It looks at how the variable is used afterwards to infer its type.
- Aliasing types. The `type` statement can be used to give a new name to an existing type. The main use of aliases is to reduce boilerplate.

### [6. Conversion](./conversion/src/main.rs)

https://doc.rust-lang.org/rust-by-example/conversion.html

There are built in so called `traits` that helps convert between custom types like `struct` and `enum`. The general traits are `From` and `Into`, but there are specific ones for common cases like converting to and from strings.

TryFrom and TryInto are similar to From and Into, but they return a Result instead of panicking on failure. Seems like we can think of ´Result´ as a JS Promise. It can either "resolve", by returning `Ok`, or "reject", yielding an `Err`.

`unwrap()` comes in to play here as well. It will use the Ok value from a Result. More on this later, I guess.

Converting to String uses the Display trait. We've already seen this implemented in previous sections. Rust can parse strings into other types. Sometimes by inference, but more commonly by specifying the target type.

### [7. Expressions](./expressions/src/main.rs)

https://doc.rust-lang.org/rust-by-example/expression.html

The most interesting thing with Rust here is that blocks of code can be expressions. In variable assignments, a whole block of code can run, and then assign a value.

If the last line in a block does not end with a semicolon, the value of that line will be returned. If the last line **has** a semicolon at the end, () is returned.

### [8. Flow of control](./flow-of-control/src/main.rs)

https://doc.rust-lang.org/rust-by-example/flow_control.html

Since we can use block statements when declaring variables, we can also use if/else statements there. Interesting. Also, each branch in an if/else statement must return the same type.

The `loop` keyword provides an infinite loop than then is controlled with `continue` and `break`. Loops can be `label`ed, and loops can break/continue other loops by referring to their labels. Loops can also return values, and be used as variable declarations just like if/else statements.

`while` loops are similar and straight forward. Can be labeled and can break/continue. But, they can't return values?

Iterating on vectors (arrays) becomes [a bit more complicated](https://doc.rust-lang.org/rust-by-example/flow_control/for.html) than in JS. Collections need to be turned into Iterators with an iterator trait, like `iter`, `into_iter` and `iter_mut`. Examples in code.

Rust provides pattern matching via the `match` keyword. Similar to `switch` in JS. The first matching "arm" is evaluated and all possible values must be covered. `match` provides a ton of [destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html), some of which is covered in the example code. [`pointers/ref`](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_pointers.html) is really unclear atm.
