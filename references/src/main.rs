// As we go through to following examples, try and relate them to
// the core ownership principles of Rust highlighted below:
//   - Each value in Rust has an "owner"
//   - There can only be one owner of a value at a time
//   - When the owner goes out of scope, the value will be dropped

fn main() {
    // Ask memory allocator for memory to hold the below string
    let s1 = String::from("Hello");

    // Pass a reference (&s1) of the string instead of the actual
    // string to "calculate_length()".
    let length = calculate_length(&s1);

    // Since a reference of s1 was passed, we can still use it here!
    println!("The length of {s1} is {length}");

    // Reference CANNOT be modified here!
    modify_reference(&s1);

    let mut s2 = String::from("Hello");
    // Reference CAN be modified here! Have a look at the
    // println!() following it - s2's value was modified
    // by the function and an explicit return was NOT required!
    modify_mutable_reference(&mut s2);
    println!("Modified value via reference: {s2}");

    let mut s3 = String::from("Hello");
    let _r1 = &mut s3;
    // Try uncommenting the lines 40, 41 - the compiler will
    // complain! This has to do with the 2nd ownership principle
    // of Rust (refer at top of file)

    // You can only ever has one mutable reference to a value at a time!

    // Between the creation of this mutable reference and its usage,
    // we are attempting to create another mutable reference to the same
    // value. This goes against the 2nd ownership principle of Rust!

    // let r2 = &mut s3;
    // println!("{r1}, {r2}")

    // However, we can use curly brackets to create a new scope,
    // allowing for multiple references (just not simultaneous ones)
    // as seen below

    let mut s4 = String::from("Hello");

    {
        let _r1 = &mut s4;
    } // r1 is dropped after this point, so r2 is the new (and only) owner
    let _r2 = &mut s4;

    // Another example demonstarting an important ownership concept
    let mut _s5 = String::from("Hello");
    let r1 = &_s5;
    let r2 = &_s5;
    // Try commenting out the line 60 - the complier will throw
    // an error. This is because we cannot have a mutable and
    // immutable reference to the same value. However, any number
    // of immutable references are allowed!
    // let r3 = &mut _s5;
    println!("{r1}, {r2}");
    // This is valid because the r1, r2 references were used by the
    // println!() and, as such, they were dropped after exiting the
    // function
    let r3 = &mut _s5;
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it
  // does not have ownership of what it refers
  // to, it is NOT dropped!

fn modify_reference(_s: &String) {
    // Try uncommenting the below line!
    // It throws an error - this is because
    // references are immutable by default
    // just like variables!
    // _s.push_str(", world");
}

fn modify_mutable_reference(s: &mut String) {
    s.push_str(", World");
}
