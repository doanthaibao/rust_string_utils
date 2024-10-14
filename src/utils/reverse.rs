/// Reverses the characters in the given string.
///
/// # Arguments
///
/// * `str` - A `String` to be reversed.
///
/// # Returns
///
/// * A new `String` with the characters in reverse order.
pub fn reverse(str: String) -> String {
    str.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::reverse;
    #[test]
    fn should_reverse() {
        let result = reverse(String::from("abcde"));
        assert_eq!("edcba", result);
        let result2 = reverse(String::from("a"));
        assert_eq!("a", result2);
    }
}