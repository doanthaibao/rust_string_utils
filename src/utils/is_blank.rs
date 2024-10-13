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