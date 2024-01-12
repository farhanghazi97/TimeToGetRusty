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
    // Try uncommenting line 23, the compiler will throw
    // an error. This is because this attribute is a pointer
    // to data owned by something else. For this instance
    // of the struct to have a reference to foreign data,
    // it must make use of "lifetimes" -- a concept we will
    // discuss in future examples.

    // foreign_attribute: &str,
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
    // How to create instances of a struct
    let _user1 = create_user(
        String::from("farhansghazi@outlook.com"),
        String::from("farhanghazi97"),
    );
}
