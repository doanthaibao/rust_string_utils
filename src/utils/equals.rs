/// Compares two strings for equality.
///
/// # Arguments
///
/// * `str1` - The first `String` to compare.
/// * `str2` - The second `String` to compare.
///
/// # Returns
///
/// * `true` if the strings are equal, otherwise `false`.
pub fn equals(str1: String, str2: String) -> bool {
    str1 == str2
}


/// Compares two strings for equality, ignoring case.
///
/// # Arguments
///
/// * `str1` - The first `String` to compare.
/// * `str2` - The second `String` to compare.
///
/// # Returns
///
/// * `true` if the strings are equal, ignoring case, otherwise `false`.
pub fn equals_ignore_case(str1: String, str2: String) -> bool {
    str1.to_lowercase() == str2.to_lowercase()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equals() {
        assert_eq!(true, equals("abc".to_string(), "abc".to_string()));
        assert_eq!(false, equals("abc".to_string(), "Abc".to_string()));
    }

    #[test]
    fn test_equals_ignore_case() {
        assert_eq!(true, equals_ignore_case("abc".to_string(), "abc".to_string()));
        assert_eq!(true, equals_ignore_case("abc".to_string(), "Abc".to_string()));
    }
}

