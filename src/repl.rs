use std::io::{Stdin, Stdout, Write};

use crate::{parser::Parser, scanner::scan_tokens, token::TokenType};

pub fn run_prompt(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">> ").expect("should have written prompt string >>");
        stdout.flush().expect("should have flushed stdout!");

        let mut input = String::new();
        if let Err(e) = stdin.read_line(&mut input) {
            write!(stdout, "Error: {e}").expect("should have written error message");
            return;
        }

        match scan_tokens(&input) {
            Ok(tokens) => {
                // Scannerによる解析結果を追加
                for token in tokens.iter() {
                    if token.ty == TokenType::Eof {
                        writeln!(stdout, "End of line").expect("should set error message");
                        break;
                    }

                    writeln!(stdout, "{token:?}").expect("Token should have been written");
                }

                // Parserによる解析結果の追加
                match Parser::new(tokens).parse() {
                    Ok(expr) => writeln!(stdout, "expression: {expr:?}")
                        .expect("Error message should have been written"),
                    Err(e) => writeln!(stdout, "wrong expression: {e}")
                        .expect("Error message should have been written"),
                };
            }
            Err(err) => writeln!(stdout, "Error while scanning tokens: {err}")
                .expect("Error message should have been written"),
        }
    }
}
