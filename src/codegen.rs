pub fn codegen() {
    println!(".text");
    println!(".global main");
    println!("main:");
    println!("pushq %rbp");
    println!("movq %rsp, %rbp");

    println!("movq $1, %rax");

    println!("pop %rbp");
    println!("ret");
}
