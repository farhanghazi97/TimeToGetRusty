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

// Let's have a look at how this can be achieved.
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

pub trait Area {
    fn area(&self) -> f64;
    // Default implementation for this trait method
    fn dimensions(&self) -> String {
        format!("This method tells you about the dimensions of this shape!")
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // Since we haven't explicitly provided a dimensions() method, it uses the
    // default implementation provided by the trait method
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.50 * self.base * self.height
    }
    // Here, we are opting to override the default behaviour of the
    // trait method!
    fn dimensions(&self) -> String {
        format!("Triangle | (Base: {}, Height: {})", self.base, self.height)
    }
}
