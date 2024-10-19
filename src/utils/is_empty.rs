/// Checks if the given string is empty.
///
/// # Arguments
///
/// * `str` - A `String` to check for emptiness.
///
/// # Returns
///
/// * `true` if the string is empty, otherwise `false`.
pub fn is_empty(str: &String) -> bool {
    str.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::is_empty;
    #[test]
    fn should_is_empty() {
        let result = is_empty(&String::from(""));
        assert_eq!(result, true);
    }
}
