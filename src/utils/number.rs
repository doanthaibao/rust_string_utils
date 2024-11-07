/// Checks if the input string contains only alphabetic characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be checked.
///
/// # Returns
///
/// * `true` if the input string contains only alphabetic characters and is not empty.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::number::is_alpha;
/// assert_eq!(is_alpha("abc"), true);
/// assert_eq!(is_alpha("ab2c"), false);
/// assert_eq!(is_alpha(""), false);
/// ```
pub fn is_alpha(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_alphabetic())
}

/// Checks if the input string contains only alphanumeric characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be checked.
///
/// # Returns
///
/// * `true` if the input string contains only alphanumeric characters and is not empty.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::number::is_alphanumeric;
/// assert_eq!(is_alphanumeric("abc123"), true);
/// assert_eq!(is_alphanumeric("abc 123"), false);
/// assert_eq!(is_alphanumeric(""), false);
/// ```
pub fn is_alphanumeric(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_alphanumeric())
}

/// Checks if the input string contains only alphanumeric characters or spaces.
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be checked.
///
/// # Returns
///
/// * `true` if the input string contains only alphanumeric characters or spaces.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::number::is_alphanumeric_space;
/// assert_eq!(is_alphanumeric_space("abc 123"), true);
/// assert_eq!(is_alphanumeric_space("abc-123"), false);
/// assert_eq!(is_alphanumeric_space(""), true);
/// ```
pub fn is_alphanumeric_space(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace())
}

/// Checks if the input string contains only alphabetic characters or spaces.
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be checked.
///
/// # Returns
///
/// * `true` if the input string contains only alphabetic characters or spaces.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::number::is_alpha_space;
/// assert_eq!(is_alpha_space("abc"), true);
/// assert_eq!(is_alpha_space("ab c"), true);
/// assert_eq!(is_alpha_space("ab2c"), false);
/// assert_eq!(is_alpha_space("ab-c"), false);
/// ```
pub fn is_alpha_space(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_alphabetic() || c.is_whitespace())
}

/// Checks if the input string contains only ASCII printable characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be checked.
///
/// # Returns
///
/// * `true` if the input string contains only ASCII printable characters or is empty.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::number::is_ascii_printable;
/// assert_eq!(is_ascii_printable(""), true);
/// assert_eq!(is_ascii_printable(" "), true);
/// assert_eq!(is_ascii_printable("Ceki"), true);
/// assert_eq!(is_ascii_printable("ab2c"), true);
/// assert_eq!(is_ascii_printable("!ab-c~"), true);
/// assert_eq!(is_ascii_printable("\x7F"), false);
/// assert_eq!(is_ascii_printable("Ceki Gülcü"), false);
/// ```
pub fn is_ascii_printable(input: &str) -> bool {
    input.is_empty()
        || input
            .chars()
            .all(|c| c.is_ascii() && (c.is_ascii_graphic() || c.is_ascii_whitespace()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alpha() {
        assert_eq!(is_alpha(""), false);
        assert_eq!(is_alpha("  "), false);
        assert_eq!(is_alpha("abc"), true);
        assert_eq!(is_alpha("ab2c"), false);
        assert_eq!(is_alpha("ab-c"), false);
    }

    #[test]
    fn test_is_alphanumeric() {
        assert_eq!(is_alphanumeric(""), false);
        assert_eq!(is_alphanumeric("  "), false);
        assert_eq!(is_alphanumeric("abc"), true);
        assert_eq!(is_alphanumeric("ab c"), false);
        assert_eq!(is_alphanumeric("ab2c"), true);
        assert_eq!(is_alphanumeric("ab-c"), false);
    }

    #[test]
    fn test_is_alphanumeric_space() {
        assert_eq!(is_alphanumeric_space(""), true);
        assert_eq!(is_alphanumeric_space("  "), true);
        assert_eq!(is_alphanumeric_space("abc"), true);
        assert_eq!(is_alphanumeric_space("ab c"), true);
        assert_eq!(is_alphanumeric_space("ab2c"), true);
        assert_eq!(is_alphanumeric_space("ab-c"), false);
    }

    #[test]
    fn test_is_alpha_space() {
        assert_eq!(is_alpha_space(""), true);
        assert_eq!(is_alpha_space("  "), true);
        assert_eq!(is_alpha_space("abc"), true);
        assert_eq!(is_alpha_space("ab c"), true);
        assert_eq!(is_alpha_space("ab2c"), false);
        assert_eq!(is_alpha_space("ab-c"), false);
    }

    #[test]
    fn test_is_ascii_printable() {
        assert_eq!(is_ascii_printable(""), true);
        assert_eq!(is_ascii_printable(" "), true);
        assert_eq!(is_ascii_printable("Ceki"), true);
        assert_eq!(is_ascii_printable("ab2c"), true);
        assert_eq!(is_ascii_printable("!ab-c~"), true);
        assert_eq!(is_ascii_printable(" "), true);
        assert_eq!(is_ascii_printable("!"), true);
        assert_eq!(is_ascii_printable("~"), true);
        assert_eq!(is_ascii_printable("\x7F"), false);
        assert_eq!(is_ascii_printable("Ceki Gülcü"), false);
    }
}
