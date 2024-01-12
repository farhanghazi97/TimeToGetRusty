#[derive(Debug)] //Weird I know, don't worry about this - we explain it later
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rect1 is: {}", area(&rect1));
    // Pretty sure the following thought has come across
    // your mind already -- why dont we try printing the
    // contents of a "rectangle" instance. Let's do that
    // right now. But as you can see, we've commented it
    // out because - you guessed it - you'll get an error
    // if you try printing it. Go ahead and uncomment line 24

    // println!("{rect1}");

    // Hover over the above error after uncommenting it, we
    // get a hint saying the following:

    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

    // Let's give the suggestion a try. This time the previous
    // error goes away - but we get another error (hence why
    // this line is also commented out). Try uncommenting it
    // to see the compiler's hint.

    // println!("rect1 is: {:?}", rect1);

    // We get the following note from the compiler:

    // = help: the trait `Debug` is not implemented for `Rectangle`
    // = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`

    // Now let's try this suggestion, the compiler has suggested
    // we add an "outer attribute" called "#[derive(Debug)]"
    // right before the struct definition. Peek at the top of
    // the file - we've added it!

    // Now let's try printing the rectangle instance
    println!("rect1 is: {:?}", rect1);
    // We can even pretty-print its content by adding a
    // modifier (#) to the above format string
    println!("rect1 is: {:#?}", rect1);
}
