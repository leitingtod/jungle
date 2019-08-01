#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, AppSettings};
use bookee::utils;
mod cmd;

fn main() {
    pretty_env_logger::init();


    let app = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .after_help("For more information about a specific command, try `mdbook <command> --help`")
        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::build::make_subcommand());

    let matches = match app.get_matches().subcommand() {
        ("init", Some(sub_matches)) => cmd::init::execute(sub_matches),
        ("build", Some(sub_matches)) => cmd::build::execute(sub_matches),
        (_, _) => unreachable!(),
    };

    if let Err(e) = matches {
        utils::log_backtrace(&e);

        std::process::exit(101);
    }
}
