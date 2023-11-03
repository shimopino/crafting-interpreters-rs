use crate::token_refactor::{Token, TokenType};

/// `Scanner`は、入力された文字列をトークンの配列に解析するための構造体
struct Scanner {
    /// 入力文字列を保持する
    /// マルチバイトのUTF-8文字も安全に取り扱えるように char 型として保持する
    pub source: Vec<char>,
    /// スキャン中のトークンの最初の文字の位置を指す
    pub start: usize,
    /// スキャン中に注目している文字を指す
    pub current: usize,
    /// `current`が入力文字列の何行目に当たるのかを追跡管理する
    pub line: usize,
}

pub fn scan_tokens(input: &str) -> Result<Vec<Token>, String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    Ok(tokens)
}

impl Scanner {
    fn new(input: &str) -> Self {
        Scanner {
            source: input.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            tokens.push(token)
        }

        tokens.push(Token {
            ty: TokenType::Eof,
            lexeme: vec![],
            literal: None,
            line: self.line,
        });
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, String> {
        let c = self.advance();
        let token = match c {
            '{' => self.create_token(TokenType::LBrace),
            '}' => self.create_token(TokenType::RBrace),
            '(' => self.create_token(TokenType::LParan),
            ')' => self.create_token(TokenType::RParan),
            ',' => self.create_token(TokenType::Comma),
            '.' => self.create_token(TokenType::Dot),
            '-' => self.create_token(TokenType::Minus),
            '+' => self.create_token(TokenType::Plus),
            ';' => self.create_token(TokenType::SemiColon),
            '/' => self.create_token(TokenType::Slash),
            '*' => self.create_token(TokenType::Star),
            _ => return Err(String::from("invalid token")),
        };

        Ok(token)
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

    fn create_token(&self, ty: TokenType) -> Token {
        Token {
            ty,
            lexeme: self.source[self.start..self.current].to_vec(),
            literal: None,
            line: self.line,
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

        let result = scan_tokens(input);
        assert_eq!(Ok(expected), result);
    }
}
