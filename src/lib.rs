pub mod error;
pub mod filter;
pub mod replace;
pub mod search;
pub mod stats;

pub use error::TextkitError;
pub use filter::{FilterMode, filter_lines};
pub use replace::replace_text;
pub use search::search_lines;
pub use stats::{FileStats, count_text};
