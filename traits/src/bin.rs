use cool_lib::{Area, NewsArticle, Rectangle, Summary, Triangle, Tweet};

pub fn main() {
    let article = NewsArticle {
        author: String::from("Farhan Ghazi"),
        headline: String::from("Farhan Ghazi"),
        content: String::from("Farhan Ghazi"),
        location: String::from("Farhan Ghazi"),
    };
    println!("Article summary: {}", article.summarize());
    let tweet = Tweet {
        username: String::from("@farhanghazi97"),
        content: String::from("Rust is awesome!"),
        reply: false,
        retweet: false,
    };
    println!("Article summary: {}", tweet.summarize());
    let rectangle = Rectangle {
        width: 10.0,
        height: 2.0,
    };
    println!(
        "The default area of this rectangle is: {}",
        rectangle.area()
    );
    println!("{}", rectangle.dimensions());
    let triangle = Triangle {
        base: 10.06,
        height: 20.87,
    };
    println!("The area of the triangle is: {}", triangle.area());
    println!("{}", triangle.dimensions());
}
