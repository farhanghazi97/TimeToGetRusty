use rand::Rng;
use std::{cmp::Ordering, io};

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100. Got {value} instead!");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl std::cmp::Ord for Guess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value < other.value {
            Ordering::Less
        } else if self.value > other.value {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl std::cmp::PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value < other.value {
            Some(Ordering::Less)
        } else if self.value > other.value {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl std::cmp::Eq for Guess {
    fn assert_receiver_is_total_eq(&self) {}
}

impl std::cmp::PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

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

        // Ooft, "guess" of type Guess which is a custom type
        // and it dosen't look like we know how to compare two
        // instances of a Guess...
        // We will fix this later...with TRAITS!

        // 19th Jan - Have a look at the updated implementation
        // where we can defined how to compare instances of Guess
        // We have implemented the relevant traits on the type.

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
