use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Var(String),
    Binary {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: String,
        operand: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        type_: String,
        expr: Expr,
    },
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    Return(Option<Expr>),
    Print(Expr),
    ExprStmt(Expr),
    FnDecl {
        name: String,
        params: Vec<(String, String)>,
        return_type: Option<String>,
        body: Vec<Stmt>,
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Stmt>,
    pub main_body: Vec<Stmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek_kind(&self, offset: usize) -> TokenType {
        let i = self.pos + offset;
        if i < self.tokens.len() {
            self.tokens[i].token_type.clone()
        } else {
            TokenType::Eof
        }
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok
    }

    fn check(&self, kind: TokenType) -> bool {
        self.peek().token_type == kind
    }

    fn match_(&mut self, kinds: &[TokenType]) -> bool {
        if kinds.iter().any(|k| self.peek().token_type == *k) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenType) -> Result<&Token, String> {
        if self.peek().token_type != kind {
            let tok = self.peek();
            Err(format!(
                "line {}: expected {:?}, got {:?} ({:?})",
                tok.line, kind, tok.token_type, tok.value
            ))
        } else {
            Ok(self.advance())
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();
        let mut main_body = Vec::new();
        while !self.check(TokenType::Eof) {
            if self.check(TokenType::Fun) {
                functions.push(self.parse_fn()?);
            } else {
                main_body.push(self.parse_stmt()?);
            }
        }
        Ok(Program { functions, main_body })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        if self.check(TokenType::LBrace) {
            self.advance();
            let mut stmts = Vec::new();
            while !self.check(TokenType::RBrace) && !self.check(TokenType::Eof) {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(TokenType::RBrace)?;
            return Ok(Stmt::Block(stmts));
        }
        if self.check(TokenType::Let) {
            return self.parse_let();
        }
        if self.check(TokenType::If) {
            return self.parse_if();
        }
        if self.check(TokenType::While) {
            return self.parse_while();
        }
        if self.check(TokenType::Return) {
            return self.parse_return();
        }
        if self.check(TokenType::Return) {
            return self.parse_return();
        }
        if self.check(TokenType::Print) {
            return self.parse_print();
        }
        self.parse_assign_or_expr()
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Let)?;
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(n) => n.clone(),
            _ => return Err(format!("line {}: expected identifier, got {:?}", name_token.line, name_token.token_type)),
        };
        self.expect(TokenType::Colon)?;
        let type_ = self.parse_type()?;
        self.expect(TokenType::Eq)?;
        let expr = self.parse_expr()?;
        self.expect(TokenType::Semi)?;
        Ok(Stmt::Let { name, type_, expr })
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::If)?;
        self.expect(TokenType::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(TokenType::RParen)?;
        let then_branch = Box::new(self.parse_stmt()?);
        let else_branch = if self.check(TokenType::Else) {
            self.advance();
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };
        Ok(Stmt::If { cond, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::While)?;
        self.expect(TokenType::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(TokenType::RParen)?;
        let body = Box::new(self.parse_stmt()?);
        Ok(Stmt::While { cond, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Return)?;
        let expr = if self.check(TokenType::Semi) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        self.expect(TokenType::Semi)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Print)?;
        self.expect(TokenType::LParen)?;
        let expr = self.parse_expr()?;
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::Semi)?;
        Ok(Stmt::Print(expr))
    }

    fn parse_assign_or_expr(&mut self) -> Result<Stmt, String> {
        if let TokenType::Ident(_) = self.peek().token_type {
            if self.peek_kind(1) == TokenType::Eq {
                let name_token = self.advance();
                let name = match &name_token.token_type {
                    TokenType::Ident(n) => n.clone(),
                    _ => return Err(format!("line {}: expected identifier, got {:?}", name_token.line, name_token.token_type)),
                };
                self.expect(TokenType::Eq)?;
                let expr = self.parse_expr()?;
                self.expect(TokenType::Semi)?;
                return Ok(Stmt::ExprStmt(Expr::Binary { 
                    op: "=".to_string(), 
                    left: Box::new(Expr::Var(name)), 
                    right: Box::new(expr)
                }));
            }
        }
        let expr = self.parse_expr()?;
        self.expect(TokenType::Semi)?;
        Ok(Stmt::ExprStmt(expr))
    }

    fn parse_fn(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::Fun)?;
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(n) => n.clone(),
            _ => return Err(format!("line {}: expected identifier, got {:?}", name_token.line, name_token.token_type)),
        };
        self.expect(TokenType::LParen)?;
        let mut params = Vec::new();
        if !self.check(TokenType::RParen) {
            params.push(self.parse_param()?);
            while self.check(TokenType::Comma) {
                self.advance();
                params.push(self.parse_param()?);
            }
        }
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::Colon)?;
        let return_type = Some(self.parse_type()?);
        self.expect(TokenType::LBrace)?;
        let mut body = Vec::new();
        while !self.check(TokenType::RBrace) && !self.check(TokenType::Eof) {
            body.push(self.parse_stmt()?);
        }
        self.expect(TokenType::RBrace)?;
        Ok(Stmt::FnDecl { name, params, return_type, body })
    }

    fn parse_param(&mut self) -> Result<(String, String), String> {
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Ident(n) => n.clone(),
            _ => return Err(format!("line {}: expected identifier, got {:?}", name_token.line, name_token.token_type)),
        };
        self.expect(TokenType::Colon)?;
        let type_ = self.parse_type()?;
        Ok((name, type_))
    }

    // --- Expression Parsers ---

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.check(TokenType::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                op: "||".to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.check(TokenType::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                op: "&&".to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        while self.check(TokenType::Eq) || self.check(TokenType::Bang) {
            let op_tok = self.advance().token_type.clone();
            let op = match op_tok {
                TokenType::Eq => "==",
                TokenType::Bang => "!=",
                _ => unreachable!(),
            }.to_string();

            let right = self.parse_comparison()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while self.check(TokenType::Lt) 
            || self.check(TokenType::Le) 
            || self.check(TokenType::Gt) 
            || self.check(TokenType::Ge) 
        {
            let op_tok = self.advance().token_type.clone();
            let op = match op_tok {
                TokenType::Lt => "<",
                TokenType::Le => "<=",
                TokenType::Gt => ">",
                TokenType::Ge => ">=",
                _ => unreachable!(),
            }.to_string();

            let right = self.parse_term()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        while self.check(TokenType::Plus) || self.check(TokenType::Minus) {
            let op_tok = self.advance().token_type.clone();
            let op = match op_tok {
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                _ => unreachable!(),
            }.to_string();

            let right = self.parse_factor()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while self.check(TokenType::Star) || self.check(TokenType::Slash) {
            let op_tok = self.advance().token_type.clone();
            let op = match op_tok {
                TokenType::Star => "*",
                TokenType::Slash => "/",
                _ => unreachable!(),
            }.to_string();

            let right = self.parse_unary()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.check(TokenType::Bang) || self.check(TokenType::Minus) {
            let op_tok = self.advance().token_type.clone();
            let op = match op_tok {
                TokenType::Bang => "!",
                TokenType::Minus => "-",
                _ => unreachable!(),
            }.to_string();

            let operand = self.parse_unary()?;
            Ok(Expr::Unary {
                op,
                operand: Box::new(operand),
            })
        } else {
            self.parse_call()
        }
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        while self.check(TokenType::LParen) {
            self.advance();
            let mut args = Vec::new();
            if !self.check(TokenType::RParen) {
                args.push(self.parse_expr()?);
                while self.check(TokenType::Comma) {
                    self.advance();
                    args.push(self.parse_expr()?);
                }
            }
            self.expect(TokenType::RParen)?;
            match expr {
                Expr::Var(name) => {
                    expr = Expr::Call { name, args };
                }
                _ => return Err(format!("line {}: expected function name before '(', got {:?}", self.peek().line, expr)),
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let tok = self.advance().clone();
        match tok.token_type {
            TokenType::Number(n) => Ok(Expr::Number(n as i64)),
            TokenType::True => Ok(Expr::Bool(true)),
            TokenType::False => Ok(Expr::Bool(false)),
            TokenType::Ident(name) => Ok(Expr::Var(name)),
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("line {}: unexpected token {:?} ({:?})", tok.line, tok.token_type, tok.value)),
        }
    }

    fn parse_type(&mut self) -> Result<String, String> {
        let type_token = self.advance();
        match &type_token.token_type {
            TokenType::Ident(t) => Ok(t.clone()),
            TokenType::IntKw => Ok("int".to_string()),
            TokenType::BoolKw => Ok("bool".to_string()),
            _ => Err(format!("line {}: expected type identifier, got {:?}", type_token.line, type_token.token_type)),
        }
    }
}