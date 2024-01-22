use rand::Rng;
use std::{cmp::Ordering, io};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {} instead.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {} instead.",
                value
            );
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

// impl std::cmp::Ord for Guess {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         if self.value < other.value {
//             Ordering::Less
//         } else if self.value > other.value {
//             Ordering::Greater
//         } else {
//             Ordering::Equal
//         }
//     }
// }

// impl std::cmp::PartialOrd for Guess {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.value < other.value {
//             Some(Ordering::Less)
//         } else if self.value > other.value {
//             Some(Ordering::Greater)
//         } else {
//             Some(Ordering::Equal)
//         }
//     }
// }

// impl std::cmp::Eq for Guess {
//     fn assert_receiver_is_total_eq(&self) {}
// }

// impl std::cmp::PartialEq for Guess {
//     fn eq(&self, other: &Self) -> bool {
//         self.value == other.value
//     }
// }

fn main() {
    println!("Guess the number!");

    let secret_number = Guess::new(rand::thread_rng().gen_range(1..=100));

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        // 17th Jan:
        // Ooft, "guess" of type Guess is a custom type
        // and it dosen't look like we know how to compare two
        // instances of a Guess...
        // We will fix this later...with TRAITS!

        // 19th Jan:
        // Have a look at the updated implementation
        // above where we have defined how to compare instances
        // of Guess. We have implemented the relevant traits on the type.

        // 22nd Jan:
        // Another update! This time, we are deriving the
        // traits for PartialEq, Eq, PartialOrd, Ord from the standard
        // library which does the trick!

        // Hence, we should derive behaviour from the base traits (as
        // it often suffices). However, if we wish to change the default
        // behaviour / implementation of these traits, we can override them
        // explicitly like we did before.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // The #[should_panic] annotation asserts if the test
    // causes a panic
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // The "expected" parameter takes in a substring of the error
    // message that should be present in the panic!()
    // The below test panics with an error message containing the
    // expected substring - thus it passes! If the error message
    // is incorrect, this test will fail.

    // THIS IS A REALLY GOOD WAY OF MAKING SURE THAT A TEST
    // PANICS FOR THE "RIGHT / EXPECTED" REASON!
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_with_expected() {
        Guess::new(200);
    }
}
