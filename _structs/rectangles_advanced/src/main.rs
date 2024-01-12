// This example builds on top of the "rectangles_basic" example
// Ensure you are across the previous example before
// delving into this one!

#[derive(Debug)] //Weird I know, don't worry about this - we explain it later
struct Rectangle {
    width: u32,
    height: u32,
}

// The impl (implementation) block for "Rectangle"
// captures everything that is associated with the
// "Rectangle" type.

// Unlike before, the function definition has changed -
// we use "&self" which is an alias for (self: &Self).
// The "self" keyword refers to the instance itself
// (pretty much the same as the "self" in Python)
impl Rectangle {
    // The "&"" (ampersand) in "&self" is important.
    // We don't want the method to take "ownership"
    // of the attribute - we simply want the method
    // to read the data in the struct - not write to it!
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // Unlike before, we invoke the "area()" method
    // on this "Rectangle" instance which is now part
    // of the implementation for a "Rectangle" type.
    println!("Area of rect1 is: {}", rect1.area());
}
