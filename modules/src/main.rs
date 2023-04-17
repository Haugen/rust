// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my_module;

// We can grab the items we want to use directly from the module.
// Use `as` to rename the item.
use crate::my_module::my_mod::{double as double_me, ClosedBox, OpenBox};

#[allow(unused_variables)]

fn main() {
    let number = double_me(10);
    println!("The number is {}", number);

    let open_box = OpenBox {
        contents: "public information",
    };

    // This wont work. Even though the struct is public, the field is private.
    // let closed_box = ClosedBox {
    //     contents: "classified information",
    // };

    // We have to use the `new` constructor we created with `impl`.
    let closed_box = ClosedBox::new("Classified information");

    {
        // `use` bindings have local scope.
        use crate::my_module::my_mod::double;
        let number = double(15);
        println!("The number is {}", number)
    }

    // Making a direct call to a nested function.
    my_module::my_mod_nested::indirect_call();
}
