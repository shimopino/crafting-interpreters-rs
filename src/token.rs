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
