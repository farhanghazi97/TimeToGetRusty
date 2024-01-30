# What's a pointer ?

A <i>pointer</i> is a general concept for a variables that contains an address in memory. This address refers to, or "points at", some other data. The most common kind of pointer in Rust is a <i>reference</i>. <i>References</i> are indicated by the `&` symbol and borrow the value they point to. They don't have any special capabilities other than referring to data, and have no compile / runtime overhead.

# Ok, then what's a <i>smart</i> pointer?

Smart pointers are <i>data structures</i> that act like a pointer but also have additional metadata and capabilities.

Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by <i>references</i>.

Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: <i>while references only borrow data, in many cases, smart pointers OWN the data they point to.</i>

Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The `Drop` trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

This example crate will explore the 3 most commonly used smart pointers:

- `Box<T>`: for allocating values on the heap
- `Rc<T>`: a reference counting type that enables multiple ownership
- `Ref<T>` / `RefMut<T>` accessed through `RefCell<T>`: a type that enforces the borrowing rules at <i>RUNTIME</i> instead of <i>COMPILE TIME<I/>.
