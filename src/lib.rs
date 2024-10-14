
pub mod utils {
    pub mod is_empty;
    pub mod is_blank;
    pub mod reverse;
    pub mod start_with;
    pub mod join_char;
    pub mod replace;
    pub mod replace_chars;
    pub mod count_matches;
    pub mod trip;
}

pub use utils::is_empty::is_empty;
pub use utils::is_blank::is_blank;
pub use utils::reverse::reverse;
pub use utils::start_with::start_with;
pub use utils::join_char::join_char;
pub use utils::replace::replace;
pub use utils::replace_chars::replace_chars;
pub use utils::count_matches::count_matches;
pub use utils::trip::trip;

