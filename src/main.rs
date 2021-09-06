mod codegen;

use codegen::codegen;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("argc is {}, not 2", args.len());
        return;
    }
    let code: i8 = args[1].parse().unwrap();
    codegen(code);
    return;
}
