// A "Tuple Struct" - a struct without named fields
// to create different types
struct Location(i32, i32, i32);

struct User {
    active: bool,
    // Notice the types for "username" and "email".
    // The choice of "String" type is deliberate -- we
    // want instances of this struct to OWN all of its
    // data.
    username: String,
    email: String,
    sign_in_count: u64,
    location: Location,
    // This is an interesting one
}

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        sign_in_count: 0,
        location: Location(0, 0, 0),
        email,
        username,
    }
}

fn main() {
    let user1 = create_user(
        String::from("farhansghazi@outlook.com"),
        String::from("farhanghazi97"),
    );
    println!("Hello, world!");
}
