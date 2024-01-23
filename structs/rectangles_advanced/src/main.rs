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

    // The "functions" below are better called "methods"
    // since they act upon an instance of a Rectangle

    // Methods must take &self as their first parameter

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This "function" below is not a "method" since it does
    // not act upon an instance of a Rectangle. This function
    // is an example of an "associated function"

    // Notice how the square() function does not take
    // &self as the first argument - this is because it is not
    // a "method" being called on a Rectangle instance

    // Associated functions are often used for constructors
    // that will return a new instance of the struct.

    // Associated functions are invoked directly via the type itself
    // (and not an instance of the type).

    // Associated functions DO NOT take &self as their first parameter!

    // THe "Self" keyword in the return type and in the body
    // of the function are aliases for the type that appears
    // after the "impl" keyword, which in this case is Rectangle

    // To call an associated function. we use the :: syntax
    // with the struct name; similar to String::from()!

    // This function is namespaced by the struct. We'll take a
    // closer look at associated functions and namespaces in
    // future chapters.

    // For example usage, refer to the main() function below!

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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
    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {}", rect1.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // Because we passed the REFERENCE to the rectangles, we can
    // still use them after the println!() and can_hold()
    println!("{:?}, {:?}", rect2, rect3);

    // Associated function
    let square = Rectangle::square(20);
    println!("{:?}", square);
}
