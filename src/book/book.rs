use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::book::is_hidden;
use crate::errors::*;

pub fn load_book<P: AsRef<Path>>(src_dir: P) -> Result<Book> {
    trace!("{:?}", src_dir.as_ref());

    let mut chapters = Vec::new();

    let walker = WalkDir::new(src_dir.as_ref()).max_depth(1).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let filename = entry.path().file_stem().unwrap().to_str().unwrap();
        if entry.path().is_file() && !filename.to_lowercase().eq("readme") {
            trace!("{}", entry.path().display());
            chapters.push(Chapter {
                name: filename.into(),
                path: entry.into_path(),
            });
        }
    }

    let name = src_dir.as_ref().to_path_buf().to_str().unwrap().into();

    Ok(Book {
        name,
        chapters,
        ..Default::default()
    })
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub chapters: Vec<Chapter>,
    __non_exhaustive: (),
}

impl Book {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    /// The chapter's name.
    pub name: String,
    /// The chapter's location, relative to the `README.md` file.
    pub path: PathBuf,
}

impl Chapter {
    pub fn new<P: Into<PathBuf>>(
        name: &str,
        path: P,
    ) -> Chapter {
        Chapter {
            name: name.to_string(),
            path: path.into(),
        }
    }
}



