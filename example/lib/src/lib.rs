/// Adds two numbers together
///
/// ```
/// /// Validate that add_one works!
/// use lib::add;
/// assert_eq!(add(41, 1), 42)
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
