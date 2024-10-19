/// Rotates the characters in a string by a given shift amount.
///
/// # Arguments
///
/// * `s` - A string slice to be rotated.
/// * `shift` - An integer representing the number of positions to shift the characters.
///
/// # Returns
///
/// A `String` with the characters rotated by the specified shift amount.
///
/// # Examples
///
/// ```
/// use rust_string_utils::rotate;
/// let result = rotate("abcdefg", 2);
/// assert_eq!(result, "fgabcde");
/// ```
pub fn rotate(s: &str, shift: i32) -> String {
    let len = s.len() as i32;
    let shift = ((shift % len) + len) % len;
    let split_index = (len - shift) as usize;
    format!("{}{}", &s[split_index..], &s[..split_index])
}

/// Rotates the characters in a string by a given shift amount.
///
/// # Arguments
///
/// * `s` - A string slice to be rotated.
/// * `shift` - An integer representing the number of positions to shift the characters.
///
/// # Returns
///
/// A `String` with the characters rotated by the specified shift amount.
///
/// # Examples
///
/// ```
/// use rust_string_utils::rotate;
/// let result = rotate("abcdefg", 2);
/// assert_eq!(result, "fgabcde");
/// ```
pub fn split(s: &String) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

/// Splits a string into a vector of strings, using the specified separator characters.
///
/// # Arguments
///
/// * `s` - A `String` to be split.
/// * `separator_chars` - A `String` containing the characters to be used as separators.
///
/// # Returns
///
/// A `Vec<String>` containing the substrings of the original string, split by the specified separator characters.
///
/// # Examples
///
/// ```
/// use rust_string_utils::split_with_separator;
/// let result = split_with_separator(&"abc,def".to_string(), ",".to_string());
/// assert_eq!(result, vec!["abc", "def"]);
/// ```
pub fn split_with_separator(s: &String, separator_chars: String) -> Vec<String> {
    s.split(&separator_chars).map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(rotate(&"abcdefg", 0), "abcdefg");
        assert_eq!(rotate(&"abcdefg", 2), "fgabcde");
        assert_eq!(rotate(&"abcdefg", -2), "cdefgab");
        assert_eq!(rotate(&"abcdefg", 7), "abcdefg");
        assert_eq!(rotate(&"abcdefg", -7), "abcdefg");
    }

    #[test]
    fn test_split() {
        assert_eq!(split(&"abc def".to_string()), vec!["abc", "def"]);
        assert_eq!(split(&"abc  def".to_string()), vec!["abc", "def"]);
        assert_eq!(split(&" abc ".to_string()), vec!["abc"]);
    }

    #[test]
    fn test_split_with_separator() {
        assert_eq!(
            split_with_separator(&"abc,def".to_string(), ",".to_string()),
            vec!["abc", "def"]
        );
        assert_eq!(
            split_with_separator(&"abc.def".to_string(), ".".to_string()),
            vec!["abc", "def"]
        );
        assert_eq!(
            split_with_separator(&"ab:cd:ef".to_string(), ":".to_string()),
            vec!["ab", "cd", "ef"]
        );
    }
}
