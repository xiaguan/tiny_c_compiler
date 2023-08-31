use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

// test the easy compiler
#[cfg(test)]
mod tests {

    use std::process::Command;

    use crate::setup;

    fn expr_test_func(expr: &str, expected_value: i32) {
        setup();
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
