fn main() {
    // When we call panic!(), Rust will print a failure
    // message, unwind, clean up the stack, and quit.
    // Using an environment variables, we can also
    // instruct Rust to display the call stack when a
    // panic occurs to make it easier to track down the
    // source of the panic.

    // Try uncommenting line 19 and run the program.
    // The console will display the message passed to our
    // panic!() and print out the line where our panic()!
    // occured!

    // OUTPUT
    // thread 'main' panicked at 'crash and burn', src/main.rs:17:5
    // where 19 = line number
    // and 5 = character index

    // panic!("Crash and burn!");

    // Now let's look at another example to see what its like when
    // a panic! call comes from a library because of a bug in our
    // code instead of us calling the macro directly.

    // In the example below, we attempt to access an index in a
    // vector beyond the range of its valid indexes.
    let v = vec![1, 2, 3];
    v[99];

    // The above code causes our program to panic and we exit
    // immediately. Unlike C, Rust prevents undefined memory
    // access upfront (getting rid of a plethora of potential
    // bugs + providing greater security)

    // Now let's observe the BACKTRACE. A backtrace is a list of
    // all functions that have been called to get to the point
    // where our program panic'd!

    // The key to reading the backtrace is to start from the top
    // and read until see files we wrote. That's the spot where the
    // panic! originated.

    // PROTIP: The lines ABOVE that spot are code that your code called;
    // PROTIP NOTE: The lines BELOW that spot are code that called your code!

    // PROTIP: The place to start to debug the panic is from the location pointed
    // to by the first line mentioning a file we wrote in the backtrace!
}
