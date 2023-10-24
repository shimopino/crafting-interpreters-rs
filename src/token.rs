#[derive(PartialEq, Debug)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // 記号1個のトークン
    LParan,
    RParan,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // 記号1個、または2個によるトークン
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // リテラル
    Identifier,
    String,
    Number,

    // キーワード
    And,
    Or,
    If,
    Else,
    True,
    False,
    For,
    While,
    Nil,
    Fun,
    Return,
    Class,
    Super,
    This,
    Var,

    // End of file
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;

        match self {
            LParan => write!(f, "("),
            RParan => write!(f, ")"),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            SemiColon => write!(f, ";"),
            Slash => write!(f, "/"),
            Star => write!(f, "*"),
            _ => todo!(),
        }
    }
}
