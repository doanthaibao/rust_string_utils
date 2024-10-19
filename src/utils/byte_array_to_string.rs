/// Converts a Unix timestamp in milliseconds to a formatted string.
///
/// # Arguments
///
/// * `timestamp` - A 64-bit integer representing the Unix timestamp in milliseconds.
/// * `date_format` - A string slice that holds the desired date format.
///
/// # Returns
///
/// A `String` containing the formatted date and time.
///
/// # Panics
///
/// This function will panic if the timestamp cannot be converted to a valid `DateTime`.
///
/// # Examples
///
/// ```
/// use rust_string_utils::byte_array_to_string;
///let result = byte_array_to_string(&[97, 98, 99]);
///assert_eq!("abc", result);
/// ```
pub fn byte_array_to_string(byte_array: &[u8]) -> String {
    byte_array.iter().map(|&c| c as char).collect()
}

#[cfg(test)]
mod tests {
    use crate::byte_array_to_string;

    #[test]
    fn should_convert_byte_array_to_string() {
        let result = byte_array_to_string(&[97, 98, 99]);
        assert_eq!("abc", result);
    }
}
