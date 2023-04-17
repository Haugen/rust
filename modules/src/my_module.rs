pub mod my_mod_nested;

// use crate::my_module::my_mod::my_mod_nested;

#[allow(dead_code)]
#[allow(unused_variables)]

fn function() {
    println!("called `function()`");
}

pub mod my_mod {
    // Private function
    fn multiply_by_2(x: i32) -> i32 {
        x * 2
    }

    // Public function. Can be called outside this module.
    pub fn double(x: i32) -> i32 {
        multiply_by_2(x)
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Modules can be nested.

    // Call functions in nested module.
    // pub fn call_nested() {
    //     my_mod_nested::public_in_my_mod();
    //     // Wont work. pub(self) makes it private.
    //     // my_mod_nested::public_in_my_mod_nested();
    //     my_mod_nested::public_in_my_mod_super();
    // }

    pub(crate) fn public_in_crate() {
        println!("This is public in the crate.");
    }
    // Nested module follow the same rules for visibility.
    mod private_nested {
        // Parent item will restrict visibility of a child item.
        pub(crate) fn restricted_function() {
            println!("This is public in the crate. Or is it?");
        }
    }

    // structs

    // A public struct, with a public field.
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct, with a private field.
    pub struct ClosedBox<T> {
        contents: T,
    }

    // Implement a constructor for the `ClosedBox` struct. This will allow other
    // modules to create instances of this struct.
    impl<T> ClosedBox<T> {
        // A public constructor method.
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}
