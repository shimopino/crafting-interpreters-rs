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
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Identifier => write!(f, "Identifier"),
            String => write!(f, "String"),
            Number => write!(f, "Number"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            For => write!(f, "for"),
            While => write!(f, "while"),
            Nil => write!(f, "nil"),
            Fun => write!(f, "fun"),
            Return => write!(f, "return"),
            Class => write!(f, "class"),
            Super => write!(f, "super"),
            This => write!(f, "this"),
            Var => write!(f, "var"),
            Eof => write!(f, "eof"),
        }
    }
}
