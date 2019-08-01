use walkdir::DirEntry;

mod init;
mod book;
mod summary;

pub use self::book::*;
pub use self::summary::*;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}