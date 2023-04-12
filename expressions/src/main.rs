#![allow(path_statements)]
#![allow(unused_must_use)]

fn main() {
    // Variable binding.
    let x = 5;

    // Expression.
    x;
    x + 1;
    15;

    // Blocks are expressions too and can be used as values in assignments.
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;
        // This expression will be assigned to `y`.
        // If the last expression in the block ends with a semicolon, `()` is returned instead.
        x_cubed + x_squared + x
    };

    println!("y is {:?}", y);
}
