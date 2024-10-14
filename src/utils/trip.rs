/// Removes all spaces from the given string.
///
/// # Arguments
///
/// * `str` - The input string from which spaces will be removed.
///
/// # Returns
///
/// A new `String` with all spaces removed.
///
/// # Examples
///
/// ```
/// use rust_string_utils::trip;
/// let result = trip(String::from("a b c"));
/// assert_eq!(result, "abc");
/// let result2 = trip(String::from(" a     "));
/// assert_eq!(result2, "a");
/// ```
pub fn trip(str: String) -> String {
    str.chars().filter(|&c| c != ' ').collect()
}

#[cfg(test)]
mod tests {
    use crate::trip;

    #[test]
    fn should_trip() {
        let result = trip(String::from("a b c"));
        assert_eq!("abc", result);
        let result2 = trip(String::from(" a     "));
        assert_eq!("a", result2);
    }
}