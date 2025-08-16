use std::io::Read;

mod skip_nth;
mod dupe_count;
mod consecutive_overlapping_pairs;
mod tilemap;
mod point;
mod direction;

pub use skip_nth::*;
pub use dupe_count::*;
pub use consecutive_overlapping_pairs::*;
pub use tilemap::*;
pub use point::*;
pub use direction::*;

pub fn read_stdin_to_string() -> String {
    let mut buf = "".to_string();

    std::io::stdin()
        .read_to_string(&mut buf).unwrap();

    buf
}