use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {num} + 1 is {}!", add_one::add_one(num));
    println!("Hello, world! {num} + 2 is {}!", add_two::add_two(num));
}
