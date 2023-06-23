# Tiny C Compiler

This project is heavily influenced by the [chibicc](https://github.com/rui314/chibicc) project, specifically its 5th commit.

## What does it do?

The main purpose of this project is to parse expressions and generate assembly code. For example, you can use the following command:

```
cargo run 1+2
```

This will print the generated assembly code to the standard output:

```
.globl main
main:
    mov $2, %rax
    push %rax
    mov $1, %rax
    pop %rdi
    add %rdi, %rax
    ret
```

You can then use `gcc` to compile it into an executable file, allowing you to run it:

```
gcc -static -o tmp tmp.s
```

The result of the expression will be the return value of the program, which is the value stored in the `rax` register.

For more supported expressions, you can refer to the tests in `main.rs`.