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

        let matching_literal = match self {
            LParan => "(",
            RParan => ")",
            LBrace => "{{",
            RBrace => "}}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            SemiColon => ";",
            Slash => "/",
            Star => "*",
            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Greater => ">",
            GreaterEqual => ">=",
            Less => "<",
            LessEqual => "<=",
            Identifier => "Identifier",
            String => "String",
            Number => "Number",
            And => "and",
            Or => "or",
            If => "if",
            Else => "else",
            True => "true",
            False => "false",
            For => "for",
            While => "while",
            Nil => "nil",
            Fun => "fun",
            Return => "return",
            Class => "class",
            Super => "super",
            This => "this",
            Var => "var",
            Eof => "eof",
        };

        write!(f, "{matching_literal}")
    }
}
