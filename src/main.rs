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
