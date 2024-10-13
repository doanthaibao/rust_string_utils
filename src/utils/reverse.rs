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