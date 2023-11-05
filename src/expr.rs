#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Number,
    String,
    True,
    False,
    Nil,
}

#[derive(PartialEq, Debug)]
pub enum UnaryOp {
    Bang,
    Minus,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOp {
    // 中値演算子
    Plus,
    Minus,
    Star,
    Slash,
    // 論理演算子
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
