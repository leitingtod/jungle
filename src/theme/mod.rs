use std::path::Path;

use crate::utils::load_file_contents;

pub static INDEX: &[u8] = include_bytes!("index.hbs");
pub static BOOK: &[u8] = include_bytes!("book.hbs");