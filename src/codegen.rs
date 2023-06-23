use crate::ast::{BinaryOpType, Node};

fn push() {
    info!("push %rax");
    println!("    push %rax");
}

use log::info;

fn pop(register: &str) {
    info!("pop {}", register);
    println!("    pop {}", register);
}

pub fn gen_expr(node: &Node) {
    match node {
        Node::NUM(n) => {
            info!("gen expr: {:?}", node);
            println!("    mov ${}, %rax", n);
        }
        Node::BINOP { op, lhs, rhs } => {
            info!("gen expr: {:?}", node);
            gen_expr(rhs);
            push();
            gen_expr(lhs);
            pop("%rdi");
            match op {
                BinaryOpType::Add => {
                    println!("    add %rdi, %rax");
                }
                BinaryOpType::Sub => {
                    println!("    sub %rdi, %rax");
                }
                BinaryOpType::Mul => {
                    println!("    imul %rdi, %rax");
                }
                BinaryOpType::Div => {
                    println!("    cqo");
                    println!("    idiv %rdi");
                }
            }
        }
    }
}
