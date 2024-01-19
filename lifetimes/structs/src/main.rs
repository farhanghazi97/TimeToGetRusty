// We can define structs to hold references, but in that case we would
// need to add a lifetime annotation on every reference in the struct's
// definition.

use std::fmt::Display;

pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }
    pub fn longest_with_an_announcement<T>(&self, x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement!: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

// 'a is the lifetime specifier
// T is the generic type parameter

fn main() {
    let novel = String::from("Call me Farhan. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Level: {}", i.level());
    let value = i.longest_with_an_announcement("abcd", "xyz", 10);
    println!("Value from announcement: {}", value);
}
