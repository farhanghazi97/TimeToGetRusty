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
    meal_complete.toast = String::from("wheat");
}
