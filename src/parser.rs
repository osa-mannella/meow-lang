use crate::types::{ast::*, constants::Precedence, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            self.skip_newlines();
            if !self.is_at_end() {
                statements.push(self.statement()?);
            }
        }
        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        let line = self.current_line();
        match self.current() {
            Token::Let | Token::LetBang => self.let_statement(line),
            Token::Func => self.func_statement(line),
            _ => Ok(Stmt::Expr(
                self.expression(Precedence::Pipeline.as_u8())?,
                line,
            )),
        }
    }

    fn let_statement(&mut self, line: usize) -> Result<Stmt, String> {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => {
                return Err(format!(
                    "Expected identifier at line {}",
                    self.current_line()
                ));
            }
        };
        self.expect(Token::Assign)?;
        let value = self.expression(Precedence::Pipeline.as_u8())?;
        Ok(Stmt::Let { name, value, line })
    }

    fn func_statement(&mut self, line: usize) -> Result<Stmt, String> {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(n) => n,
            _ => {
                return Err(format!(
                    "Expected identifier at line {}",
                    self.current_line()
                ));
            }
        };
        self.expect(Token::LeftParen)?;
        let mut params = Vec::new();
        while !matches!(self.current(), Token::RightParen) {
            if let Token::Identifier(p) = self.advance() {
                params.push(p);
            }
            if matches!(self.current(), Token::Comma) {
                self.advance();
            }
        }
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        let mut body = Vec::new();
        while !matches!(self.current(), Token::RightBrace) {
            self.skip_newlines();
            if !matches!(self.current(), Token::RightBrace) {
                body.push(self.statement()?);
            }
        }
        self.expect(Token::RightBrace)?;
        Ok(Stmt::Func {
            name,
            params,
            body,
            line,
        })
    }

    fn expression(&mut self, min_prec: u8) -> Result<Expr, String> {
        let mut left = self.nud()?;
        while self.precedence(false)? >= min_prec {
            left = self.led(left)?;
        }
        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Identifier(s) => Ok(Expr::Identifier(s)),
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::String(s) => Ok(Expr::String(s)),
            Token::LeftParen => {
                let expr = self.expression(Precedence::Pipeline.as_u8())?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Token::Minus => {
                let right = self.expression(Precedence::Unary.as_u8())?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    right: Box::new(right),
                })
            }
            Token::Not => {
                let right = self.expression(Precedence::Unary.as_u8())?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    right: Box::new(right),
                })
            }
            Token::LeftBracket => {
                let mut elements = Vec::new();

                // Handle empty array
                if matches!(self.current(), Token::RightBracket) {
                    self.advance();
                    return Ok(Expr::Array { elements });
                }

                // Parse array elements [expr, expr, ...]
                loop {
                    elements.push(self.expression(Precedence::Pipeline.as_u8())?);

                    match self.current() {
                        Token::Comma => {
                            self.advance();
                            // Allow trailing comma [1, 2, 3,]
                            if matches!(self.current(), Token::RightBracket) {
                                break;
                            }
                        }
                        Token::RightBracket => break,
                        _ => {
                            return Err(format!(
                                "Expected ',' or ']' in array literal at line {}",
                                self.current_line()
                            ));
                        }
                    }
                }

                self.expect(Token::RightBracket)?;
                Ok(Expr::Array { elements })
            }
            Token::True => Ok(Expr::Boolean(true)),
            Token::False => Ok(Expr::Boolean(false)),
            t => Err(format!(
                "Unexpected token in nud: {:?} at line {}",
                t,
                self.current_line()
            )),
        }
    }

    fn led(&mut self, left: Expr) -> Result<Expr, String> {
        match self.current() {
            Token::Plus
            | Token::Minus
            | Token::Multiply
            | Token::Divide
            | Token::Equal
            | Token::NotEqual
            | Token::Less
            | Token::Greater
            | Token::LessEqual
            | Token::GreaterEqual => {
                let op = self.binary_op()?;
                self.advance();
                let right = self.expression(self.precedence(true)? + 1)?;
                Ok(Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
            }
            Token::LeftParen => {
                self.advance();
                let mut args = Vec::new();
                while !matches!(self.current(), Token::RightParen) {
                    args.push(self.expression(Precedence::Pipeline.as_u8())?);
                    if matches!(self.current(), Token::Comma) {
                        self.advance();
                    }
                }
                self.expect(Token::RightParen)?;
                Ok(Expr::Call {
                    func: Box::new(left),
                    args,
                })
            }
            Token::Pipeline => {
                self.advance();
                let right = self.expression(self.precedence(true)? + 1)?;
                Ok(Expr::Pipeline {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Token::Update => {
                self.advance();
                // Make update right-associative: parse RHS with same precedence
                let right = self.expression(self.precedence(true)?)?;
                Ok(Expr::Update {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => Ok(left),
        }
    }

    fn binary_op(&self) -> Result<BinaryOp, String> {
        match self.current() {
            Token::Plus => Ok(BinaryOp::Add),
            Token::Minus => Ok(BinaryOp::Sub),
            Token::Multiply => Ok(BinaryOp::Mul),
            Token::Divide => Ok(BinaryOp::Div),
            Token::Equal => Ok(BinaryOp::Eq),
            Token::NotEqual => Ok(BinaryOp::Ne),
            Token::Less => Ok(BinaryOp::Lt),
            Token::Greater => Ok(BinaryOp::Gt),
            Token::LessEqual => Ok(BinaryOp::Le),
            Token::GreaterEqual => Ok(BinaryOp::Ge),
            _ => Err(format!(
                "Not a binary operator: {:?} at line {}",
                self.current(),
                self.current_line()
            )),
        }
    }

    fn precedence(&self, right_parse: bool) -> Result<u8, String> {
        match self.current() {
            Token::Pipeline | Token::Update => Ok(Precedence::Pipeline.as_u8()),
            Token::Equal
            | Token::NotEqual
            | Token::Less
            | Token::Greater
            | Token::LessEqual
            | Token::GreaterEqual => Ok(Precedence::Comparison.as_u8()),
            Token::Plus | Token::Minus => Ok(Precedence::Term.as_u8()),
            Token::Multiply | Token::Divide => Ok(Precedence::Factor.as_u8()),
            Token::LeftParen => Ok(Precedence::Unary.as_u8()),
            Token::String(_)
            | Token::Number(_)
            | Token::Identifier(_)
            | Token::True
            | Token::False
            | Token::LeftBracket
            | Token::LeftBrace => {
                if right_parse {
                    return Ok(Precedence::Lowest.as_u8());
                } else {
                    return Err(format!(
                        "Invalid hanging literal: {:?} at line {}",
                        self.current(),
                        self.current_line()
                    ));
                }
            }
            _ => Ok(Precedence::Lowest.as_u8()),
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) -> Token {
        let token = self.current().clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.current()) != std::mem::discriminant(&expected) {
            return Err(format!(
                "Expected {:?}, found {:?} at line {}",
                expected,
                self.current(),
                self.current_line()
            ));
        }
        self.advance();
        Ok(())
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current(), Token::Newline) {
            self.advance();
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.skip_newlines();
        matches!(self.current(), Token::Eof)
    }

    fn current_line(&self) -> usize {
        let mut line = 1;
        for t in self.tokens.iter().take(self.pos) {
            if matches!(t, Token::Newline) {
                line += 1;
            }
        }
        line
    }
}
