//! # Documentation Crate
//!
//! 'documentation' is a collection of example functions
//! demonstarting how to write documentation comments!

/// Adds two numbers
///
/// # Examples
///
/// ```
///     let arg1 = 5;
///     let arg2 = 6;
///     assert_eq!(11, documentation::add(arg1, arg2));
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
