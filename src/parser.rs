use crate::{
    expr::{BinaryOp, Expr, Literal, UnaryOp},
    token::{Token, TokenType},
};

pub struct Parser {
    /// `Scanner` によって解析したトークンのシーケンス
    tokens: Vec<Token>,
    /// 次に解析すべきトークン位置
    current: usize,
}

#[derive(Debug)]
struct ParserError(String);

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: {}", self.0)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(e) => {
                self.synchronize();
                println!("{e}");
                None
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

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

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let unary_op = parse_unary_op(operator)?;
            let right = self.unary()?;
            return Ok(Expr::Unary(unary_op, Box::new(right)));
        }

        Ok(self.primary()?)
    }

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
        _ => return Err(ParserError(format!("should be binaryOp"))),
    };

    Ok(binary_op)
}

fn parse_unary_op(token: &Token) -> Result<UnaryOp, ParserError> {
    let unary_op = match token.ty {
        TokenType::Bang => UnaryOp::Bang,
        TokenType::Minus => UnaryOp::Minus,
        _ => return Err(ParserError(format!("should be unaryOp"))),
    };

    Ok(unary_op)
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::{BinaryOp, Expr, Literal},
        scanner::scan_tokens,
    };

    use super::Parser;

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
