use std::env;
use std::process;

use log::{debug, error};

use tiny_c_compiler::parser::TinyCParser;
use tiny_c_compiler::scanner::{TinyCScanner, Token};
use tiny_c_compiler::stream::BasicStream;

fn main() {
    // enable the log  print debug info
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        error!("{} Invalid number of argument", args[0]);
        process::exit(1);
    }

    // init the scanner
    let scanner = TinyCScanner::new(BasicStream::new_with_string(args[1].clone()));

    debug!("start to parse the expr");
    let mut parser = TinyCParser::new(scanner);

    let node = parser.expr();

    debug_assert_eq!(parser.get_current_token(), &Token::Eof);

    debug!("parse the expr success: {:?}", node);

    println!("  .globl main");
    println!("main:");
    // generate the assembly code
    tiny_c_compiler::codegen::gen_expr(&node);
    println!("    ret");
}
