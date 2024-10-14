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

#[cfg(test)]
mod tests {
    use crate::start_with;
    #[test]
    fn should_start_with() {
        let result = start_with(String::from("abcde"), String::from("a"));
        assert_eq!(true, result);
        let result2 = start_with(String::from("abcde"), String::from("b"));
        assert_eq!(false, result2);
    }
}