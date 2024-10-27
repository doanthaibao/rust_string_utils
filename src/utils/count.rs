/// Counts the number of characters in a string that satisfy a given predicate.
///
/// # Arguments
///
/// * `str` - A reference to the string to be processed.
/// * `f` - A closure that takes a character and returns a boolean indicating
///         whether the character should be counted.
///
/// # Returns
///
/// The number of characters in the string that satisfy the predicate.
///
/// # Examples
///
/// ```
/// use rust_string_utils::count_string_bytes;
/// let str = String::from("hello");
/// let count = count_string_bytes(&str, |c| c.is_alphabetic());
/// assert_eq!(count, 5);
/// ```
pub fn count_string_bytes<F>(str: &String, mut f: F) -> usize
where
    F: FnMut(char) -> bool,
{
    str.chars().filter(|c| f(*c)).count()
}

/// Extracts a substring from the beginning of the given string up to the first occurrence of the specified delimiter.
///
/// # Arguments
///
/// * `str` - A reference to the string to be processed.
/// * `end` - The delimiter string to search for.
///
/// # Returns
///
/// An `Option<String>` containing the substring from the start of the string up to the delimiter, or `None` if the delimiter is not found.
///
/// # Examples
///
/// ```
/// use rust_string_utils::slice_string_between;
/// let str = String::from("hello world");
/// let result = slice_string_between(&str, " ");
/// assert_eq!(result, Some("hello".to_string()));
/// ```
pub  fn slice_string_between(str: &String, end: &str) -> Option<String> {
    let end_index = str.find(end)?;
    Some(str[..end_index].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_string_bytes() {
        let str = String::from("hello");
        let count = count_string_bytes(&str, |c| c.is_alphabetic());
        assert_eq!(count, 5);
    }

    #[test]
    fn test_slice_string_between() {
        let str = String::from("hello world");
        let result = slice_string_between(&str, " ");
        assert_eq!(result, Some("hello".to_string()));
    }
}