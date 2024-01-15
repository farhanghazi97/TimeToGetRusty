fn main() {
    let config_max_1 = Some(3u8);
    match config_max_1 {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Let's say we have a value that will match
    // one pattern and that it is safe to ignore
    // all other patterns, in that case, the
    // "if let" statement is useful syntactic sugar

    // In the above example, notice how the second
    // case _ => () is redundant - we are not really
    // doing anything here

    // Our use of the "Option" type in unison with the
    // match statement enforces us to handle all cases
    // BUT this results in more boilerplate code. To
    // "tidy" things up a bit, we can make use of the
    // "if let" clause

    // Let's re-write the above using "if let"!

    let config_max_2 = Some(3u8);
    if let Some(max) = config_max_2 {
        println!("The maximum is configured to be {max}");
    }

    // As you'll notice, we don't get any errors and the
    // compiler does not complain here. This is because,
    // unlike match, "if let" is not exhaustive!
    // The "if let" statement block only runs if the value
    // matches the pattern. Otherwise, it doesn't.

    // We can also include an "else" caluse with an "if let".
    // The block of code that goes with the "else" clause is
    // the same as the block of code that would go with the
    // _ case in the match expression at the top.

    let config_max_2 = Some(3u8);
    if let Some(max) = config_max_2 {
        println!("The maximum is configured to be {max}");
    }
}
