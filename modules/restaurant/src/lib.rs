mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Try uncommenting line 18, you'll get an error!
    // This is because the function within the module
    // hasn't been marked as public

    // crate::front_of_house::hosting::seat_at_table();

    // These references are valid since we have marked
    // all components up to the function call as
    // "public"
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    // Having to write out the paths to call functions
    // can feel inconvenient and repetitive though.
    // There is a way to simplify this process: we can
    // create a shortcut to a path with the "use"
    // keyword once, and then use that shorter name
    // everywhere else in scope. Let's try it!

    use crate::front_of_house::hosting;

    // As you can see below, we can now call the
    // "add_to_waitlist()" function with a much shorter
    // path. It's usage is scoped to within the
    // "eat_at_restaurant()" function though.

    // Try moving the "use" statement outside the function.
    // Doing so allows you to update previous function calls
    // as the scope of the "use" statement is larger now.

    hosting::add_to_waitlist();

    // BEST PRACTICES

    // When calling FUNCTIONS that are not LOCALLY defined,
    // the "use" statement should bring in the PARENT MODULE
    // into scope. This way we gain the following benefits:

    //     - we have access to all public functions via a shorter path
    //     - explicitly specifying the parent module makes it clear
    //       that this function is not locally defined

    // On the other hand, when bringing in STRUCTS, ENUMS and
    // other items with "use", it's idiomatic to specify the
    // FULL PATH. For example:

    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 2);

    // The execption to this idiom is when we bring in two items
    // with the same name into scope with "use" statements.
    // Rust does not allow this. To rectify this, we can differentiate
    // the items via their parent module.

    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
    }
    fn function2() -> io::Result<()> {
        //  --snip--
    }

    // Another solution is to bring in the two items with an
    // alias using the "as" keyword:

    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function3() -> Result {
        // --snip --
    }
    fn function4() -> IoResult<()> {
        // --snip --
    }
}

// We can also use "pub" to designate "structs" and "enums" as public,
// but there are a few extra details to the usage of "pub" with
// "structs" and "enums".

// Similar to using "pub" with "mod", marking a struct as public with
// (pub struct _____) makes the struct public, but it's fields
// will still be private. We can make each field public or not on a
// case-by-case basis.

// Refer to the example below for a better understanding.
// "toast" is a public field but "seasonal_fruit" is private.
// "Breakfast" can be constructed and requires a value for
// "toast" to be specified, but "seasonal fruit" is private
// and its details are internal to the struct defintion and
// cannot be changed / accessed
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Unlike "mod" and "struct", marking an enum as
    // public makes all of its variants public
    pub enum Appetizer {
        Soup(String),
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// RE-EXPORTING NAMES WITH "pub use"

// When we bring a name into scope using the "use" keyword, the name
// available in the new scope is PRIVATE. To enable the code that calls
// our code to refer to that name as if it had been defined in that
// code's scope, we can combine "pub" with "use". This technique is
// called "re-exporting" because we're bringing an item into scope but
// also making that item available for others to bring into their scope.
pub use crate::back_of_house::Appetizer;

// Short-hand for imports

// As you can see, importing multiple items from the same crate
// or module can result in a lot of vertical space in our files.
use std::fmt::DebugList;
use std::fmt::DebugMap;

// To minimise this, we can use nested paths to bring the same
// items into scope using only one line.
use std::fmt::{DebugList, DebugMap};

fn main() {
    // Let's now try constructing an instance of a "Breakfast"
    let meal_incomplete = back_of_house::Breakfast {
        toast: String::from("Rye"),
        // Notice the error here - "seasonal_fruit" is a private
        // field and not accessible! Since our struct definition
        // has a private field, we MUST provide a PUBLIC
        // ASSOCIATED FUNCTION that provides a complete instance
        // of "Breakfast".
        seasonal_fruit: String::from("Cherries"),
    };
    // Now that we have implemented a complete constructor, the
    // compiler is happy here.
    let mut meal_complete = back_of_house::Breakfast::summer("Rye");
    let side_meal = back_of_house::Appetizer::Soup(String::from("Chicken Corn Soup"));
    // Since meal_complete is mutable, we can update its attributes like so:
    meal_complete.toast = String::from("wheat");
}
