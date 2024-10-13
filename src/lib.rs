pub mod utils {
    pub mod is_empty;
    pub mod is_blank;
    pub mod reverse;
    pub mod start_with;
    pub mod join_char;
}

pub use utils::is_empty::is_empty;
pub use utils::is_blank::is_blank;
pub use utils::reverse::reverse;
pub use utils::start_with::start_with;
pub use utils::join_char::join_char;

#[cfg(test)]
mod tests {
    use super::*;

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
}