pub(in crate::my_module) fn public_in_my_mod() {
    println!("This is only public in the `path`. In `my_mod`");
}

pub(self) fn public_in_my_mod_nested() {
    println!("This is only public in `my_mod_nested`. Same as private");
}

pub(super) fn public_in_my_mod_super() {
    println!("With super it's only public in the parent module.");
}

fn function() {
    println!("called `my_mod::my_mod_nested::function()`");
}

// Using self, super, or module name to call a specific function.
pub fn indirect_call() {
    // Both refer to the function declared in this module.
    self::function();
    function();

    // Refer to the function declared in the parent module.
    // super::function();
}
