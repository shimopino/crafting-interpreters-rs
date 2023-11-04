use std::io;

use crate::repl::run_prompt;

pub mod expr;
pub mod repl;
pub mod scanner;
pub mod token;

fn main() {
    println!("Lox言語のReplです。");
    println!("コードを記述すれば解析したトークンを出力することが可能です。");

    run_prompt(io::stdin(), io::stdout());
}
