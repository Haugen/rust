use std::fmt::{self, Display, Result};
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // variable `mutable` is assigned to, but never used
    // note: consider using `_mutable` instead

    let _logical: bool = true;
    let _an_integer = 5i32; // Suffix annotation
    let _a_float: f64 = 1.0; // Regular annotation

    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;

    let _num_underscore = 1_000_000;

    // Tuples
    // https://doc.rust-lang.org/rust-by-example/primitives/tuples.html

    // #[derive(Debug)]
    // Only applicable for struct, enum and union?
    let _long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("{:?}", _long_tuple);
    let _tuple_of_tuple = (_long_tuple, (4u64, -1i8));

    // Long Tuples (more than 12 elements) cannot be printed.

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // Reversing a tuple with our helper function
    let _pair = (1, true);
    println!("Reversed pair {:?}", reverse(_pair));

    let _matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // Inherited Debug print trait.
    println!("{:?}", _matrix);
    // Custom Display print trait.
    println!("{}", _matrix);
    // Transpose the matrix.
    println!("Transponse\n{}", transpose(_matrix));

    // Arrays
    // https://doc.rust-lang.org/rust-by-example/primitives/array.html

    // Fixed-size array (type signature is superfluous).
    let _xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "Printing first element of array with length {1}: {0}",
        _xs[0],
        _xs.len()
    );

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&_xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow a section as a slice");
    analyze_slice(&_xs[1..3]);

    // First look at a loop over an array
    for i in 0.._xs.len() {
        println!("Element at index {}: {}", i, _xs[i]);
    }

    // Same loop, but with safety. The loop "panicks" if the index is out of bounds.
    // + 1 to create out of bounds
    for i in 0.._xs.len() + 1 {
        match _xs.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("Slow down!, {} is too far", i),
        }
    }
}
