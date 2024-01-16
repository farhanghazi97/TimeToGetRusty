// Vectors allow us to store more than one value in a
// single data structure that puts all the values next
// to each other in memory.
// NOTE: Vectors can only store values of the same type.

fn main() {
    // --- Instantiating a vector ---
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];
    let mut v3 = vec![1, 2, 3];
    // --- Updating a vector ---
    v3.push(1);
    // --- Reading elements of a vector ---
    // There are two ways to reference a value
    // stored in a vector: via indexing or using
    // the "get()" method. Both these methods
    // return a different type. Let's explore
    // them with examples.

    // Method 1 - Indexing the vector
    let third_element = &v3[2];
    // Automatic de-referencing in action!!!
    println!("The third element is {third_element}");

    // Method 2 - Using the .get() method
    let second_element = v3.get(1);
    match second_element {
        Some(second_element) => println!("The second element is {second_element}"),
        None => println!("There is no second element in the vector!"),
    }

    // Notice how the return types are different.
    // For method 1, we get back &i32 which gets de-referenced in println!()
    // For method 2, we get back Option<&i32> which is wrapped around
    // an Option<T>

    // Each method lets us decide on how we want our program to
    // behave when something goes wrong, that is, we try and index
    // a value that is outside the range of the existing elements.

    // Let's induce a deliberate error and see how the compiler reacts.
    let _v4 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v4[100];
    // let does_not_exist = v4.get(100);

    // Line 38 (using indexing) causes the program to panic because it
    // references a non-existant element. This method is best used when
    // you want your program to crash if there's an attempt to access
    // an element past the end of the vector

    // Line 39 (using get() method) returns None without panicking.
    // This method is best used if accessing an element beyond the range
    // of the vector may happen occasionally under normal circumstances.
    // By return a Some(&element) we are ensuring type-safety by handling
    // both null and non-null values.

    // Rust's borrow checker enforces ownership and borrowing rules
    // here too (since we're working with references).
    // Let's test it out

    let mut v5 = vec![1, 2, 3];
    // Here we grab an IMMUTABLE REFERENCE to the first item in the vector
    let first_element = &v5[0];
    // We are now attempting to add an element to the same vector
    // for which we are holding a reference to.
    // ALERT - We are violating Rust's 2nd ownership principle - we cannot
    // have mutable and immutable references to the same data!
    // v5.push(4);
    println!("The first element is {first_element}");

    // Iterating over values in a Vector
    let v6 = vec![1, 2, 3, 4, 5];
    for i in &v6 {
        println!("{i}")
    }

    let mut v7 = vec![1, 2, 3, 4, 5];
    for i in &mut v7 {
        *i += 50;
        println!("{i}");
    }

    // Refer to the definition of a vector at the top of the
    // file, we mentioned that vectors can only store values
    // that are of the same type. This can be inconvenient;
    // there are definitely use cases for needing to store a
    // list of items of different types. Fortunately, we can
    // do this by leveraging "enum".

    // The variants of an enum are all defined under the same
    // over-arching type, so when we need one type to represent
    // elements of different types, we can use an enum!

    // Let's take a closer look.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // This is OK since the vector comprises of 3 elements - all
    // of which are of type <SpreadsheetCell>!
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("value")),
    ];

    println!("{:?}", row);
}
