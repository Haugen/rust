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
}
