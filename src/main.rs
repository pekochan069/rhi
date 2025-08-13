#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

mod lexer;
mod token;

fn main() {
    let str = String::from(";(){}[].");
    let res = lexer::lex(str.as_str());

    for token in res {
        println!("token: {:?}", token);
    }
}
