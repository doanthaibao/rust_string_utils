use chrono::{TimeZone, Utc};

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
/// use rust_string_utils::timestamp_to_string;
/// let formatted_date = timestamp_to_string(1618033988000, "%Y-%m-%d %H:%M:%S".to_string());
/// assert_eq!(formatted_date, "2021-04-10 05:53:08");
/// ```
pub fn timestamp_to_string(timestamp: i64, date_format: String) -> String {
    let datetime = Utc.timestamp_millis_opt(timestamp);
    datetime.unwrap().format(date_format.as_str()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_to_string() {
        assert_eq!(timestamp_to_string(1618033988000, "%Y-%m-%d %H:%M:%S".to_string()), "2021-04-10 05:53:08");
        assert_eq!(timestamp_to_string(1618033988000, "%d/%m/%Y %H:%M:%S".to_string()), "10/04/2021 05:53:08");
        assert_eq!(timestamp_to_string(1618033988000, "%B %d, %Y".to_string()), "April 10, 2021");
    }
}