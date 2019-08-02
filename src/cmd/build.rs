use clap::{App, ArgMatches, SubCommand};

use jungle::book::*;
use jungle::errors::*;

use crate::cmd::{get_root_dir, open};

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Builds books from their markdown files")
        // the {n} denotes a newline which will properly aligned in all help messages
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg_from_usage("-o, --open 'Opens the compiled book in a web browser'")
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let root = get_root_dir(args);
    trace!("root dir: {:?}", root);

    if get_books_dir(&root).exists() {
        build(root.as_path())?;

        if args.is_present("open") {
            open(get_build_dir(&root).join("index.html"));
        }
    }

    Ok(())
}