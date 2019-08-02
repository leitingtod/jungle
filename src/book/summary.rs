use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::book::{get_books_dir, is_hidden};
use crate::errors::*;

pub fn load_summary<P: AsRef<Path>>(root_dir: P) -> Result<Summary> {
    debug!("root dir: {:?}", root_dir.as_ref());
    let src_dir = get_books_dir(&root_dir.as_ref().to_path_buf());
    let books = make_summary(src_dir.as_path(), 0)?;

    let title = Some(String::from(src_dir.to_path_buf().to_str().unwrap()));

    Ok(Summary {
        title,
        root: root_dir.as_ref().to_path_buf(),
        items: books,
    })
}

fn make_summary<P: AsRef<Path>>(src_dir: P, level: usize) -> Result<Vec<Link>> {
    let walker = WalkDir::new(src_dir.as_ref()).max_depth(1).into_iter();

    let mut books = Vec::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let filename = entry.path().file_stem().unwrap().to_str().unwrap();
        let depth = entry.depth();

        if entry.path().is_dir() && !filename.eq("assets") && depth != 0 {
            trace!("{} - {}", depth, entry.path().display());

            let path = entry.path().to_str().unwrap();

            books.push(Link {
                name: filename.into(),
                level,
                is_book: entry.path().join("README.md").exists(),
                path: PathBuf::from(path),
                nested_items: make_summary(PathBuf::from(path), level + 1).unwrap(),
            });
        }
    }

    Ok(books)
}


#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    /// An optional title for the `SUMMARY.md`, currently just ignored.
    pub title: Option<String>,
    /// Books' root directory.
    pub root: PathBuf,
    /// Books
    pub items: Vec<Link>,
}

impl Summary {
    /// Create an empty book.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get a depth-first iterator over the items in the book.
    pub fn iter(&self) -> Books<'_> {
        Books {
            items: self.items.iter().collect(),
        }
    }

    pub fn for_each_mut<F>(&mut self, mut func: F)
        where
            F: FnMut(&mut Link),
    {
        for_each_mut(&mut func, &mut self.items);
    }
}

pub fn for_each_mut<'a, F, I>(func: &mut F, items: I)
    where
        F: FnMut(&mut Link),
        I: IntoIterator<Item=&'a mut Link>,
{
    for item in items {
        func(item);
        for_each_mut(func, &mut item.nested_items);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// The name of the book.
    pub name: String,
    pub is_book: bool,
    /// The location of the book's source file, retaking the book's `src`
    /// directory as the root.
    pub path: PathBuf,
    pub level: usize,
    pub nested_items: Vec<Link>,
}

impl Link {
    /// Create a new link with no nested items.
    pub fn new<S: Into<String>, P: AsRef<Path>>(name: S, path: P) -> Link {
        Link {
            name: name.into(),
            is_book: false,
            level: 0,
            path: path.as_ref().to_path_buf(),
            nested_items: Vec::new(),
        }
    }
}

impl Default for Link {
    fn default() -> Self {
        Link {
            name: String::new(),
            is_book: false,
            level: 0,
            path: PathBuf::new(),
            nested_items: Vec::new(),
        }
    }
}


/// A depth-first iterator over the items in a book.
pub struct Books<'a> {
    items: VecDeque<&'a Link>,
}

impl<'a> Iterator for Books<'a> {
    type Item = &'a Link;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.pop_front();

        if let Some(link) = item {
            // if we wanted a breadth-first iterator we'd `extend()` here
            for sub_item in link.nested_items.iter().rev() {
                self.items.push_front(sub_item);
            }
        }

        item
    }
}


