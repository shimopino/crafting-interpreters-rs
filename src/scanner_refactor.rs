use crate::token_refactor::{Token, TokenType};

/// `Scanner`は、入力された文字列をトークンの配列に解析するための構造体
struct Scanner {
    /// 入力文字列を保持する
    /// マルチバイトのUTF-8文字も安全に取り扱えるように char 型として保持する
    pub source: Vec<char>,
    /// 字句解析した結果のトークンを保持する
    pub tokens: Vec<Token>,
    /// スキャン中のトークンの最初の文字の位置を指す
    pub start: usize,
    /// スキャン中に注目している文字を指す
    pub current: usize,
    /// `current`が入力文字列の何行目に当たるのかを追跡管理する
    pub line: usize,
}

pub fn scan_tokens(input: &str) -> Result<Vec<Token>, String> {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens()?;
    Ok(scanner.tokens)
}

impl Scanner {
    fn new(input: &str) -> Self {
        Scanner {
            source: input.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            ty: TokenType::Eof,
            lexeme: vec![],
            literal: None,
            line: self.line,
        });

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '{' => self.add_token(TokenType::LBrace),
            '}' => self.add_token(TokenType::RBrace),
            '(' => self.add_token(TokenType::LParan),
            ')' => self.add_token(TokenType::RParan),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => {
                self.line += 1;
            }
            _ => return Err(String::from(format!("invalid token: {c}"))),
        };

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    /// ソースコードの終わりに達しているかどうかを判定します。
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, ty: TokenType) -> () {
        self.tokens.push(Token {
            ty,
            lexeme: self.source[self.start..self.current].to_vec(),
            literal: None,
            line: self.line,
        })
    }

    /// 次の文字が期待したものであった場合に `true`` を返却し、文字を消費する
    /// 期待したものではなかった場合は、文字を消費しない
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
            '\0'
        } else {
            self.source[self.current]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{scanner_refactor::scan_tokens, token_refactor::Token, token_refactor::TokenType};

    #[test]
    fn test_one_char_token() {
        let input = "{}(),.-+;/*";

        let expected = vec![
            Token {
                ty: TokenType::LBrace,
                lexeme: vec!['{'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::RBrace,
                lexeme: vec!['}'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::LParan,
                lexeme: vec!['('],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::RParan,
                lexeme: vec![')'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Comma,
                lexeme: vec![','],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Dot,
                lexeme: vec!['.'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Minus,
                lexeme: vec!['-'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Plus,
                lexeme: vec!['+'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::SemiColon,
                lexeme: vec![';'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Slash,
                lexeme: vec!['/'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Star,
                lexeme: vec!['*'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Eof,
                lexeme: vec![],
                literal: None,
                line: 1,
            },
        ];

        let tokens = scan_tokens(input).expect("スキャンに失敗しました。");
        assert_eq!(
            expected.len(),
            tokens.len(),
            "トークンの数が期待と異なります。"
        );

        for (expected_token, actual_token) in expected.into_iter().zip(tokens.into_iter()) {
            assert_eq!(
                expected_token, actual_token,
                "期待するトークンと実際のトークンが異なります。"
            );
        }
    }

    #[test]
    fn test_conditional_char_token() {
        let input = "!!====>>=<<=";

        let expected = vec![
            Token {
                ty: TokenType::Bang,
                lexeme: vec!['!'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::BangEqual,
                lexeme: vec!['!', '='],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::EqualEqual,
                lexeme: vec!['=', '='],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Equal,
                lexeme: vec!['='],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Greater,
                lexeme: vec!['>'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::GreaterEqual,
                lexeme: vec!['>', '='],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Less,
                lexeme: vec!['<'],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::LessEqual,
                lexeme: vec!['<', '='],
                literal: None,
                line: 1,
            },
            Token {
                ty: TokenType::Eof,
                lexeme: vec![],
                literal: None,
                line: 1,
            },
        ];

        let tokens = scan_tokens(input).expect("スキャンに失敗しました。");
        assert_eq!(
            expected.len(),
            tokens.len(),
            "トークンの数が期待と異なります。"
        );

        for (expected_token, actual_token) in expected.into_iter().zip(tokens.into_iter()) {
            assert_eq!(
                expected_token, actual_token,
                "期待するトークンと実際のトークンが異なります。"
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
                lexeme: vec!['('],
                literal: None,
                line: 2,
            },
            Token {
                ty: TokenType::RParan,
                lexeme: vec![')'],
                literal: None,
                line: 4,
            },
            Token {
                ty: TokenType::Eof,
                lexeme: vec![],
                literal: None,
                line: 5,
            },
        ];

        let tokens = scan_tokens(input).expect("スキャンに失敗しました。");
        assert_eq!(
            expected.len(),
            tokens.len(),
            "トークンの数が期待と異なります。"
        );

        for (expected_token, actual_token) in expected.into_iter().zip(tokens.into_iter()) {
            assert_eq!(
                expected_token, actual_token,
                "期待するトークンと実際のトークンが異なります。"
            );
        }
    }
}
