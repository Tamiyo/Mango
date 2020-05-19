use crate::cli::parse_cli;

mod bytecode;
mod cli;
mod compiler;
mod parser;
mod vm;

fn main() {
    match parse_cli() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    };
}
