use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

use serde_json::to_string;
use walkdir::DirEntry;

use crate::errors::*;
use crate::render::{make_book_data, render_book, render_summary, RenderContext};
use crate::utils::remove_dir_content;

pub use self::book::*;
pub use self::summary::*;

mod book;
mod summary;

pub fn build<P: AsRef<Path>>(root_dir: P) -> Result<()> {
    // clear build-dir files
    let build_dir = get_build_dir(&root_dir.as_ref().to_path_buf());
    if build_dir.exists() {
        remove_dir_content(build_dir.as_path())?;
    }

    let mut summary = load_summary(root_dir.as_ref()).unwrap();
    debug!("{:#?}", summary);

    let mut data = String::new();
    summary.for_each_mut(|e| {
        if e.is_book {
            let path = e.path.join("README.html").display()
                .to_string().replace("src", "build");
            data.push_str(
                format!("{:width$}- [{name}]({path})\n", "",
                        width = e.level * 2, name = e.name, path = path).as_str());
        } else {
            data.push_str(
                format!("{:width$}- {name}\n", "",
                        width = e.level * 2, name = e.name).as_str());
        }
    });

    render_summary(data.as_str(),
                   &get_build_dir(&root_dir.as_ref().to_path_buf()))?;

    trace!("--------------------\n");

    for entry in summary.iter().filter(|e| e.is_book) {
        trace!("{:#?}\n~~~~~~~~~~~~~~\n", entry);
        let book = load_book(entry.path.as_path()).unwrap();
        debug!("{:#?}\n~~~~~~~~~~~~~~\n", book);

        render_book(&RenderContext::new(
            root_dir.as_ref().to_path_buf(),
            book.clone(),
            build_dir.clone(),
        ))?;
    }

    Ok(())
}

pub fn init<P: AsRef<Path>>(root_dir: P) -> Result<()> {
    let src_dir = get_books_dir(&root_dir.as_ref().to_path_buf());
    if !src_dir.exists() {
        create_dir_all(src_dir.as_path())?;
    }
    Ok(())
}

pub fn get_books_dir(root: &PathBuf) -> PathBuf {
    root.join("src")
}

pub fn get_build_dir(root: &PathBuf) -> PathBuf {
    root.join("build")
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}