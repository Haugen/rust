#![allow(unused_variables)]
#![allow(dead_code)]

use std::convert::{From, TryFrom};
use std::fmt::Display;

#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug, PartialEq, Default)]
struct EvenNumber(i32);

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

impl Display for EvenNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EvenNumber({})", self.0)
    }
}

fn main() {
    // Casting
    // From and Into

    // Converting a str to a String.
    // str, also called a "string slice" is the most primitive string type.
    let my_str = "hello";
    let my_string = String::from(my_str);

    // Using From traits on our custom struct
    let num = Number::from(30);
    println!("Using From trait {:?}", num);

    // Using .into() is using the Into trait. It will typically require that we
    // specify the type.
    let int = 5;
    let num2: Number = int.into();
    println!("Using Into traits is {:?}", num2);

    // TryFrom and TryInto are used for fallible conversions, and return Results.

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Using the Display trait to convert to a string.
    println!("My number: {}", EvenNumber(4));

    // Parsing a String
    // We can specify the type we want "parse" to parse into.
    let parsed: i32 = "5".parse().unwrap();
    // "Turbofish" syntax.
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum of our strings: {}", sum);
}
