// Let's dig into traits!!!

// A trait defines functionality a particular type has
// and can share with other types.

// We can use traits to defined shared behaviour in an
// abstract way.

// We can use "trait bounds" to specify that a generic
// type can be any type that has certain behaviour.

// Traits are a way to group method signatures together
// to define a set of behaviours necessary to accomplish
// some purpose.

use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
    // A trait can have multiple methods in its body; the method
    // signatures are listed one per line and each line ends in
    // a semi-colon.
}

// Here, we declare a trait using the "trait" keyword and
// then the trait's name, which is Summary. We've declared
// the trait as "pub" so that crates depending on this crate
// can make use of this trait too.

// NOTE: Unlike methods and associated functions, instead of
// providing an implementation for the  method, we use a
// semicolon.

// Each type that implements the trait must provide its own
// custom implementation of the method defined by the trait!
// The compiler will enforce that any type that has the Summary
// trait will have the method summarize defined with this
// signature exactly.

// Cool, we now know how to create traits. Let's
// now implement them for a type!

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Going back to our above definitions, we mentioned that
// any type that implements a trait must provide its custom
// implementation of the methods defined by the trait.

// Obviously this can prove to be cumbersome. It would instead
// be useful to have default behaviour for some or all of the
// methods in a trait and then opt-in to override each trait
// method

// To provide a  default implementation for a trait method,
// provide a concrete implementation within the trait method
// body.

// Let's have a look at how this can be achieved.
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub trait ShapeOperation {
    // No default - compiler will enforce any types that implement this trait
    // to provide its custom implementation for area() method
    fn area(&self) -> f64;
    // Default implementation for this trait method
    fn dimensions(&self) -> String {
        format!("This method tells you about the dimensions of this shape!")
    }
}

impl ShapeOperation for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // Since we haven't explicitly provided a dimensions() method, it uses the
    // default implementation provided by the trait method
}

impl ShapeOperation for Triangle {
    fn area(&self) -> f64 {
        0.50 * self.base * self.height
    }
    // Here, we are opting to override the default behaviour of the
    // trait method!
    fn dimensions(&self) -> String {
        format!("Triangle | (Base: {}, Height: {})", self.base, self.height)
    }
}

// TRAITS AS PARAMETERS

// We can also use traits to define functions that accept many different types.
// The parameter type in the function below essentially says that this function
// accepts any type that implements the "ShapeOperation" trait.

// Instead of a concrete type for the "item" parameter, we specify the "impl"
// keyword and the trait name.

// Hence, this method will accept our "Rectangle" and "Triangle" types above
// (both of whom are implementors of ShapeOperation)!
pub fn get_shape_area(item: &impl ShapeOperation) -> f64 {
    item.area()
}

// TRAIT BOUND SYNTAX

// The above trait parameter type (i.e &impl ShapeOperation)
// is actually syntactic sugar for the following:

pub fn get_shape_area<T: ShapeOperation>(item: &T) {}

// There are certain nuances with regards to the use of trait's
// as parameters that we need to keep in mind. Let's discuss them
// below:

// In the above function, T can be of any type so long as it
// implements the "ShapeOperation" trait. What if we had more than
// one parameter? Sounds simple enough, let's update our function
// definition (using the shorthand syntax):

pub fn get_shape_area(item1: &impl ShapeOperation, item2: &impl ShapeOperation) {}

// That does it! Our "get_shape_area()" function now accepts
// two parameters of ANY type so long as they both implement
// the ShapeOperation trait.

// So, now we could call the function in the following ways:

pub fn get_shape_area(triangle, rectangle);  // (A)
pub fn get_shape_area(rectangle, rectangle); // (B)
pub fn get_shape_area(triangle, triangle);   // (C)
pub fn get_shape_area(rectangle, triangle);  // (D)

// As you can see our scope of types is very wide here.
// What if we wanted to constrain the call to the SAME TYPE
// for both parameters (B, C)?

// This is where our syntactic sugar hits it's limitation.
// To accomodate the above, we'll need to explicitly make use of
// the trait bound syntax. Let's update our function definition
// to see how we can accomodate this:

pub fn get_shape_area<T: ShapeOperation>(item1: &T, item2: &T) {}

// Ta-da! The GENERIC TYPE "T" specified as the type of "item1" and
// "item2" parameters constrains the function such that the CONCRETE
// TYPE of the value passed as an argument for item1 and item2
// MUST BE THE SAME TYPE! Sick!

// HERE'S ANOTHER REALLY COOL THING YOU CAN DO

// We can also conditionally implement a trait for any type
// that implements another trait! Here's an example to help 
// explain

// Snippet from the standard library...
impl <T: Display> ToString for T {
    fn to_string(&self) -> String {}
}

// The above is what is called a "blanket implementation",
// that is, we can call the "to_string()" method defined by
// the "ToString" trait on any type that implements the 
// "Display" trait!

// Integers implement the "Display" trait, thus why, we can
// call "to_string()" on it!
println!(3.to_string());