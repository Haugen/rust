#![allow(unused_variables)]

fn main() {
    // Casting

    let decimal: f32 = 65.4321;

    // No implicit conversion (coercion). Need to be explicit with `as`.
    let integer: u8 = decimal as u8;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;

    // Casting unsigned integers to other types will add or subtract T::MAX + 1
    // until it fits into the new type.
    // Interesting concept. Works similar to modulus. The following two lines
    // result in the same output, 232. (100 - 256 - 256 - 256 = 232)
    let thousand: u16 = 1000; // 1000 fits into u16
    println!("thousand: {}", thousand as u8);
    println!("thousand: {}", thousand % 256);

    // Casting unsigned to signed results in something else.
    // Not sure what's going on here yet.
    println!("thousand: {}", thousand as i8); // -24

    // Literals

    let i = 1; // Integers defaults to i32 (if possible)
    let f = 1.0; // Floats defaults to f64 (if possible)

    // std is crate
    // mem is module (inside crate)
    // size_of_val is function (inside module)
    println!("Size of i: {}", std::mem::size_of_val(&i));

    // Type inference

    let elem = 5u8;
    // vec first don't know what type it holds.
    let mut vec = Vec::new();
    // When we push elem, it becomes Vec<u8>
    vec.push(elem);

    // Aliasing

    // Custom types must have UpperCamelCase names
    type NanoSecond = u64;
    let nanoseconds: NanoSecond = 5;
}
