fn main() {
    // Lines 22 and 23 are the same!
    // However, we can agree that line 22 is waaaay better

    // Under the hood, Rust actually uses a vector of bytes
    // to represent the string in memory, plus some methods
    // to provide useful functionality on those bytes
    // when they are interpreted as text.

    // THERE IS AN IMPORTANT DISTINCTION TO NOTE

    // The "String" type is provided by Rust's standard
    // library rather than being coded into the core
    // language.

    // On the other hand, "str" is the string type that is
    // part of the core language (and is often seen in it's
    // borrowed form - &str). String literals (like the one
    // on line 24) refer to a part of a string and are known
    // at compile time.

    let mut s1 = String::new();
    let mut s2 = "".to_string();
    let mut s3 = "Welcome to Rust.";

    s1.push_str("Hello, world!");
    s2.push_str("Hello, world!");

    // Try uncommenting line 35. The compiler will throw an error!

    // The variable "s3" is a mutable reference to a "str". This
    // reference can be mutated to point to another "str"
    // but the "str" itself can never change!

    // s3.push_str("");

    // Note the pass by reference here, we don't want
    // to take ownership of the "s1" variable here!
    let mut s4 = String::from(&s1);
    // The .push() method appends a single character!
    s4.push(' ');
    s4.push_str(s3);

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
}
