/// Compares two strings lexicographically.
///
/// # Arguments
///
/// * `s1` - A `String` to be compared.
/// * `s2` - A `String` to be compared.
///
/// # Returns
///
/// An `i32` value indicating the result of the comparison:
/// * `0` if the strings are equal,
/// * A negative number if `s1` is less than `s2`,
/// * A positive number if `s1` is greater than `s2`.
///
/// # Examples
///
/// ```
/// use rust_string_utils::compare;
/// let result = compare(&"abc".to_string(), &"abc".to_string());
/// assert_eq!(result, 0);
/// ```
pub fn compare(s1: &String, s2: &String) -> i32 {
    s1.cmp(&s2) as i32
}

/// Compares two strings lexicographically, ignoring case differences.
///
/// # Arguments
///
/// * `s1` - A `String` to be compared.
/// * `s2` - A `String` to be compared.
///
/// # Returns
///
/// An `i32` value indicating the result of the comparison:
/// * `0` if the strings are equal,
/// * A negative number if `s1` is less than `s2`,
/// * A positive number if `s1` is greater than `s2`.
///
/// # Examples
///
/// ```
/// use rust_string_utils::compare_ignore_case;
/// let result = compare_ignore_case(&"abc".to_string(), &"Abc".to_string());
/// assert_eq!(result, 0);
/// ```
pub fn compare_ignore_case(s1: &String, s2: &String) -> i32 {
    s1.to_lowercase().cmp(&s2.to_lowercase()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(0, compare(&"abc".to_string(), &"abc".to_string()));
        assert_eq!(compare(&"abc".to_string(), &"Abc".to_string()) > 0, true);
        assert_eq!(compare(&"Abc".to_string(), &"abc".to_string()) > 0, false);
    }

    #[test]
    fn test_compare_ignore_case() {
        assert_eq!(
            0,
            compare_ignore_case(&"abc".to_string(), &"abc".to_string())
        );
        assert_eq!(
            compare_ignore_case(&"abc".to_string(), &"Abc".to_string()) > 0,
            false
        );
        assert_eq!(
            compare_ignore_case(&"Abc".to_string(), &"abc".to_string()) < 0,
            false
        );
        assert_eq!(
            0,
            compare_ignore_case(&"abc".to_string(), &"Abc".to_string())
        );
    }
}
