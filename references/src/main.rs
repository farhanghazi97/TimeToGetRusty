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
