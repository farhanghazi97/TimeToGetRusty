// The example below demonstrates some error prone code
// which we will later fix using "slices"

// The "first_word()" function is simple -- it traverses
// a string element by element and returns the first index
// where it finds an instance of a space

// The call to first_word() on line 25 works as expected.
// However, we then modify the original string that was
// passed to the function by clearing it. Hence,
// the index value previously returned is now invalid
// since the original string is empty!

// Rust throws no errors even though the code is
// logically incorrect! We have data that is out of
// sync with the state of the string!

// To solve this problem, we will utilise string "slices".
// Refer now to the code after lines 28 for an
// in-depth look.

fn main() {
    let mut s1 = String::from("Hello World");
    println!("String: {s1}");
    let first_word = first_word(&s1);
    s1.clear();
    println!("String: {s1} -- this is now empty");
    println!("Ending index: {first_word} -- this is incorrect!");

    // A string "slice" is a reference to a part of a String type!
    let s2 = String::from("Hello World");

    let hello1 = &s2[0..5];
    let hello2 = &s2[..5];

    let world1 = &s2[6..11];
    let world2 = &s2[6..];

    println!("{hello1} {world1}");
    println!("{hello2} {world2}");

    // Using our new understanding of slices, let optimize our
    // first_word() function!
    let first_word_optimized = first_word_optimized(&s2);

    // Notice how the optimized function actually returns a
    // "slice" reference (as denoted by the return type of "&str")
    // We then print the contents at this slice address by
    // de-referencing the pointer in the println!() call!
    println!("First word: {first_word_optimized}");
    // Try uncommenting line 65. Unlike before, we get a
    // compiler error! Ta-da!!!

    // REASON: This produces an error because it is a violation
    // of the ownership principle of Rust, that is, we cannot
    // have a mutable and immutable reference to the same piece
    // data at the same time!

    // But why is this ???

    // When .clear() is called below, it needs a mutable reference.
    // However, since the call to println!() on line 50 is still
    // using the active immutable reference from the variable on
    // line 44 (first_word_optimised) we end up violating the rule!
    // s2.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_optimized(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
