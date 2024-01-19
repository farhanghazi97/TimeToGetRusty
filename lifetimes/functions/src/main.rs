// Let's now take a look at generic lifetimes in functions
// We're going to write a function that returns the longer
// of two string slices.

// The function will take two string slices and return a single
// string slice.

// Try hovering over the error - it says the following:

// - missing lifetime specifier
// - this function's return type contains a borrowed value,
//   but the signature does not say whether it is borrowed
//   from `str1` or `str2`

fn longest_iteraiton_1(str1: &str, str2: &str) -> &str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// When we’re defining this function, we don’t know the concrete
// values that will be passed into this function, so we don’t know
// whether the if case or the else case will execute. We also don’t
// know the concrete lifetimes of the references that will be passed
// in, so we can’t look at the scopes. The borrow checker can’t determine
// this either, because it doesn’t know how the lifetimes of x and y
// relate to the lifetime of the return value. To fix this error, we’ll
// add generic lifetime parameters that define the relationship between
// the references so the borrow checker can perform its analysis.

// Lifetime annotations don’t change how long any of the references live.
// Rather, they describe the relationships of the lifetimes of multiple
// references to each other without affecting the lifetimes. Just as functions
// can accept any type when the signature specifies a generic type parameter,
// functions can accept references with any lifetime by specifying a generic
// lifetime parameter.

// Lifetime annotations have a slightly unusual syntax: the names of lifetime
// parameters must start with an apostrophe (') and are usually all lowercase
// and very short, like generic types. Most people use the name 'a for the first
// lifetime annotation. We place lifetime parameter annotations after the & of
// a reference, using a space to separate the annotation from the reference’s type.

// LIFETIME ANNOTATIONS IN FUNCTION SIGNATURES

// To use lifetime annotations in function signatures, we need to declare the generic
// lifetime parameters inside angle brackets between the function name and the parameter
// list, just as we did with generic type parameters.

// We want the signature to express the following constraint: the returned reference
// will be valid as long as both the parameters are valid. This is the relationship
// between lifetimes of the parameters and the return value. We’ll name the lifetime 'a
// and then add it to each reference,

// Let's update our longest() function to account for lifetime specifiers

fn longest_iteraiton_2<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_iteraiton_1(string1.as_str(), string2);
    println!("The longer string is: {}", result);

    let result = longest_iteraiton_2(string1.as_str(), string2);
    println!("The longer string is: {}", result);
}
