// We are able to do this because we have re-exported these
// definitions from the parent crate (Refer to "pub use"
// statements within "src/lib.rs")

use export_api::{mix, PrimaryColor};

fn main() {
    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;
    mix(red, blue);
}
