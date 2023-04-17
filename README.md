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

In some cases, `match` can be awkward and boilerplate'y. So we have `if`/`let`.

`let`/`else` is stable since Rust 1.65 (We're at 1.68 at the time of writing this). This concept was more complicated and the code example less straight forward. But, it's basically a way to combine `if` and `match` into one statement. The `else` block is only executed if the `if` block fails.

Last one. `while`/`let`, similar to `if`/`let`. The `while` block is only executed if the `let` block succeeds.

### [9. Functions](./functions/src/main.rs)

https://doc.rust-lang.org/rust-by-example/fn.html

Functions are declared using the `fn` keyword. Arguments are type annotated. If the functions returns a value, that too must be types after an arrow `->`. The final expression is used as a return value. Optionally, the `return` keyword can be used to return early, for example inside loops or if statements.

Closures have a lot of detailed and new information. The closures can capture variables from outer scope. They can be captured by value, reference or mutable reference. This can be explicit or inferred. Keyword `move` can be used to force the closure to take ownership of the captured variables.

Not fully grasping the concepts yet. `move` is changing place where the content is stored in memory? `borrow` is a new reference to the already existing content? If a move is possible, then any type of borrow should also be possible. The reverse it not true.

When a closure is used as a parameter in a function, it needs to satisfy one of three traits. In order of decreasing restriction:

- Fn: the closure uses the captured value by reference (&T) Borrowing?
- FnMut: the closure uses the captured value by mutable reference (&mut T) Borrowing mutably?
- FnOnce: the closure uses the captured value by value (T) Moving?

Just like closures, functions can also be used as arguments to other functions. In a function that takes a closure as a parameter, any function that satisfies the trait bound (Fn, FnMut or FnOnce) can also be passed as an argument.

Functions can also return closures. Here, the `move` keyword must be used, because any captures by reference would be dropped as soon as the function exited, leaving invalid references in the closure.

Iterator methods like `any`, `find` and `position` are examples of core functions that take closures as arguments and implements one of these traits.

Rust provides Higher Order Functions (HOF). These are functions that takes one or more functions as arguments and/or produce a more useful function. **HOFs and lazy iterators give Rust its functional flavor**.

Diverging functions are functions that never return. (Explicit return value is `!`) It's different that functions returning `()`. A diverging function panics and never gives back control to the caller. It is used in for example `loop`s and `match`. `continue` or `break` panics and never returns, which is fine. It is also the return type of functions that loop forever (`loop {}`) like network servers or functions that terminate the process (`exit()`).

### [10. Modules](./modules/src/main.rs)

https://doc.rust-lang.org/rust-by-example/mod.html

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules. Modules are declared with the `mod` keyword. By default, items in a module have `private` visibility, but can be overridden with the `pub` modifier. Only public items can be accessed from outside the module.

Modules can be nested, and make use of the `self` and `super` keywords to refer to the current and parent module respectively. They can be organized in a tree-like structure. More in this with crates and cargo?
