fn main() {
    let x = 6;
    if x % 4 == 0 {
        println!("Number is divisible by 4");
    } else if x % 3 == 0 {
        println!("Number is divisible by 3");
    } else if x % 2 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is NOT divisible by 2,3 or 4");
    }
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("The value of result is {result}");
}
