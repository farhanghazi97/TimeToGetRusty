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
    // These references are valid since we have marked
    // all components up to the function call as
    // "public"
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    // Try uncommenting line _, you'll get an error!
    // This is because the function within the module
    // hasn't been marked as public
    // crate::front_of_house::hosting::seat_at_table();
}

// We can also use "pub" to designate "structs" and "enums" as public,
// but there are a few details extra to the usage of "pub" with
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
        // ASSOCIATED FUNCTION that constructs an instance
        // of "Breakfast" that provides a complete definition
        // of "Breakfast"
        seasonal_fruit: String::from("Cherries"),
    };
    // Now that we have implemented a complete constructor, the
    // compiler is happy here.
    let mut meal_complete = back_of_house::Breakfast::summer("Rye");
    meal_complete.toast = String::from("wheat");
}
