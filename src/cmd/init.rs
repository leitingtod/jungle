use clap::{App, ArgMatches, SubCommand};

use bookee::errors::*;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init")
        .about("Creates the boilerplate structure and files for books")
        // the {n} denotes a newline which will properly aligned in all help messages
        .arg_from_usage(
            "[dir] 'Directory to create books in{n}\
             (Defaults to the Current Directory when omitted)'",
        )
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    Ok(())
}