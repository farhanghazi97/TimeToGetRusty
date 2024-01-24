// The Iterator pattern allows you to perform some task on a sequence
// of items in turn. An iterator is responsible for the logic of iterating
// over each itme and determining when the sequence has finished. When you
// use iterators, you do not have to re-implement that logic yourself.

// In Rust, iterators are "lazy", that is, they have no effect until you
// call methods that consume the iterator to use it up.

// All Iterators implement a trait named Iterator that is defined in the
// standard library.

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }

    // ---------- Methods that CONSUME the iterator i.e take ownership ----------

    // The following code is all commented out - this is because
    // when we iterate over the iterator, the Iterator trait
    // internally calls the next() method on it, which is a
    // "consuming adaptor" which takes ownership of the values
    // passed into next()

    // "v1_iter" is invalid beyond this point

    // println!("{:?}", v1_iter);

    // the .sum() method is another method implemented on the
    // Iterator trait (one amongst many!)

    // let total: i32 = v1_iter.sum();
    // println!("Total: {total}");

    // ---------- Methods that PRODUCE other iterators (do not take ownership)

    // "Iterator adaptors" are methods defined on the Iterator trait
    // that DO NOT consume the iterator. Instead, they produce different
    // iterators by changing some aspect of the original iterator.

    let v2 = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    assert_eq!(v3, [2, 3, 4]);
}

mod tests {

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        // The Iterator trait only requires implementors to define
        // one method: next(). This method returns one itme of the
        // iterator at a time wrapped in "Some()"" and, when the iteration
        // is over, returns "None"

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
}
