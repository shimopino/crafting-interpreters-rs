use crate::token_refactor::Token;

/// `Scanner`は、入力された文字列をトークンの配列に解析するための構造体
pub struct Scanner {
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
    todo!()
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            source: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{scanner_refactor::scan_tokens, token_refactor::Token, token_refactor::TokenType};

    #[test]
    fn test_one_char_token() {
        let input = "{";
        // let input = "{}(),.-+;/*";

        let expected = vec![Token {
            ty: TokenType::LBrace,
            lexeme: vec!['{'],
            literal: None,
            line: 0,
        }];

        let result = scan_tokens(input);
        assert_eq!(Ok(expected), result);
    }
}
