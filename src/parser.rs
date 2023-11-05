use crate::{
    expr::{BinaryOp, Expr, Literal, UnaryOp},
    token::{Token, TokenType},
};

/// 構文解析器を表す構造体です
///
/// C言語と同じ優先順位と結合度を採用し、以下の式文法に従って解析を進めていく
///
/// * expression -> equality
/// * equality   -> comparison ( ("!=" | "==") comparison )* ;
/// * comparison -> term ( (">" | ">=" | "<" | "<=") term )* ;
/// * term       -> factor ( ("-" | "+") factor )* ;
/// * factor     -> unary ( ("/" | "*") unary )* ;
/// * unary      -> ("!" | "-") unary
///               | primary ;
/// * primary    -> Number | String | "true" | "false" | "nil"
///               | "(" expression ")" ;
///
pub struct Parser {
    /// `Scanner` によって解析したトークンのシーケンス
    tokens: Vec<Token>,
    /// 次に解析すべきトークン位置
    current: usize,
}

/// 構文解析エラーを表すカスタムエラー型です。
///
/// このエラーは、解析中に発生した特定の問題を表すために使用されます。
/// `String`はエラーメッセージを保持します。
#[derive(PartialEq, Debug)]
pub struct ParserError(String);

impl std::error::Error for ParserError {}

/// `ParserError`の表示形式を定義します。
///
/// この実装により、`ParserError`は人間が読める形式で出力され、
/// デバッグやエラーログに役立ちます。
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: {}", self.0)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression().map_err(|e| {
            self.synchronize();
            e
        })
    }

    // expression -> equality
    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    // equality   -> comparison ( ("!=" | "==") comparison )* ;
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let binary_op = parse_binary_op(operator)?;
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right));
        }

        Ok(expr)
    }

    // comparison -> term ( (">" | ">=" | "<" | "<=") term )* ;
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let binary_op = parse_binary_op(operator)?;
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right));
        }

        Ok(expr)
    }

    // term       -> factor ( ("-" | "+") factor )* ;
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let binary_op = parse_binary_op(operator)?;
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right));
        }

        Ok(expr)
    }

    // factor     -> unary ( ("/" | "*") unary )* ;
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let binary_op = parse_binary_op(operator)?;
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), binary_op, Box::new(right));
        }

        Ok(expr)
    }

    // unary      -> ("!" | "-") unary
    //             | primary ;
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let unary_op = parse_unary_op(operator)?;
            let right = self.unary()?;
            return Ok(Expr::Unary(unary_op, Box::new(right)));
        }

        self.primary()
    }

    // primary    -> Number | String | "true" | "false" | "nil"
    //             | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.matches(&[TokenType::Number]) {
            return Ok(Expr::Literal(Literal::Number));
        }
        if self.matches(&[TokenType::String]) {
            return Ok(Expr::Literal(Literal::String));
        }
        if self.matches(&[TokenType::LParan]) {
            let expr = self.expression()?;
            self.consume(TokenType::RParan, "expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let next_token = self.peek();
        match next_token.ty {
            TokenType::Eof => Err(ParserError(format!(
                "token line {}, lexeme: {:?}, error {}",
                next_token.line, next_token.lexeme, "Expect expression"
            ))),
            _ => Err(ParserError(format!(
                "token line {} at end, error {}",
                next_token.line, "Expect expression"
            ))),
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for ty in types.iter() {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ty: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().ty == *ty;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn consume(&mut self, ty: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(&ty) {
            return Ok(self.advance());
        }

        let next_token = self.peek();
        match next_token.ty {
            TokenType::Eof => Err(ParserError(format!(
                "token line {}, lexeme: {:?}, error {}",
                next_token.line, next_token.lexeme, message
            ))),
            _ => Err(ParserError(format!(
                "token line {} at end, error {}",
                next_token.line, message
            ))),
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().ty == TokenType::SemiColon {
                return;
            }

            match self.peek().ty {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}

fn parse_binary_op(token: &Token) -> Result<BinaryOp, ParserError> {
    let binary_op = match token.ty {
        // 中値演算子
        TokenType::Plus => BinaryOp::Plus,
        TokenType::Minus => BinaryOp::Minus,
        TokenType::Star => BinaryOp::Star,
        TokenType::Slash => BinaryOp::Slash,
        // 論理演算子
        TokenType::EqualEqual => BinaryOp::EqualEqual,
        TokenType::BangEqual => BinaryOp::BangEqual,
        TokenType::Greater => BinaryOp::Greater,
        TokenType::GreaterEqual => BinaryOp::GreaterEqual,
        TokenType::Less => BinaryOp::Less,
        TokenType::LessEqual => BinaryOp::LessEqual,
        _ => return Err(ParserError("should be binaryOp".to_string())),
    };

    Ok(binary_op)
}

fn parse_unary_op(token: &Token) -> Result<UnaryOp, ParserError> {
    let unary_op = match token.ty {
        TokenType::Bang => UnaryOp::Bang,
        TokenType::Minus => UnaryOp::Minus,
        _ => return Err(ParserError("should be unaryOp".to_string())),
    };

    Ok(unary_op)
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::{BinaryOp, Expr, Literal, UnaryOp},
        parser::{parse_binary_op, ParserError},
        scanner::scan_tokens,
        token::{Token, TokenType},
    };

    use super::{parse_unary_op, Parser};

    #[test]
    fn test_parse_unary_op() {
        let unary_op = parse_unary_op(&Token {
            ty: TokenType::Minus,
            lexeme: vec!['-'],
            literal: None,
            line: 1,
        })
        .expect("Failed to parse Token");
        assert_eq!(UnaryOp::Minus, unary_op);

        let error = parse_unary_op(&Token {
            ty: TokenType::Plus,
            lexeme: vec!['+'],
            literal: None,
            line: 1,
        })
        .expect_err("Unexpectedly Success to parse Token");
        assert_eq!(ParserError(format!("should be unaryOp")), error);
    }

    #[test]
    fn test_parse_binary_op() {
        let binary_op = parse_binary_op(&Token {
            ty: TokenType::EqualEqual,
            lexeme: vec!['=', '='],
            literal: None,
            line: 1,
        })
        .expect("Failed to parse Token");
        assert_eq!(BinaryOp::EqualEqual, binary_op);

        let error = parse_binary_op(&Token {
            ty: TokenType::Bang,
            lexeme: vec!['!'],
            literal: None,
            line: 1,
        })
        .expect_err("Unexpectedly Success to parse Token");
        assert_eq!(ParserError(format!("should be binaryOp")), error);
    }

    #[test]
    fn test_simple_tokens() {
        let input = "2 + 3";

        let tokens = scan_tokens(input).expect("Failed to scan input string");
        println!("{tokens:?}");

        let expr = Parser::new(tokens).parse().expect("Failed to parse Tokens");

        assert_eq!(
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number)),
                BinaryOp::Plus,
                Box::new(Expr::Literal(Literal::Number)),
            ),
            expr
        );
    }
}
