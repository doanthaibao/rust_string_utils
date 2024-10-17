/// Checks if the given string starts with the specified prefix.
///
/// # Arguments
///
/// * `str` - A `String` to check for the prefix.
/// * `prefix` - A `String` representing the prefix to check for.
///
/// # Returns
///
/// * `true` if the string starts with the prefix, otherwise `false`.
pub fn start_with(str: String, prefix: String) -> bool {
    str.starts_with(&prefix)
}

/// Checks if the given string ends with the specified suffix.
///
/// # Arguments
///
/// * `str` - A `String` to check for the suffix.
/// * `suffix` - A `String` representing the suffix to check for.
///
/// # Returns
///
/// * `true` if the string ends with the suffix, otherwise `false`.
pub fn end_with(str: String, suffix: String) -> bool {
    str.ends_with(&suffix)
}

#[cfg(test)]
mod tests {
    use crate::start_with;
    use crate::utils::start_with::end_with;

    #[test]
    fn should_start_with() {
        let result = start_with(String::from("abcde"), String::from("a"));
        assert_eq!(true, result);
        let result2 = start_with(String::from("abcde"), String::from("b"));
        assert_eq!(false, result2);
    }

    #[test]
    fn should_end_with() {
        let result = end_with(String::from("abcde"), String::from("e"));
        assert_eq!(true, result);
        let result2 = end_with(String::from("abcde"), String::from("b"));
        assert_eq!(false, result2);
    }
}