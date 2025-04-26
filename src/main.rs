use std::fs;

use interpreter::Interpreter;

mod expression;
mod interpreter;
mod machine;

fn main() {
    let target = std::env::args().nth(1).unwrap_or_else(|| {
        let default = "bf_tests/test.bf";
        println!("No argument provided, using default file: {}", default);
        default.to_string()
    });

    let src = fs::read_to_string(target).expect("Something went wrong reading the file");
    print!("Code:\n{}", src);

    let mut interpreter = Interpreter::new(&src);
    interpreter.run();
    print!("\n")
}
