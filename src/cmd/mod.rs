use std::env;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

pub mod init;
pub mod build;

fn get_root_dir(args: &ArgMatches) -> PathBuf {
    if let Some(dir) = args.value_of("dir") {
        // Check if path is relative from current dir, or absolute...
        let p = Path::new(dir);
        if p.is_relative() {
            env::current_dir().unwrap().join(dir)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().expect("Unable to determine the current directory")
    }
}
