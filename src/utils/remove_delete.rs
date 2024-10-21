
/// Removes all occurrences of the specified substring from the given string.
///
/// # Arguments
///
/// * `str` - The `String` from which to remove the substring.
/// * `remove` - The `String` to be removed.
///
/// # Returns
///
/// A new `String` with all occurrences of the specified substring removed.
pub fn remove(str: &String, remove: &String ) -> String {
    str.replace(remove, "")
}

/// Removes all whitespace characters from the given string.
///
/// # Arguments
///
/// * `str` - The `String` from which to remove whitespace characters.
///
/// # Returns
///
/// A new `String` with all whitespace characters removed.
pub fn delete_white_space(str: &String) -> String {
    str.replace(" ", "")
}


/// Removes all occurrences of the specified substring from the given string, ignoring case.
///
/// # Arguments
///
/// * `str` - The `String` from which to remove the substring.
/// * `remove` - The `String` to be removed.
///
/// # Returns
///
/// A new `String` with all occurrences of the specified substring removed, ignoring case.
pub fn remove_ignore_case(str: &String, remove: &String) -> String {
    str.to_lowercase().replace(&remove.to_lowercase(), "")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        assert_eq!(remove(&"abcdef".to_string(), &"c".to_string()), "abdef");
        assert_eq!(remove(&"abcdef".to_string(), &"z".to_string()), "abcdef");
        assert_eq!(remove(&"abcdef".to_string(), &"".to_string()), "abcdef");
        assert_eq!(remove(&"".to_string(), &"a".to_string()), "");
    }

    #[test]
    fn test_delete_white_space() {
        assert_eq!(delete_white_space(&"a b c".to_string()), "abc");
        assert_eq!(delete_white_space(&"a  b  c".to_string()), "abc");
        assert_eq!(delete_white_space(&" abc ".to_string()), "abc");
        assert_eq!(delete_white_space(&"".to_string()), "");
    }

    #[test]
    fn test_remove_ignore_case() {
        assert_eq!(remove_ignore_case(&"abcdef".to_string(), &"C".to_string()), "abdef");
        assert_eq!(remove_ignore_case(&"abcdef".to_string(), &"E".to_string()), "abcdf");
        assert_eq!(remove_ignore_case(&"abcdef".to_string(), &"".to_string()), "abcdef");
        assert_eq!(remove_ignore_case(&"".to_string(), &"A".to_string()), "");
    }
}
