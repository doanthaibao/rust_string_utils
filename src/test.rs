#[cfg(test)]
mod tests {
    use crate::{is_blank, is_empty, join_char, replace, replace_chars, reverse, start_with, count_matches};

    #[test]
    fn should_is_empty() {
        let result = is_empty(String::from(""));
        assert_eq!(result, true);
    }

    #[test]
    fn should_is_blank() {
        let result = is_blank(String::from("   "));
        assert_eq!(result, true);
        let result2 = is_blank(String::from(""));
        assert_eq!(result2, true);
    }

    #[test]
    fn should_reverse() {
        let result = reverse(String::from("abcde"));
        assert_eq!("edcba", result);
        let result2 = reverse(String::from("a"));
        assert_eq!("a", result2);
    }

    #[test]
    fn should_start_with() {
        let result = start_with(String::from("abcde"), String::from("a"));
        assert_eq!(true, result);
        let result2 = start_with(String::from("abcde"), String::from("b"));
        assert_eq!(false, result2);
    }

    #[test]
    fn should_join_char() {
        let result = join_char(vec!['a', 'b', 'c'], ',');
        assert_eq!("a,b,c", result);
        let result = join_char(vec!['a'], ',');
        assert_eq!("a", result);
    }

    #[test]
    fn should_replace_string() {
        let result = replace(String::from("abcdef"), String::from("a"), String::from(""));
        assert_eq!("bcdef", result);
        let result2 = replace(String::from("aba"), String::from("a"), String::from("z"));
        assert_eq!("zbz", result2)
    }

    #[test]
    fn should_replace_chars() {
        let result = replace_chars(String::from("abcdef"), 'a', 'z');
        assert_eq!("zbcdef", result);
        let result2 = replace_chars(String::from("aba"), 'a', 'z');
        assert_eq!("zbz", result2);
    }

    #[test]
    fn should_count_matches() {
        let result = count_matches(String::from("hello"), 'o');
        assert_eq!(1, result);
        let result2 = count_matches(String::from("hello world world"), 'o');
        assert_eq!(3, result2);
    }
}
