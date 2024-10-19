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
    use crate::join_char;

    #[test]
    fn should_join_char() {
        let result = join_char(vec!['a', 'b', 'c'], ',');
        assert_eq!("a,b,c", result);
        let result = join_char(vec!['a'], ',');
        assert_eq!("a", result);
    }
}
