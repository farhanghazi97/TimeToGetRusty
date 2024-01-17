// Generics allow us to replace specific types with a placeholder
// that represents multiple types to remove code duplication.

// To demonstarte the use of generics in FUNCTIONS, take a look
// at the functions below in their iteration order.

// Function to find the largest number in a list
fn iteration_1(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn iteration_2(list: &[char]) -> &char {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// Notice the <T: std::cmp::PartialOrd>!
// This is a trait - we will discuss this in a bit
// For now, know that comparisons between elements
// of type T are not possible for all values of T
// This tells us that we can only compare
// values that have a type which can be ordered!
// I.e the type has the PartialOrd trait implemented!
// By specifying <T: std::cmp::ParitalOrd> we are
// restricting the types that T can be to only be ones
// that can be ordered
fn iteration_3<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // Let's say we need to find the largest number
    // in a list of numbers.

    // To achieve this, we wrote the function below.
    // Note the parameter type + return type of the function
    let numbers_list = vec![34, 50, 25, 100, 65];
    let result = iteration_1(&numbers_list);
    println!("The largest number is: {result}");

    // Now we have a new requirement - find the largest
    // in a list of chars. To do this, we wrote the function
    // below
    let chars_list = vec!['y', 'm', 'a', 'z'];
    let result = iteration_2(&chars_list);
    println!("The largest char is: {result}");

    // Take a close look at iteration_1() and iteration_2().
    // Their function bodies are exactly the same, however,
    // only their names, input & return type differ (&i32 vs &char)
    // There is still a good amount of code duplication here!

    // We can reudce code duplication further in these scenarios
    // by using generics! Take a look at iteration_3()
    // iteration_3() now makes iteration_1() and iteration_2()
    // reudundant as it can operate on both numbers and chars
    // (or any list that has values that can be ordered!)
    let result = iteration_3(&numbers_list);
    println!("The largest number is: {:?}", result);
    let result = iteration_3(&chars_list);
    println!("The largest char is: {:?}", result);
}
