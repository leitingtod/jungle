#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

pub mod book;
pub mod render;
pub mod utils;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}