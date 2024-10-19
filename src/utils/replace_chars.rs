/// Replaces all occurrences of a specific character in a given text with another character.
///
/// # Arguments
///
/// * `text` - The string in which to perform the replacement.
/// * `search_char` - The character to be replaced.
/// * `replacement` - The character to replace `search_char` with.
///
/// # Returns
///
/// A new `String` with all occurrences of `search_char` replaced by `replacement`.
///
/// # Examples
///
/// ```
///  use rust_string_utils::replace_chars;
/// let result = replace_chars(&String::from("hello world"), 'o', 'a');
/// assert_eq!(result, "hella warld");
/// ```
pub fn replace_chars(text: &String, search_char: char, replacement: char) -> String {
    text.chars()
        .map(|c| if c == search_char { replacement } else { c })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::replace_chars;
    #[test]
    fn should_replace_chars() {
        let result = replace_chars(&String::from("abcdef"), 'a', 'z');
        assert_eq!("zbcdef", result);
        let result2 = replace_chars(&String::from("aba"), 'a', 'z');
        assert_eq!("zbz", result2);
    }
}
