// Commandline Toolset for Mango
// https://docs.rs/structopt/0.3.14/structopt/

use crate::vm::natives::link_natives;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Debug flag
    #[structopt(short, long)]
    pub debug: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

pub fn parse_cli() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.input)?;
    compile_input(content.as_str(), args)?;
    Ok(())
}

fn compile_input(buf: &str, args: Cli) -> Result<(), String> {
    use crate::bytecode::module::Module;
    use crate::compiler::compiler::Compiler;
    use crate::parser::parser::Parser;
    use crate::vm::vm::VM;

    let mut parser = Parser::new(buf);
    let statements = match parser.parse() {
        Ok(stmts) => stmts,
        Err(e) => panic!(format!("{:?}", e)),
    };

    if args.debug {
        println!("AST: {:?}", statements);
    }

    let mut compiler = Compiler::new(parser.strings, args.debug);
    let mut module: Module = match compiler.compile(&statements) {
        Ok(module) => module.clone(),
        Err(e) => panic!(format!("{:?}", e)),
    };

    let mut vm = VM::new(&mut module, args.debug);
    link_natives(&mut vm);
    match vm.interpret() {
        Ok(_) => (),
        Err(e) => panic!(format!("{:?}", e)),
    }
    Ok(())
}
