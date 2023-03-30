use std::env;
use std::process;

use env_logger::Builder;
use log::{debug, error, info, LevelFilter};

use tiny_c_compiler::scanner::{Scanner, TinyCScanner, Token};
use tiny_c_compiler::string_stream::DoubleBufferStringStream;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    // let mut builder = Builder::from_default_env();

    // builder.filter(None, LevelFilter::Info).init();

    if args.len() != 2 {
        error!("{} Invalid number of argument", args[0]);
        process::exit(1);
    }

    // init the scanner
    let mut scanner = TinyCScanner::new(DoubleBufferStringStream::new_with_string(args[1].clone()));

    println!(" .global main");
    println!("main:");

    let first_token = scanner.next_token().unwrap();
    println!("    mov ${},%rax", first_token.get_number());

    // get the token from the scanner
    while let Some(token) = scanner.next_token() {
        if token.is_eof() {
            break;
        }
        match token {
            Token::Keyword(keyword) => {
                if keyword.eq("+") {
                    let next_token = scanner.next_token().unwrap();
                    println!("    add ${},%rax", next_token.get_number());
                } else if keyword.eq("-") {
                    let next_token = scanner.next_token().unwrap();
                    println!("    sub ${},%rax", next_token.get_number());
                }
            }
            _ => {
                panic!("invalid token: {:?}", token);
            }
        }
    }
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
    fn test_basic_expr_easy_expr() {
        expr_test_func("1+2", 3);
        expr_test_func("5+20-4", 21);
        expr_test_func("1+2+3+4+5+6+7+8+9+10", 55);
    }
}
