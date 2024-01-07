fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    loop_example_advanced();
    while_loop_example();
    for_loop_example();
}

fn loop_example_advanced() {
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining - {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}")
}

fn while_loop_example() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Lift Off!")
}

fn for_loop_example() {
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("The value is {element}");
    }
}
