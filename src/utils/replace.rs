/// Replaces all occurrences of a search string within a given text with a replacement string.
///
/// # Arguments
///
/// * `text` - The original string where the search and replace will be performed.
/// * `search_string` - The substring that will be searched for within the text.
/// * `replacement` - The string that will replace each occurrence of the search string.
///
/// # Returns
///
/// A new `String` with all occurrences of the search string replaced by the replacement string.
///
/// # Examples
///
/// ```
/// use rust_string_utils::replace;
/// let result = replace(&String::from("hello world"), &String::from("world"), &String::from("Rust"));
/// assert_eq!(result, "hello Rust");
/// ```
pub fn replace(text: &String, search_string: &String, replacement: &String) -> String {
    text.replace(search_string.as_str(), replacement.as_str())
}

#[cfg(test)]
mod tests {
    use crate::replace;
    #[test]
    fn should_replace_string() {
        let result = replace(
            &String::from("abcdef"),
            &String::from("a"),
            &String::from(""),
        );
        assert_eq!("bcdef", result);
        let result2 = replace(&String::from("aba"), &String::from("a"), &String::from("z"));
        assert_eq!("zbz", result2)
    }
}
