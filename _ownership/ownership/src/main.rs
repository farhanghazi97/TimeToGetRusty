fn main() {
    // Pre-determined, fixed size, not mutable, must
    // know during compile time
    let s = "Hello";
    println!("{s}");

    // ---------- EXAMPLE 1 ----------

    // Stored on heap - requsted from memory allocator;
    let mut allocated_s = String::from("Hello");
    allocated_s.push_str(", world");
    println!("{allocated_s}");

    // Over here, we are assigning `allocated_s` to a new variable.
    // In the underlying memory, we do a shallow copy of the data
    // (that is, we only copy metadata pertaining to the variable such
    // as the pointer, length and capacity, NOT the content thats on
    // the heap i.e the actual string)

    // Rust invalidates the first variable (that is, "allocated_s") by
    // calling drop() on the first variable. Hence, only allocated_s_copy
    // has a valid pointer to the data. This process is called a "MOVE".
    let new_allocated_s = allocated_s;

    // Below code throws an error, as per above comment
    // println!("{allocated_s}")

    // This is valid in accordance with Rust's ownership principles.
    // "new_allocated_s" is now the new and only owner of the string
    // (and it's associated metadata)
    println!("{new_allocated_s}");

    // ---------- EXAMPLE 2 ----------

    let new_string = String::from("Hello");
    takes_ownership(new_string);

    // The println!() call below is invalid
    // -- data comes from heap
    // -- data was MOVED into takes_ownership() function
    // println!("{new_string}");

    let number = 10;
    makes_copy(number);

    // This is valid - value was COPIED since its an integer!
    // Copying primitive types don't result in a performance hit
    println!("{number}");

    // ---------- EXAMPLE 3 ----------

    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);

    println!("{s1}");

    // Calling println!() on s2 is invalid here - ownership has been
    // transferred to s3!
    // println!("{s2}")

    println!("{s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}
