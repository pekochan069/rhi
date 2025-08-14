#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
#![feature(gen_blocks)]

mod error;
mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let str = String::from("0b101 0o17 0xabcdef 0101 0101b");
    let mut lexer = Lexer::new(&str);

    for token in lexer.process() {
        match token {
            Ok(t) => println!("{}", t),
            Err(e) => e.report(),
        }
    }
}
