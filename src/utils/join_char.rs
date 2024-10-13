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