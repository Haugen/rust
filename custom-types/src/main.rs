// An attribute to hide warnings for unused code.
#![allow(dead_code)]

const CONSTANT: u32 = 123_456;
static LANGUAGE: &str = "Rust";

use std::fmt;
struct Unit;
// Tuple struct, which are, basically, named tuples
#[derive(Debug)]
struct Tripple(bool, String, usize);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    _top_left: Point,
    _bottom_right: Point,
}

enum WebEvent {
    PageLoad,
    Paste(String),
    Click { x: i64, y: i64 },
}
type WE = WebEvent;

enum Status {
    Rich,
    Poor,
}

fn inspect(event: WE) {
    match event {
        WE::PageLoad => println!("Page loaded"),
        WE::Paste(s) => println!("Pasted: {}", s),
        WE::Click { x, y } => println!("Clicked x {}, y {}", x, y),
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        _top_left: Point { x: x1, y: y1 },
        _bottom_right: Point { x: x2, y: y2 },
    } = rect;

    ((x2 - x1) * (y2 - y1)).abs()
}

fn square(point: Point, side: f32) -> Rectangle {
    let Point { x: _x, y: _y } = point;
    Rectangle {
        _top_left: point,
        _bottom_right: Point {
            x: _x + side,
            y: _y + side,
        },
    }
}

fn main() {
    let _name = String::from("WhiteFluffy");
    let _unit = Unit; // Wat?

    let _tripple_tuple = Tripple(true, "Hello".to_string(), 42);
    println!("Tuple struct: {:?}", _tripple_tuple);

    let _point = Point { x: 0.3, y: 0.4 };
    println!("Point struct: {}", _point);

    let bottom_right = Point { x: 5.2, .._point };
    let _rect = Rectangle {
        _top_left: Point { x: 2.4, y: 1.3 },
        _bottom_right: bottom_right,
    };

    println!("Rectangle: {:#?}", _rect);
    // Round to decimal places when printing
    println!("Area: {:.2}", rect_area(_rect));
    println!("Square: {:#?}", square(_point, 3.0));

    // Enums
    // WE is an alias for WebEvent
    let page_load = WE::PageLoad;
    let paste = WE::Paste("Fluffy".to_owned());
    let click = WE::Click { x: 45, y: 123 };
    inspect(page_load);
    inspect(paste);
    inspect(click);

    // Use make enum values available without prefix.
    use crate::Status::*;
    let status = Poor;
    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    // enums can be cast as integers. Rich = 0, Poor = 1.
    println!("test {}", Rich as i32);

    // Accessing constants
    println!("Language: {}, Number: {}", LANGUAGE, CONSTANT);
}
