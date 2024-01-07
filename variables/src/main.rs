fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // Example of variable shadowing (pretty cool)
    let y = 10;
    let y = y + 2;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is {y}");
    // We can use the "let" keyword to reassign a value to
    // the same variable which can be of a different type!
    let spaces = "     ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces in the line");
    // Try uncommenting the next 2 lines -- they will throw an error
    // Mutable variables CANNOT have their types changed
    // let mut mutable_spaces = "   ";
    // mutable_spaces = mutable_spaces.len()
}
