use std::path::{Path, PathBuf};

use clap::{App, ArgMatches, SubCommand};

use bookee::book::*;
use bookee::errors::*;

use crate::cmd::get_book_dir;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Builds books from their markdown files")
        // the {n} denotes a newline which will properly aligned in all help messages
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);

    trace!("{:?}", book_dir);
    let summary = load_summary(book_dir.as_path()).unwrap();
    debug!("{:#?}", summary);

    trace!("--------------------\n");

    for book in summary.iter().filter(|e| e.is_book) {
        trace!("{:#?}\n~~~~~~~~~~~~~~\n", book);
        let book = load_book(book.path.as_path()).unwrap();
        debug!("{:#?}\n~~~~~~~~~~~~~~\n", book);
    }
    Ok(())
}