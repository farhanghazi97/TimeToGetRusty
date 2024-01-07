fn main() {
    println!("Hello, world!");
    another_function(10);
    print_labeled_measurements(5, 'h');
    let x = five();
    println!("The value of x is {x}");
    let y = plus_one(x);
    println!("The value of y is {y}");
}

fn another_function(number: i32) {
    println!("The provided number is: {number}")
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
