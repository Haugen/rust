// Lots of more formatted print options can be found here:
// https://doc.rust-lang.org/rust-by-example/hello/print.html

use std::fmt::{self, Display, Formatter};

struct StructWithDisplay(i32);

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for StructWithDisplay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "real: {}, imag: {}", self.real, self.imag)
    }
}

fn main() {
    let x: i8 = 5 + 5;
    println!("Hello, world! x = {}", x);
    println!("{0} first argument. {1} second argument", "Hey hey", 10);

    // Using {:?} helps debug inside std::fmt::Display.
    // Gave hint to add "#[derive(Debug)]" above Structure. Which solved it.
    // It automatically creates a debug implementation for the struct.
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
    #[derive(Debug)]
    struct Structure(i32);
    println!("This struct `{:?}` won't print pretty...", Structure(3));

    // Custom struct with Display implementation.
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
    println!("Printing Structure with Display: {}", StructWithDisplay(10));

    // Print to specific decimal places with {:.3}
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // Playing with another custom struct
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
