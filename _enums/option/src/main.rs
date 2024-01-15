// Rust does not have a notion of "null" - instead it
// enforces the handling of a null value using the handy
// "Option" type from the standard library.

// "match" statements are exhaustive - try removing either
// the "None" arm or the "Some" arm, the compiler will be
// upset!

// This entire ideology and enforcement by the compiler
// avoids the whole issue of "assuming we have a value
// when we might actually have null"

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // The pattern below basically says that if we
        // get a value that dosent match any pattern
        // in a previous arm, we don't want to run any
        // code
        // _ => (),
    }
}
