use clap::{App, ArgMatches, SubCommand};

use bookee::book::*;
use bookee::errors::*;

use crate::cmd::get_root_dir;

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
    let root = get_root_dir(args);
    trace!("root dir: {:?}", root);

    if get_books_dir(&root).exists() {
        build(root.as_path())?;
    }

    Ok(())
}