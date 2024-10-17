/// Finds the index of the first occurrence of the specified substring.
///
/// # Arguments
///
/// * `str` - The `String` to search within.
/// * `search` - The `String` to search for.
///
/// # Returns
///
/// The index of the first occurrence of the specified substring, or `-1` if the substring is not found.
pub fn index_of(str: String, search: String) -> i32 {
    match str.find(&search) {
        Some(index) => index as i32,
        None => -1,
    }
}

/// Finds the index of the first occurrence of the specified substring.
///
/// # Arguments
///
/// * `str` - The `String` to search within.
/// * `search` - The `String` to search for.
///
/// # Returns
///
/// The index of the first occurrence of the specified substring, or `-1` if the substring is not found.
pub fn index_of_from(str: String, search: String, from_index: i32) -> i32 {
    let from_index = from_index.max(0) as usize;
    match str[from_index..].find(&search) {
        Some(index) => (index + from_index) as i32,
        None => -1,
    }
}

/// Finds the index of the last occurrence of the specified substring.
///
/// # Arguments
///
/// * `str` - The `String` to search within.
/// * `search` - The `String` to search for.
///
/// # Returns
///
/// The index of the last occurrence of the specified substring, or `-1` if the substring is not found.
pub fn last_index_of(str: String, search: String) -> i32 {
    match str.rfind(&search) {
        Some(index) => index as i32,
        None => -1,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        assert_eq!(index_of("hello".to_string(), "l".to_string()), 2);
        assert_eq!(index_of("hello".to_string(), "o".to_string()), 4);
        assert_eq!(index_of("hello".to_string(), "a".to_string()), -1);
        assert_eq!(index_of_from("hello".to_string(), "l".to_string(), 3), 3);
        assert_eq!(index_of_from("hello".to_string(), "o".to_string(), 3), 4);
        assert_eq!(index_of_from("hello".to_string(), "a".to_string(), 3), -1);
    }

    #[test]
    fn test_last_index_of() {
        assert_eq!(last_index_of("hello".to_string(), "l".to_string()), 3);
        assert_eq!(last_index_of("hello".to_string(), "o".to_string()), 4);
        assert_eq!(last_index_of("hello".to_string(), "a".to_string()), -1);
    }
}
