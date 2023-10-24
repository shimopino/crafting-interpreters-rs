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
            '/' => self.add_token(TokenType::Slash, "/".to_string()),
            '*' => self.add_token(TokenType::Star, "*".to_string()),
            _ => unimplemented!(),
        };
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, ty: TokenType, literal: String) {
        self.tokens.push(Token {
            ty,
            literal,
            lexeme: String::new(),
            line: self.line,
        })
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::Scanner;

    #[test]
    fn test_one_char_token() {
        let input = "(";

        let expected = vec![
            Token {
                ty: TokenType::LParan,
                literal: "(".to_string(),
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

        assert_eq!(scanner.tokens, expected);
    }
}
