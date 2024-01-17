// Use of generics in struct definitions

#[derive(Debug)]
struct PointA<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointB<T, U> {
    x: T,
    y: U,
}

fn main() {
    let _integer = PointA { x: 1, y: 2 };
    let _float = PointA { x: 3.0, y: 4.0 };
    // Since we have specified the generic type T to define
    // Point<T>, this defintion says that the Point<T> struct
    // is generic over some type T, and the fields "x" and "y"
    // are BOTH that same type. Let's test this out!
    let _mismatch = PointA { x: 5, y: 6.0 };
    let _mismatch = PointA { x: 7.0, y: 8 };
    // Both the "mismatch" variables throw an error since
    // we're trying to to mix types when both fields can
    // only be one the same type!

    // To define a Point struct where x and y are both generics
    // but could have different types, we can use multiple generic
    // type parameters! Take a look at Point B!

    // Now all instances of a Point are allowed! We can use as
    // many generics type parameters in a definition as we'd like,
    // but using too many makes our code hard to read and interpret.
    // Hence, best practice is to use only a few.

    let both_integer_point = PointB { x: 1, y: 2 };
    let both_float = PointB { x: 1.0, y: 2.0 };
    let int_and_float = PointB { x: 1, y: 2.0 };
    let chars = PointB { x: 'A', y: 'B' };

    println!(
        "{:?}-{:?}-{:?}-{:?}",
        both_integer_point, both_float, int_and_float, chars,
    );
}
