// Closures can capture values from their environment in 3 ways:

//   - borrowing immutably
//   - borrowing mutably
//   - taking ownership

// The closure will decide which of these to use based on what the
// body of the function does with the captured values.

fn main() {
    // --------------- Borrowing immutably  ---------------

    // The example illustrates that a variable can bind to a closure
    // definition, and we can later call the closure by using the
    // variable name with parentheses as if the variable name were
    // a function name i.e only_borrows();

    // Because we have multiple immutable references to a "list" at the
    // same time, we can still access the code:

    // - BEFORE the closure is defined
    // - BEFORE the closure is called
    // - AFTER the closure is called

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // --------------- Borrowing mutably ---------------

    let mut list = vec![1, 2, 3];
    println!("Befor defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(4);

    // Try uncommenting this line! The complain won't be happy. Let's
    // discuss why.

    // When "borrows_mutably()" is defined, it captures a MUTABLE reference
    // to "list". We don't use the closure again after the closure is called,
    // so the mutable borrow ends. Between the closure definition and the
    // closure call, an immutable boorrow to println()! isn't allowed
    // because we cannot have multiple mutable references to the same data -
    // this is violation of Rust's ownership principle.

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // --------------- Taking ownership  ---------------

    // If we explicitly want the closure to take ownership of the values it
    // uses in its environment, we can use the "move" keyword before the
    // parameter list

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut takes_ownership = move || list.push(4);

    takes_ownership();

    // Try uncommenting
    // This is invalid - since we have explicitly "moved" list into the closure,
    // it is owned by the closure and thus cannot be accessed beyond this point

    // println!("After calling closure: {:?}", list);
}
