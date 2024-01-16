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
    let s3 = "Welcome to Rust.";

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

    // String concatenation using + Operator
    let s5 = String::from("Hello");
    let s6 = String::from(", World!");

    // Let's dissect the line 69!
    // Firstly, let's think about what types we are adding.
    // Looking at the types of each individual component, we
    // see that:
    //    s5 has type: String:
    //    s6 has type: &String:
    //    the + operator accepts an argument of type &str:
    //      this explains why we are adding &s6 and not just s6. But wait!
    //      s6 has type &String and not &str. Why isn't the compiler
    //      angry?! This is because in this case Rust knows what we're
    //      trying to do and peforms "deref coercion" (we'll explore this
    //      in depth later). Essentially, it coerces &String to &str!

    //      Also, note that the use of &s6 ensures that we are not taking
    //      ownership of the variable, making it accessible later in our program!

    // HOWEVER, there is one error! Try uncommenting line 79!
    // You'll notice the compiler is complaining...

    // This is expected because using the + operator results in
    // the add() string method taking ownership of s5 (LHS)
    // s5 has moved into s7 and so therefore is no longer valid!

    let s7 = s5 + &s6;
    println!("{s7}");
    println!("{s6}");
    // println!("{s5}");

    // As you can imagine, this can be quite a drag if we are
    // concatenating multiple strings. Thankfully, we have a
    // Rust macro! to our rescue! For complicated string combining,
    // we can instead use the "format!" macro. Let's take a look:
    let s8 = format!("{s7}-{s6}");
    println!("{s8}");

    // The format!() macro works like println!(), but instead of
    // printing output to the screen, it returns a "String" with
    // specified contents. Plus, the format! macro uses references
    // so it dosen't take ownership of any of its parameters!

    // Try uncommenting line 100 - you're in for a surprise!

    // ALERT: Unlike almost every other language, Rust DOES NOT
    // allow indexing into a string!!! BA DUM TSS! Why?
    // There's a fair bit to explain so best bet is to read
    // through here: https://doc.rust-lang.org/book/ch08-02-strings.html

    // let character = s7[0];
}
