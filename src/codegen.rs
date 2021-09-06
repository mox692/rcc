pub fn codegen(code: i8) {
    println!(".text");
    println!(".global main");
    println!("main:");
    println!("pushq %rbp");
    println!("movq %rsp, %rbp");

    println!("movq ${}, %rax", code);

    println!("pop %rbp");
    println!("ret");
}
