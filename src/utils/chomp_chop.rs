/// Removes one newline from the end of a string if it's there, otherwise leaves it alone.
/// A newline is considered to be "\n", "\r", or "\r\n".
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be processed.
///
/// # Returns
///
/// A `String` with one newline removed from the end if it exists.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::chomp_chop::chomp;
/// let result = chomp("Hello, world!\n");
/// assert_eq!(result, "Hello, world!");
/// ```
///
/// ```
/// use rust_string_utils::utils::chomp_chop::chomp;
/// let result = chomp("Hello, world!\r\n");
/// assert_eq!(result, "Hello, world!");
/// ```
///
/// ```
/// use rust_string_utils::utils::chomp_chop::chomp;
/// let result = chomp("Hello, world!");
/// assert_eq!(result, "Hello, world!");
/// ```
pub fn chomp(input: &str) -> String {
    if input.ends_with("\r\n") {
        input[..input.len() - 2].to_string()
    } else if input.ends_with('\n') || input.ends_with('\r') {
        input[..input.len() - 1].to_string()
    } else {
        input.to_string()
    }
}


/// Removes the last character from a string.
/// If the string ends in "\r\n", then removes both characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the string to be processed.
///
/// # Returns
///
/// A `String` with the last character removed, or both characters if it ends with "\r\n".
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::chomp_chop::chop;
/// let result = chop("Hello, world!\r\n");
/// assert_eq!(result, "Hello, world!");
/// ```
///
/// ```
/// use rust_string_utils::utils::chomp_chop::chop;
/// let result = chop("Hello, world!");
/// assert_eq!(result, "Hello, world");
/// ```
pub fn chop(input: &str) -> String {
    if input.ends_with("\r\n") {
        input[..input.len() - 2].to_string()
    } else if !input.is_empty() {
        input[..input.len() - 1].to_string()
    } else {
        input.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop() {
        assert_eq!(chop(""), "");
        assert_eq!(chop("abc \r"), "abc ");
        assert_eq!(chop("abc\n"), "abc");
        assert_eq!(chop("abc\r\n"), "abc");
        assert_eq!(chop("abc"), "ab");
        assert_eq!(chop("abc\nabc"), "abc\nab");
        assert_eq!(chop("a"), "");
        assert_eq!(chop("\r"), "");
        assert_eq!(chop("\n"), "");
        assert_eq!(chop("\r\n"), "");
    }

    #[test]
    fn test_chomp() {
        assert_eq!(chomp(""), "");
        assert_eq!(chomp("abc \r"), "abc ");
        assert_eq!(chomp("abc\n"), "abc");
        assert_eq!(chomp("abc\r\n"), "abc");
        assert_eq!(chomp("abc\r\n\r\n"), "abc\r\n");
        assert_eq!(chomp("abc\n\r"), "abc\n");
        assert_eq!(chomp("abc\n\rabc"), "abc\n\rabc");
        assert_eq!(chomp("\r"), "");
        assert_eq!(chomp("\n"), "");
        assert_eq!(chomp("\r\n"), "");
    }
}