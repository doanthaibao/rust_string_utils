
/// Checks if the given string is empty.
///
/// # Arguments
///
/// * `str` - A `String` to check for emptiness.
///
/// # Returns
///
/// * `true` if the string is empty, otherwise `false`.
pub fn is_empty(str: String) -> bool {
    str.is_empty()
}

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

/// Joins a vector of characters into a single string, separated by a delimiter.
///
/// # Arguments
///
/// * `arr` - A `Vec<char>` containing the characters to join.
/// * `delimiter` - A `char` used to separate the characters in the resulting string.
///
/// # Returns
///
/// * A `String` with the characters joined by the delimiter.
pub fn join_char(arr: Vec<char>, delimiter: char) -> String {
    arr.iter()
        .map(|&c| c.to_string())
        .collect::<Vec<String>>()
        .join(&delimiter.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_is_empty() {
        let result = is_empty(String::from(""));
        assert_eq!(result, true);
    }

    #[test]
    fn should_is_blank() {
        let result = is_blank(String::from("   "));
        assert_eq!(result, true);
        let result2 = is_blank(String::from(""));
        assert_eq!(result2, true);
    }

    #[test]
    fn should_reverse() {
        let result = reverse(String::from("abcde"));
        assert_eq!("edcba", result);
        let result2 = reverse(String::from("a"));
        assert_eq!("a", result2);
    }

    #[test]
    fn should_start_with() {
        let result = start_with(String::from("abcde"), String::from("a"));
        assert_eq!(true, result);
        let result2 = start_with(String::from("abcde"), String::from("b"));
        assert_eq!(false, result2);
    }

    #[test]
    fn should_join_char() {
        let result = join_char(vec!['a', 'b', 'c'], ',');
        assert_eq!("a,b,c", result);
        let result = join_char(vec!['a'], ',');
        assert_eq!("a", result);
    }
}
