use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

// So this is like a class, kind of? An implementation of a struct?
// These are associated functions. They don't need to be called with an
// instance. They are generally used like constructors.
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

// These are methods. Each function received `Self` which is the type of the
// caller object. In this case `Rectangle`.
impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    // This method required the caller object to be mutable.
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

fn main() {
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // Corner case, early return.
        if rhs == 0 {
            return false;
        }

        // return keyword not needed here. Can't end with semi-colon.
        lhs % rhs == 0
    }

    println!("10 is divisible by 2: {}", is_divisible_by(10, 2));

    let rectangle = Rectangle {
        // Associated functions are called using double colons.
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator. The first argument is `self`,
    // and is implicitly passed.
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::new(1.5, 1.5),
        p2: Point::new(2.0, 2.0),
    };
    println!("Square area before translate: {}", square.area());
    // We mutate here
    square.translate(2.0, 3.5);
    // But this prints the same area? Not sure why.
    println!("Square area after translate: {}", square.area());

    // Cloures

    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing scope.
    // So we need to create closures.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // When the types are not annotated, the compiler will infer them based on
    // the caller.
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // A super simple closure, without arguments, returning 1.
    let one = || 1;
    println!("closure returning one: {}", one());

    // Capturing reference

    let color = String::from("green");
    // The closure "borrows" `color` with `&`. It will remain borrowed until
    // the last usage of `print_color`.
    let print_color = || println!("`color`: {}", color);
    print_color();
    // The original variable can be borrowed immutably again.
    let _reborrow = &color;
    // But it can not be moved (without &) because its borrowed.
    // let _reborrow2 = color; // Error!
    print_color();
    // Here we can move `color` because the closure no longer needs it.
    let _reborrow2 = color;

    // Capturing mutable reference

    let mut count = 0;
    // Inc borrows `count`mutable. inc also needs to be mut.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    // Here we can't reborrow count. A mutable variable can only be mutably
    // borrowed once at a time.
    // let _reborrow = &mut count; // Error
    inc();
    // Here we can reborrow, since the closure is no longer using `count`.
    let _count_reborrowed = &mut count;

    // As input parameters

    // A function that takes a closure as an argument and calls it.
    fn apply<F>(f: F)
    where
        // FnOnce is least restrictive.
        // Fn and FnMut are more restrictive.
        F: FnOnce(),
    {
        f();
    }

    // Function that takes a closure and returns i32.
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // Closure that captures two variables:
    // `greeting` by reference and `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by mutable reference.
        // Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);

        // Manually calling drop forces `farewell` to be captured by value.
        // Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound.
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

    // `add_10` also satisfies `apply_to_3`'s trait bound.
    // A function that satisfies the trait bound can be passed, just like a closure.
    fn add_10(x: i32) -> i32 {
        x + 10
    }
    println!("3 added to 10: {}", apply_to_3(add_10));

    // Using impl here. Same goes for FnMut and FnOnce.
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        // Returning a closure here.
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    fn_plain();

    // Examples with vectors. Same behaviour for arrays.
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // Iterator::any is a function that takes a closure as argument.
    // It implements the FnMut trait.

    // `iter` borrows each element of `vec1` via reference. So vec1 can be referenced
    // again after this.
    println!("Find 2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter` creates a consuming iterator. vec2 is consumed (each value is `move`d)
    // and can not be used again after this.
    println!("Find 2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // We can iter() over vec1 again. This time using find, which returns an Option (Some or None).
    println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    assert_eq!(vec1.iter().find(|&&x| x == 2), Some(&2));

    // A functional approach example in Rust.

    fn is_odd(n: i32) -> bool {
        n % 2 == 1
    }

    // Similar to JavaScript. Nesting functions. Each taking a closure as argument.
    let sum_of_squared_odd_numbers: i32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < 1000)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!(
        "Sum of squared odd numbers under 1000: {}",
        sum_of_squared_odd_numbers
    );
}
