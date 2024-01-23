// Rust's closures are anonymous (non-named) functions that can be saved in a
// variable or passed as arguments to other functions. A closure can be created
// in one place and then be called elsewhere to evaluate it in a different context.
// Unlike functions, closures can capture values from the scope within which they're
// defined. We will use the example below to demonstarte how closure features allow
// for code re-use and behaviour customisation.

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // LET'S ZONE IN ON THE "giveaway()" METHOD

    // In the "giveaway" method, we get the user preference as a parameter of
    // type "Option<ShirtColor>" which can be:

    //   - Some(ShirtColor)
    //   - None

    // Calling the "unwrap_or_else() method on "user_preference" i.e calling
    // "unwrap_or_else()" on Option<T> takes one argument which is a closure
    // with no arguments that returns a value T (in this case, ShirtColor
    // returned by self.most_stocked())

    // If the Option<T> is the Some variant, "unwrap_or_else()" calls the
    // closure and returns the value T from within the Some. If Option<T>
    // is the None variant, "unwrap_or_else()" calls the closure and returns
    // the value returned by the closure (in this case, it calls and returns
    // the value from "self.most_stocked()")

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // || represents the argument list for the closure
        // self.most_stocked() is the closure body
        // If the closure took any arguments, they would go between the ||
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_shirts = 0;
        let mut blue_shirts = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_shirts += 1,
                ShirtColor::Blue => blue_shirts += 1,
            }
        }
        if red_shirts > blue_shirts {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user1_pref = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user1_pref);
    println!(
        "The user with preference {:?} gets {:?}",
        user1_pref, giveaway1
    );
    let user2_pref = None;
    let giveaway2 = store.giveaway(user2_pref);
    println!(
        "The user with preference {:?} gets {:?}",
        user2_pref, giveaway2
    );
}
