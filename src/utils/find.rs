/// Extracts the text between the specified start and end delimiters, excluding the delimiters themselves.
///
/// # Arguments
///
/// * `text` - The input string from which to extract the text.
/// * `start_delimiter` - The delimiter that marks the beginning of the text to extract.
/// * `end_delimiter` - The delimiter that marks the end of the text to extract.
///
/// # Returns
///
/// An `Option<String>` containing the extracted text if both delimiters are found in the input string,
/// or `None` if either delimiter is not found.
///
/// # Examples
///
/// ```
/// use rust_string_utils::get_text_without_delimiters;
/// let result = get_text_without_delimiters("hello world!", "hello ", "!");
/// assert_eq!(result, Some("world".to_string()));
/// ```
pub fn get_text_without_delimiters(text: &str, start_delimiter: &str, end_delimiter: &str) -> Option<String> {
    let start_index = text.find(start_delimiter)?;
    let end_index = text.rfind(end_delimiter)?;
    let start_index = start_index + start_delimiter.len();
    Some(text[start_index..end_index].to_string())
}

/// Extracts the text between the specified start and end delimiters, including the delimiters themselves.
///
/// # Arguments
///
/// * `text` - The input string from which to extract the text.
/// * `start_delimiter` - The delimiter that marks the beginning of the text to extract.
/// * `end_delimiter` - The delimiter that marks the end of the text to extract.
///
/// # Returns
///
/// An `Option<String>` containing the extracted text if both delimiters are found in the input string,
/// or `None` if either delimiter is not found.
///
/// # Examples
///
/// ```
/// use rust_string_utils::get_text_including_delimiters;
/// let result = get_text_including_delimiters("hello world!", "hello ", "!");
/// assert_eq!(result, Some("hello world!".to_string()));
/// ```
pub fn get_text_including_delimiters(text: &str, start_delimiter: &str, end_delimiter: &str) -> Option<String> {
    let start_index = text.find(start_delimiter)?;
    let end_index = text.rfind(end_delimiter)? + end_delimiter.len();
    Some(text[start_index..end_index].to_string())
}


/// Finds the range of text between the specified start and end delimiters.
///
/// # Arguments
///
/// * `text` - The input string from which to find the range.
/// * `start_delimiter` - The delimiter that marks the beginning of the range.
/// * `end_delimiter` - The delimiter that marks the end of the range.
///
/// # Returns
///
/// An `Option<(usize, usize)>` containing a tuple with the start and end indices of the range if both delimiters are found in the input string,
/// or `None` if either delimiter is not found.
///
/// # Examples
///
/// ```
///
/// use rust_string_utils::find_range_between_delimiters;
/// let result = find_range_between_delimiters("hello world!", "hello ", "!");
/// assert_eq!(result, Some((6, 10)));
/// ```
pub fn find_range_between_delimiters(text: &str, start_delimiter: &str, end_delimiter: &str) -> Option<(usize, usize)> {
    let start_index = text.find(start_delimiter)?;
    let end_index = text.rfind(end_delimiter)? - 1;
    let start_index = start_index + start_delimiter.len();
    Some((start_index, end_index))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_text_without_delimiters() {
        assert_eq!(
            Some("world".to_string()),
            get_text_without_delimiters("hello world!", "hello ", "!")
        );
        assert_eq!(
            Some("ello".to_string()),
            get_text_without_delimiters("hello world!", "h", " ")
        );
        assert_eq!(
            Some("orl".to_string()),
            get_text_without_delimiters("hello world!", "w", "d")
        );
    }

    #[test]
    fn test_get_text_including_delimiters() {
        assert_eq!(
            Some(" world!".to_string()),
            get_text_including_delimiters("hello world!", " ", "!")
        );
        assert_eq!(
            Some("hello world!".to_string()),
            get_text_including_delimiters("hello world!", "hello ", "!")
        );
        assert_eq!(
            Some("world!".to_string()),
            get_text_including_delimiters("hello world!", "w", "!")
        );
        assert_eq!(
            Some("llo world".to_string()),
            get_text_including_delimiters("hello world!", "l", "d")
        );
    }
    #[test]
    fn test_find_range_between_delimiters() {
        assert_eq!(
            Some((6, 10)),
            find_range_between_delimiters("hello world!", "hello ", "!")
        );
        assert_eq!(
            Some((1, 4)),
            find_range_between_delimiters("hello world!", "h", " ")
        );
        assert_eq!(
            Some((7, 9)),
            find_range_between_delimiters("hello world!", "w", "d")
        );
    }
}