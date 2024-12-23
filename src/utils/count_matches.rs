/// Counts the number of occurrences of a specific character in a given text.
///
/// # Arguments
///
/// * `text` - The string in which to count occurrences of the character.
/// * `search_char` - The character to count within the text.
///
/// # Returns
///
/// The number of times the `search_char` appears in the `text`.
///
/// # Examples
///
/// ```
/// use rust_string_utils::count_matches;
/// let count = count_matches(&String::from("hello world"), 'o');
/// assert_eq!(count, 2);
/// ```
pub fn count_matches(text: &String, search_char: char) -> usize {
    text.chars().filter(|&c| c == search_char).count()
}

#[cfg(test)]
mod tests {
    use crate::count_matches;
    #[test]
    fn should_count_matches() {
        let result = count_matches(&String::from("hello"), 'o');
        assert_eq!(1, result);
        let result2 = count_matches(&String::from("hello world world"), 'o');
        assert_eq!(3, result2);
    }
}
