#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod book;
pub mod render;
pub mod utils;

pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}