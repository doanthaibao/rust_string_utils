/// Overlays part of a string with another string.
///
/// # Arguments
///
/// * `str` - The original `String` to be modified.
/// * `overlay` - The `String` to overlay onto the original string.
/// * `start` - The starting index (inclusive) for the overlay.
/// * `end` - The ending index (exclusive) for the overlay.
///
/// # Returns
///
/// A new `String` with the specified part of the original string replaced by the overlay string.
///
/// # Examples
///
/// ```
/// use rust_string_utils::overlay;
/// let result = overlay(&"abcdef".to_string(), &"zzzz".to_string(), 2, 4);
/// assert_eq!(result, "abzzzzef");
/// ```
pub fn overlay(str: &String, overlay: &String, start: i32, end: i32) -> String {
    let len = str.len() as i32;
    let start = start.max(0).min(len);
    let end = end.max(0).min(len);
    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };
    format!(
        "{}{}{}",
        &str[..start as usize],
        overlay,
        &str[end as usize..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay() {
        assert_eq!(overlay(&"".to_string(), &"abc".to_string(), 0, 0), "abc");
        assert_eq!(
            overlay(&"abcdef".to_string(), &"".to_string(), 2, 4),
            "abef"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"".to_string(), 4, 2),
            "abef"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"zzzz".to_string(), 2, 4),
            "abzzzzef"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"zzzz".to_string(), 4, 2),
            "abzzzzef"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"zzzz".to_string(), -1, 4),
            "zzzzef"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"zzzz".to_string(), 2, 8),
            "abzzzz"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"zzzz".to_string(), -2, -3),
            "zzzzabcdef"
        );
        assert_eq!(
            overlay(&"abcdef".to_string(), &"zzzz".to_string(), 8, 10),
            "abcdefzzzz"
        );
        assert_eq!(
            overlay(
                &"5122 5621 0163 0623".to_string(),
                &"****".to_string(),
                15,
                19
            ),
            "5122 5621 0163 ****"
        );
    }
}
