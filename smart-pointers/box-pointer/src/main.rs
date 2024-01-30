// The most straightforward smart pointer is a box, whose type is
// written Box<T>. Boxes allow you to store data on the heap rather
// than on the stack. However, the Box<T> pointer itself is stored
// on the stack and simply points to the heap data.

// Boxes don't have performance overhead, other than storing their
// data on the heap instead of the stack.

// Here are some of the most common use cases for a Box<T> pointer:

// - When you have a type whose size can’t be known at compile time
//   and you want to use a value of that type in a context that
//   requires an exact size (UC1)

// - When you have a large amount of data and you want to transfer
//   ownership but ensure the data won’t be copied when you do so (UC2)

// - When you want to own a value and you care only that it’s a type
//   that implements a particular trait rather than being of a specific type (UC3)

fn main() {
    // Let's explore the UC1 above.

    // Let's say we want a data structure that is of a recursive type, that is,
    // a type that can have another value of the same type as a part of itself.

    // Recursive types pose an issue because at compile time Rust needs to know
    // how much space such a type will take up. However, the nesting of values of
    // recursive types could theoritically be infinite, so Rust can't know how much
    // space the value needs.

    // Because boxes have a known size, we can enable recursive types by inserting a
    // box in the recursive type defnition.

    // Let's explore an example of a recursive type that is commonly
    // found in functional programming languages: cons list!
    //
    // The cons list is a data structure that comes from the Lisp programming language
    // and its dialects and is made up of nested pairs - it is essentially Lisp's
    // implementation of the classic "linked list"

    #[derive(Debug)]
    enum List {
        Cons(i32, List),
        Nil,
    }

    // Note the error above - it says that the recursive type "List" has inifnite size -
    // this is because we've defined List with a variant that is recursive. It holds
    // another value of itself directly. As a result, Rust can't figure out how much space
    // it needs to store a "List" value.

    // Let's fix this now

    #[derive(Debug)]
    enum RecursiveList {
        Cons(i32, Box<RecursiveList>),
        Nil,
    }

    // By wrapping the inner variant (which is the recursive attribute - "RecursiveList") in
    // a <Box<T>, we are saying that Cons contains an i32 value + a POINTER to a RecursiveList
    // stored on the heap.

    // Because Box<RecursiveList> is a pointer, Rust always knows how much space a Box<T> needs:
    // a pointer's size does not change based on the amount of data it is pointing to!

    // From a data structure prespective, the next Cons() is placed NEXT TO the previous Cons()
    // in memory on the heap

    use RecursiveList::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);

    // The Box<T> type is a smart pointer because it implements the "Deref" trait, which allows
    // Box<T> values to be treated like references.

    // When a Box<T> value goes out of scope:
    //     - the memory required by the pointer itself is deallocated
    //     - the heap data that the box is pointing to is deallocated (via the "Drop" trait
    //       implementation)
}
