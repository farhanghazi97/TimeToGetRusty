#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The #[test] annotation helps the compiler identify
    // functions that test code
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // Asserts if some expression is true / false
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // Asserts if some expression is true / false
        assert!(smaller.can_hold(&larger) == false)
    }

    #[test]
    fn it_adds_two() {
        // This macro can be used to check for equality / inequality
        // Short-hand for assert!()
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Farhan");
        // assert()! can also be used to print out custom failure messages
        // Any arguements specified after the required arguments are passed
        // along to the format()! macro
        assert!(
            result.contains("Farhan"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
