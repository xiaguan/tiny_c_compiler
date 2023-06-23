use std::env;
use std::process;

use log::{debug, error};

use tiny_c_compiler::parser::TinyCParser;
use tiny_c_compiler::scanner::{TinyCScanner, Token};
use tiny_c_compiler::string_stream::DoubleBufferStringStream;

fn main() {
    // enable the log  print debug info
    env::set_var("RUST_LOG", "debug");
    //env_logger::init();
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        error!("{} Invalid number of argument", args[0]);
        process::exit(1);
    }

    // init the scanner
    let scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(args[1].clone()));

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

// test the easy compiler
#[cfg(test)]
mod tests {

    use std::process::Command;

    fn expr_test_func(expr: &str, expected_value: i32) {
        // let mut build_cmd = Command::new("cargo");
        // build_cmd.arg("build");
        // build_cmd.status().unwrap().success();
        // init the compiler named "susuncc"
        // the command like "susuncc expr"
        let mut cmd = Command::new("./target/debug/susuncc");
        cmd.arg(expr);
        // open the "tmp.s" file
        let tmp_file = std::fs::File::create("tmp.s").unwrap();
        // redirect the child's stdout to the file
        cmd.stdout(tmp_file);
        assert!(cmd.status().unwrap().success());
        // use gcc to compile the "tmp.s" file
        assert!(Command::new("gcc")
            .arg("-o")
            .arg("tmp")
            .arg("tmp.s")
            .status()
            .unwrap()
            .success());
        // execute "./tmp" file use std::process
        // the exit code should be the same as the expr
        println!("expr: {}, expected_value: {}", expr, expected_value);
        assert_eq!(
            Command::new("./tmp").status().unwrap().code().unwrap(),
            expected_value
        );
    }

    #[test]
    fn test_basic_expr_with_single_number() {
        expr_test_func("0", 0);
        expr_test_func("1", 1);
        expr_test_func("123", 123);
    }

    #[test]
    fn test_basic_add() {
        expr_test_func("1+2", 3);
        expr_test_func("1+2+3", 6);
        expr_test_func("1+2+3+4", 10);
    }

    #[test]
    fn test_basic_sub() {
        // notice! the sub operation is not support the negative number
        expr_test_func("4-2", 2);
        expr_test_func("4-2-1", 1);
        expr_test_func("4-2-1-1", 0);
    }

    #[test]
    fn test_basic_mul() {
        expr_test_func("1*2", 2);
        expr_test_func("1*2*3", 6);
        expr_test_func("1*2*3*4", 24);
    }

    #[test]
    fn test_basic_div() {
        expr_test_func("4/2", 2);
        expr_test_func("4/2/2", 1);
        expr_test_func("4/2/2/2", 0);
    }

    #[test]
    fn test_basic_bracket() {
        expr_test_func("(1+2)", 3);
        expr_test_func("(1+2)*3", 9);
        expr_test_func("(1+2)*(3+4)", 21);
    }

    #[test]
    fn test_basic_expr_easy_expr() {
        expr_test_func("5+20-4", 21);
        expr_test_func("12+34-5", 41);
        expr_test_func("5+6*7", 47);
        expr_test_func("5*(9-6)", 15);
        expr_test_func("(3+5)/2", 4);
    }
}
