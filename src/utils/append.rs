/// Prepends a prefix to a string if it is not already present, considering a list of possible prefixes.
///
/// # Arguments
///
/// * `str` - The original string to which the prefix may be prepended.
/// * `prefix` - The prefix to prepend if it is not already present.
/// * `prefixes` - A list of additional prefixes to check against.
///
/// # Returns
///
/// A new `String` with the prefix prepended if it was not already present.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::append::prepend_if_missing;
/// let result = prepend_if_missing(&"abc".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "xyzabc");
/// ```
///
/// ```
/// use rust_string_utils::utils::append::prepend_if_missing;
/// let result = prepend_if_missing(&"xyzabc".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "xyzabc");
/// ```
pub fn prepend_if_missing(str: &String, prefix: &String, prefixes: &[&String]) -> String {
    if str.starts_with(prefix) || prefixes.iter().any(|&p| str.starts_with(p)) {
        str.to_string()
    } else {
        format!("{}{}", prefix, str)
    }
}

/// Prepends a prefix to a string if it is not already present, considering a list of possible prefixes, ignoring case.
///
/// # Arguments
///
/// * `str` - The original string to which the prefix may be prepended.
/// * `prefix` - The prefix to prepend if it is not already present.
/// * `prefixes` - A list of additional prefixes to check against.
///
/// # Returns
///
/// A new `String` with the prefix prepended if it was not already present.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::append::prepend_if_missing_ignore_case;
/// let result = prepend_if_missing_ignore_case(&"abc".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "xyzabc");
/// ```
///
/// ```
/// use rust_string_utils::utils::append::prepend_if_missing_ignore_case;
/// let result = prepend_if_missing_ignore_case(&"xyzabc".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "xyzabc");
/// ```
pub fn prepend_if_missing_ignore_case(
    str: &String,
    prefix: &String,
    prefixes: &[&String],
) -> String {
    if str.to_lowercase().starts_with(&prefix.to_lowercase())
        || prefixes
            .iter()
            .any(|&p| str.to_lowercase().starts_with(&p.to_lowercase()))
    {
        str.to_string()
    } else {
        format!("{}{}", prefix, str)
    }
}

/// Appends a suffix to a string if it is not already present, considering a list of possible suffixes.
///
/// # Arguments
///
/// * `str` - The original string to which the suffix may be appended.
/// * `suffix` - The suffix to append if it is not already present.
/// * `suffixes` - A list of additional suffixes to check against.
///
/// # Returns
///
/// A new `String` with the suffix appended if it was not already present.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::append::append_if_missing;
/// let result = append_if_missing(&"abc".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "abcxyz");
/// ```
///
/// ```
/// use rust_string_utils::utils::append::append_if_missing;
/// let result = append_if_missing(&"abcxyz".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "abcxyz");
/// ```
pub fn append_if_missing(str: &String, suffix: &String, suffixes: &[&String]) -> String {
    if str.ends_with(suffix) || suffixes.iter().any(|&s| str.ends_with(s)) {
        str.to_string()
    } else {
        format!("{}{}", str, suffix)
    }
}

/// Appends a suffix to a string if it is not already present, considering a list of possible suffixes, ignoring case.
///
/// # Arguments
///
/// * `str` - The original string to which the suffix may be appended.
/// * `suffix` - The suffix to append if it is not already present.
/// * `suffixes` - A list of additional suffixes to check against.
///
/// # Returns
///
/// A new `String` with the suffix appended if it was not already present.
///
/// # Examples
///
/// ```
/// use rust_string_utils::utils::append::append_if_missing_ignore_case;
/// let result = append_if_missing_ignore_case(&"abc".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "abcxyz");
/// ```
///
/// ```
/// use rust_string_utils::utils::append::append_if_missing_ignore_case;
/// let result = append_if_missing_ignore_case(&"abcxyz".to_string(), &"xyz".to_string(), &[]);
/// assert_eq!(result, "abcxyz");
/// ```
pub fn append_if_missing_ignore_case(
    str: &String,
    suffix: &String,
    suffixes: &[&String],
) -> String {
    if str.to_lowercase().ends_with(&suffix.to_lowercase())
        || suffixes
            .iter()
            .any(|&s| str.to_lowercase().ends_with(&s.to_lowercase()))
    {
        str.to_string()
    } else {
        format!("{}{}", str, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend_if_missing() {
        assert_eq!(
            prepend_if_missing(&"abc".to_string(), &"xyz".to_string(), &[]),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing(&"xyzabc".to_string(), &"xyz".to_string(), &[]),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing(&"XYZabc".to_string(), &"xyz".to_string(), &[]),
            "xyzXYZabc"
        );
        assert_eq!(
            prepend_if_missing(
                &"abc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing(
                &"xyzabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing(
                &"mnoabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "mnoabc"
        );
        assert_eq!(
            prepend_if_missing(
                &"XYZabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "xyzXYZabc"
        );
        assert_eq!(
            prepend_if_missing(
                &"MNOabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "xyzMNOabc"
        );
        assert_eq!(
            prepend_if_missing(&"abc".to_string(), &"xyz".to_string(), &[&"".to_string()]),
            "abc"
        );
    }

    #[test]
    fn test_prepend_if_missing_ignore_case() {
        assert_eq!(
            prepend_if_missing_ignore_case(&"abc".to_string(), &"xyz".to_string(), &[]),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(&"xyzabc".to_string(), &"xyz".to_string(), &[]),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(&"XYZabc".to_string(), &"xyz".to_string(), &[]),
            "XYZabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(
                &"abc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(
                &"xyzabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "xyzabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(
                &"mnoabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "mnoabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(
                &"XYZabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "XYZabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(
                &"MNOabc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "MNOabc"
        );
        assert_eq!(
            prepend_if_missing_ignore_case(
                &"abc".to_string(),
                &"xyz".to_string(),
                &[&"".to_string()]
            ),
            "abc"
        );
    }

    #[test]
    fn test_append_if_missing() {
        assert_eq!(append_if_missing(&"".to_string(), &"".to_string(), &[]), "");
        assert_eq!(
            append_if_missing(&"abc".to_string(), &"".to_string(), &[]),
            "abc"
        );
        assert_eq!(
            append_if_missing(&"".to_string(), &"xyz".to_string(), &[]),
            "xyz"
        );
        assert_eq!(
            append_if_missing(&"abc".to_string(), &"xyz".to_string(), &[]),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing(&"abcxyz".to_string(), &"xyz".to_string(), &[]),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing(&"abcXYZ".to_string(), &"xyz".to_string(), &[]),
            "abcXYZxyz"
        );

        assert_eq!(append_if_missing(&"".to_string(), &"".to_string(), &[]), "");
        assert_eq!(
            append_if_missing(&"abc".to_string(), &"".to_string(), &[]),
            "abc"
        );
        assert_eq!(
            append_if_missing(&"".to_string(), &"xyz".to_string(), &[]),
            "xyz"
        );
        assert_eq!(
            append_if_missing(&"abc".to_string(), &"xyz".to_string(), &[&"".to_string()]),
            "abc"
        );
        assert_eq!(
            append_if_missing(
                &"abc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing(
                &"abcxyz".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing(
                &"abcmno".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcmno"
        );
        assert_eq!(
            append_if_missing(
                &"abcXYZ".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcXYZxyz"
        );
        assert_eq!(
            append_if_missing(
                &"abcMNO".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcMNOxyz"
        );
    }

    #[test]
    fn test_append_if_missing_ignore_case() {
        assert_eq!(
            append_if_missing_ignore_case(&"".to_string(), &"".to_string(), &[]),
            ""
        );
        assert_eq!(
            append_if_missing_ignore_case(&"abc".to_string(), &"".to_string(), &[]),
            "abc"
        );
        assert_eq!(
            append_if_missing_ignore_case(&"".to_string(), &"xyz".to_string(), &[]),
            "xyz"
        );
        assert_eq!(
            append_if_missing_ignore_case(&"abc".to_string(), &"xyz".to_string(), &[]),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing_ignore_case(&"abcxyz".to_string(), &"xyz".to_string(), &[]),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing_ignore_case(&"abcXYZ".to_string(), &"xyz".to_string(), &[]),
            "abcXYZ"
        );

        assert_eq!(
            append_if_missing_ignore_case(&"".to_string(), &"".to_string(), &[]),
            ""
        );
        assert_eq!(
            append_if_missing_ignore_case(&"abc".to_string(), &"".to_string(), &[]),
            "abc"
        );
        assert_eq!(
            append_if_missing_ignore_case(&"".to_string(), &"xyz".to_string(), &[]),
            "xyz"
        );
        assert_eq!(
            append_if_missing_ignore_case(
                &"abc".to_string(),
                &"xyz".to_string(),
                &[&"".to_string()]
            ),
            "abc"
        );
        assert_eq!(
            append_if_missing_ignore_case(
                &"abc".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing_ignore_case(
                &"abcxyz".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcxyz"
        );
        assert_eq!(
            append_if_missing_ignore_case(
                &"abcmno".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcmno"
        );
        assert_eq!(
            append_if_missing_ignore_case(
                &"abcXYZ".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcXYZ"
        );
        assert_eq!(
            append_if_missing_ignore_case(
                &"abcMNO".to_string(),
                &"xyz".to_string(),
                &[&"mno".to_string()]
            ),
            "abcMNO"
        );
    }
}
