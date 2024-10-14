/// Swaps the case of each character in the given string.
/// Uppercase characters are converted to lowercase, and lowercase characters are converted to uppercase.
///
/// # Arguments
///
/// * `s` - A string slice that holds the string to be processed.
///
/// # Returns
///
/// A new `String` with the case of each character swapped.
///
/// # Examples
///
/// ```
///  use rust_string_utils::swap_case;
///
/// let result = swap_case("Hello World!");
/// assert_eq!(result, "hELLO wORLD!");
/// let result2 = swap_case("12345");
/// assert_eq!(result2, "12345");
/// let result3 = swap_case("1a2b3c4d5e");
/// assert_eq!(result3, "1A2B3C4D5E");
/// ```
pub fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_case() {
        assert_eq!(swap_case("Hello World!"), "hELLO wORLD!");
        assert_eq!(swap_case("12345"), "12345");
        assert_eq!(swap_case("1a2b3c4d5e"), "1A2B3C4D5E");
    }
}