fn main() {
    // Referencing a function from an already compiled library.
    // Not imported here, but compiled with `rustc src/main.rs --extern rary=library.rlib`.
    rary::public_function();
}
