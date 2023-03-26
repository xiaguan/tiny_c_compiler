use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("{} Invalid number of argument", args[0]);
        process::exit(1);
    }

    println!(" .global main");
    print!("main:\n");
    print!("    movl ${}, %eax\n", args[1].parse::<i32>().unwrap());
    print!("    ret\n");
}

// test the easy compiler
#[cfg(test)]
mod tests{

    use std::process::Command;

    fn expr_test_func(expr : &str, expected_value : i64){
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
        // run the "tmp" file
        // the exit code should be the same as the expr
        assert_eq!(Command::new("./tmp").status().unwrap().code().unwrap(), expr.parse::<i32>().unwrap());
    }

    #[test]
    fn test_basic_expr_with_single_number(){
        expr_test_func("0",0);
        expr_test_func("1", 1);
        expr_test_func("1234567", 1234567);
    }
}
