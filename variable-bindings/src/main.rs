fn main() {
    let unit = ();
    println!("Meet the unit value: {:?}", unit);

    // Mutability
    // Variable bindings are immutable by default. This can be overridden with
    // the mut keyword.
    let _immutable = 1;
    let mut mutable = 1;
    // Ok
    mutable += 1;
    println!("mutable should now be 2: {}", mutable);

    // Variable bindings have a scope and are constrained to live in a block.
    let shadowed_binding = 1;
    {
        // This binding lives in this block. It is not available outside of it.
        let short_lived_binding = 1i8;

        println!("shadowed_binding is {}", shadowed_binding); // 1
        let shadowed_binding = "abc";
        println!("shadowed_binding is {}", shadowed_binding); // abc
    }
    // You can redeclare a variable with the same name? This is also shadowing?
    let shadowed_binding = 2;
    println!("shadowed_binding is {}", shadowed_binding); // 2
    let shadowed_binding = 3;
    println!("shadowed_binding is {}", shadowed_binding); // 3

    // It's possible to declare variable bindings first, and initialize them later.
    // However, this form is seldom used, as it may lead to the use of uninitialized variables.
    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    // Freezing
    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        // _mutable_integer is now frozen in this scope and can not change.
        let _mutable_integer = _mutable_integer;
    }
    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
