use crate::token::{Token, TokenType};

/// `Scanner`は、入力された文字列をトークンの配列に解析するための構造体
pub struct Scanner {
    /// 入力文字列を保持する
    /// マルチバイトのUTF-8文字も安全に取り扱えるように char 型として保持する
    pub source: Vec<char>,
    /// 入力文字列を該当するトークンに変換した配列を保持する
    pub tokens: Vec<Token>,
    /// スキャン中のトークンの最初の文字の位置を指す
    pub start: usize,
    /// スキャン中に注目している文字を指す
    pub current: usize,
    /// `current`が入力文字列の何行目に当たるのかを追跡管理する
    pub line: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: vec![],
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, source: &str) {
        self.source = source.chars().collect();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::Eof, "".to_string());
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LParan, "(".to_string()),
            ')' => self.add_token(TokenType::RParan, ")".to_string()),
            '{' => self.add_token(TokenType::LBrace, "{".to_string()),
            '}' => self.add_token(TokenType::RBrace, "}".to_string()),
            ',' => self.add_token(TokenType::Comma, ",".to_string()),
            '.' => self.add_token(TokenType::Dot, ".".to_string()),
            '-' => self.add_token(TokenType::Minus, "-".to_string()),
            '+' => self.add_token(TokenType::Plus, "+".to_string()),
            ';' => self.add_token(TokenType::SemiColon, ";".to_string()),
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, "/".to_string());
                }
            }
            '*' => self.add_token(TokenType::Star, "*".to_string()),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual, "!=".to_string())
                } else {
                    self.add_token(TokenType::Bang, "!".to_string())
                };
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual, "==".to_string())
                } else {
                    self.add_token(TokenType::Equal, "=".to_string())
                };
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual, "<=".to_string())
                } else {
                    self.add_token(TokenType::Less, "<".to_string())
                };
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual, ">=".to_string())
                } else {
                    self.add_token(TokenType::Greater, ">".to_string())
                };
            }
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => unimplemented!(),
        };
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, ty: TokenType, literal: String) {
        self.tokens.push(Token {
            ty,
            literal,
            lexeme: String::new(),
            line: self.line,
        })
    }

    fn matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::String, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::Scanner;

    #[test]
    fn test_one_char_token() {
        let input = "(){},.-+;/*";

        let expected = vec![
            Token {
                ty: TokenType::LParan,
                literal: "(".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::RParan,
                literal: ")".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::LBrace,
                literal: "{".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::RBrace,
                literal: "}".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Comma,
                literal: ",".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Dot,
                literal: ".".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Minus,
                literal: "-".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Plus,
                literal: "+".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::SemiColon,
                literal: ";".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Slash,
                literal: "/".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Star,
                literal: "*".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Eof,
                literal: "".to_string(),
                lexeme: String::new(),
                line: 1,
            },
        ];

        let mut scanner = Scanner::new();
        scanner.scan_tokens(input);

        for (idx, token) in scanner.tokens.into_iter().enumerate() {
            let exp_token = &expected[idx];
            assert_eq!(
                token.ty, exp_token.ty,
                "tokens[{idx}] ty - got={}, expected={}",
                token.ty, exp_token.ty,
            );
            assert_eq!(
                token.literal, exp_token.literal,
                "tokens[{idx}] literal - got={}, expected={}",
                token.literal, exp_token.literal
            );
        }
    }

    #[test]
    fn test_conditional_char_token() {
        let input = "!!=<<=>>====";

        let expected = vec![
            Token {
                ty: TokenType::Bang,
                literal: "!".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::BangEqual,
                literal: "!=".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Less,
                literal: "<".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::LessEqual,
                literal: "<=".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Greater,
                literal: ">".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::GreaterEqual,
                literal: ">=".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::EqualEqual,
                literal: "==".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Equal,
                literal: "=".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Eof,
                literal: "".to_string(),
                lexeme: String::new(),
                line: 1,
            },
        ];

        let mut scanner = Scanner::new();
        scanner.scan_tokens(input);

        for (idx, token) in scanner.tokens.into_iter().enumerate() {
            let exp_token = &expected[idx];
            assert_eq!(
                token.ty, exp_token.ty,
                "tokens[{idx}] ty - got={}, expected={}",
                token.ty, exp_token.ty,
            );
            assert_eq!(
                token.literal, exp_token.literal,
                "tokens[{idx}] literal - got={}, expected={}",
                token.literal, exp_token.literal
            );
        }
    }

    #[test]
    fn test_comment_out() {
        let input = r#"
        ( 
            // コメントアウト
        ) 
        "#;

        let expected = vec![
            Token {
                ty: TokenType::LParan,
                literal: "(".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::RParan,
                literal: ")".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::Eof,
                literal: "".to_string(),
                lexeme: String::new(),
                line: 1,
            },
        ];

        let mut scanner = Scanner::new();
        scanner.scan_tokens(input);

        for (idx, token) in scanner.tokens.into_iter().enumerate() {
            let exp_token = &expected[idx];
            assert_eq!(
                token.ty, exp_token.ty,
                "tokens[{idx}] ty - got={}, expected={}",
                token.ty, exp_token.ty,
            );
            assert_eq!(
                token.literal, exp_token.literal,
                "tokens[{idx}] literal - got={}, expected={}",
                token.literal, exp_token.literal
            );
        }
    }

    #[test]
    fn test_string_literal() {
        let input = r#"
        "string_value"

        "string
value"
        "#;

        let expected = vec![
            Token {
                ty: TokenType::String,
                literal: "string_value".to_string(),
                lexeme: String::new(),
                line: 1,
            },
            Token {
                ty: TokenType::String,
                literal: "string\nvalue".to_string(),
                lexeme: String::new(),
                line: 2,
            },
            Token {
                ty: TokenType::Eof,
                literal: "".to_string(),
                lexeme: String::new(),
                line: 1,
            },
        ];

        let mut scanner = Scanner::new();
        scanner.scan_tokens(input);

        for (idx, token) in scanner.tokens.into_iter().enumerate() {
            let exp_token = &expected[idx];
            assert_eq!(
                token.ty, exp_token.ty,
                "tokens[{idx}] ty - got={}, expected={}",
                token.ty, exp_token.ty,
            );
            assert_eq!(
                token.literal, exp_token.literal,
                "tokens[{idx}] literal - got={}, expected={}",
                token.literal, exp_token.literal
            );
        }
    }
}
