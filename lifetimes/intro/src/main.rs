// "Lifetimes" are another kind of generic that we've already
// been using. Rather than ensuring a type has the "behaviour"
// we want, lifetimes ensure that "references are valid as long
// as we need them to be"...

// Tad bit confusing, we'll use examples to explain

fn main() {
    // One detail we didn't mention when we looked at
    // references and borrowing is that:

    // every reference in Rust has a "lifetime", which is the
    // scope for which that reference is valid.

    // Most of the times, lifetimes are implicit and inferred,
    // just like most of the times, types are inferred. We must
    // only annotate types when multiple types are possible.
    // Similarly, we must annotate lifetimes of references when
    // the lifetimes of references could be related in a few
    // different ways.

    // Rust requires us to annotate the RELATIONSHIPS using GENERIC
    // LIFETIME PARAMETERS to ensure the actual references used at
    // runtime will definitely be valid.

    // The main aim of lifetimes is to prevent "dangling references".

    // A "dangling reference" is when a reference points to some data
    // other than the data it's intended to reference.

    // Let's create a "dangling reference" to make it clear.

    let r;
    {
        let x = 5;
        r = &x;
    } // x gets dropped here
    println!("r: {}", r); // r is referencing x which as this point has been lost
                          // But 'r' itself is still valid since it hasn't gone out of scope yet

    // Note the error above:

    // - 'x' does not live long enough
    // - borrowed value does not live long enough

    // If you think about it, it does make sense if we interpret the
    // above in terms of Rust's scope and ownership principles.

    // Since 'x' is scoped within the inner {}, when we attempt to print
    // the value of 'r' (which has a reference to x), this throws an error
    // because the data it references has gone out of scope prior to calling
    // the println()! macro

    // Let's rectify this

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // This is OK because Rust knows the reference in 'r' will always be
    // valid while 'x' is valid
}
