use crate::token::Token;

/// `Scanner`は、入力された文字列をトークンの配列に解析するための構造体
pub struct Scanner {
    /// 入力文字列を保持する
    source: String,
    /// 入力文字列を該当するトークンに変換した配列を保持する
    tokens: Vec<Token>,
    /// スキャン中のトークンの最初の文字の位置を指す
    start: usize,
    /// スキャン中に注目している文字を指す
    current: usize,
    /// `current`が入力文字列の何行目に当たるのかを追跡管理する
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            source: String::new(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        todo!()
    }
}
