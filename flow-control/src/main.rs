#![allow(unused_variables)]
#![allow(unused_labels)]

fn main() {
    // if/else

    let n = 5;

    // Similar to other languages. No need for parantheses.
    // Each condition is followed by a block.
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    // All branches must return the same type.
    let big_n = if n < 10 && n > -10 {
        // This expression returns an `i32`.
        10 * n
    } else {
        // We must return an `i32` here as well.
        n / 2
    };

    // loop

    // Rust provides a `loop` keyword to indicate an infinite loop.
    let mut count = 0u32;

    loop {
        // Outer loop
        count += 1;

        if count == 3 {
            println!("three");
            // Skip the rest of this iteration with continue.
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop with break.
            break;
        }
    }

    // Nested loops can be labeled. Loops can then continue or break other loops
    // by referring to their label.
    let mut count2 = 0u32;

    // Just like with if statements, we can declare a variable with a return
    // value from a loop.
    let result = 'outer: loop {
        count2 += 1;
        println!("Outer loop count: {}", count2);
        'inner: loop {
            // `break` here breaks the inner loop.
            // We can break the outer loop by referring to its label
            if count2 > 3 {
                // Return a value by putting it after the break.
                break 'outer count2;
            }

            // `continue` the outer loop.
            continue 'outer;
        }
    };
    assert_eq!(result, 4);

    // while

    let mut n = 0;

    println!("While loops");
    // While loops are straight forward. Can also use labels, continue, break,
    // etc. But you can't return a value in a while loop?
    'while_loop: while n < 10 {
        n += 1;
        if n == 2 {
            println!("continue");
            continue 'while_loop;
        }
        if n == 5 {
            break 'while_loop;
        }
        println!("{} ", n);
    }

    // for and range

    println!("for and range loops");
    // for/in loops with a..b, where a is inclusive and b is exclusive.
    // Can also use a..=b to make both a and b inclusive.
    for n in 1..6 {
        println!("{} ", n);
    }

    // for and iterators
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter borrows each element of the collection, leaving the original
    // collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            // & is needed here. Borrow checker? Unclear.
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter consumes the original collection. Once the original collection
    // has been consumed it is no longer available after the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // This wont work. `names` has been `consumed` by `iter_into`.
    // println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    // iter_mut mutably borrows each element of the collection, allowing for
    // the collection to be modified in place.
    for name in names.iter_mut() {
        *name = match name {
            // &mut keyword is needed here as well?
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    // Names has now been modified, because we used `iter_mut` above.
    println!("names: {:?}", names);

    // match

    let number = 8;

    // match is like a switch statement. And can, like loop and if/else, return
    // values and declare a variable.
    let result = match number {
        // Using if statements in a matching arm. Called a match "guard".
        i if number % 2 == 0 => "Even",
        0 | 2 | 4 | 6 | 8 => "Even under 10",
        13..=19 => "A teen",
        _ => "Default",
    };
    println!("result: {}", result);

    // A match block can destructure items in a variety of ways.

    // tuples
    let triple = (1, 2, 3);
    match triple {
        (0, y, z) => println!("First is 0, y is {}, z is {}", y, z),
        (1, ..) => println!("First is 1, and the rest doesn't matter"),
        (.., 2) => println!("Last is 2, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    // arrays/slices
    let array = [1, -2, 6];

    // Has similar properties like tuples, but a few additional ones:
    match array {
        // array[0] is ignored with _
        [1, _, third] => println!("array[0] = 1, array[2] = {}", third),
        [-1, second, ..] => println!("Bind some and ignore the rest"),
        [3, second, tail @ ..] => println!("Store rest in `tail`"),
        [first, middle @ .., last] => println!("Combo!"),
        // _ is unreachable because the line above catches all.
        // _ => println!("It doesn't matter what they are"),
    }

    // Further destructuring patterns are available for enums, structs, and the
    // myterious pointers/ref.

    // match binding

    fn age() -> u32 {
        15
    }
    match age() {
        0 => println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Catch all don't need to be _.
        n => println!("I'm {} years old", n),
    }
}
