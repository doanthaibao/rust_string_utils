/// Checks if the given string is blank (contains only whitespace characters).
///
/// # Arguments
///
/// * `str` - A `String` to check for blankness.
///
/// # Returns
///
/// * `true` if the string is blank, otherwise `false`.
pub fn is_blank(str: String) -> bool {
    str.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use crate::is_blank;
    #[test]
    fn should_is_blank() {
        let result = is_blank(String::from("   "));
        assert_eq!(result, true);
        let result2 = is_blank(String::from(""));
        assert_eq!(result2, true);
    }
}