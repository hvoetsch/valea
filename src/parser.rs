use crate::{
    ast::{Expr, FunctionDecl, Program, Type},
    diagnostics::Diagnostic,
    lexer::{Token, TokenKind},
};

pub fn parse(tokens: &[Token]) -> Result<Program, Vec<Diagnostic>> {
    let mut p = Parser {
        tokens,
        pos: 0,
        diagnostics: Vec::new(),
    };
    let mut functions = Vec::new();
    while !p.is_eof() {
        if let Some(function) = p.parse_function() {
            functions.push(function);
        } else {
            p.synchronize();
        }
    }

    if p.diagnostics.is_empty() {
        Ok(Program { functions })
    } else {
        Err(p.diagnostics)
    }
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    fn parse_function(&mut self) -> Option<FunctionDecl> {
        self.expect_exact(TokenKind::Fn, "E100", "Expected 'fn' to start a function")?;
        let name = self.expect_ident()?;
        self.expect_exact(
            TokenKind::LParen,
            "E101",
            "Expected '(' after function name",
        )?;
        self.expect_exact(
            TokenKind::RParen,
            "E102",
            "Parameters are not supported yet; expected ')'",
        )?;
        self.expect_exact(TokenKind::Arrow, "E103", "Expected '->' before return type")?;
        let return_type = self.expect_type()?;
        self.expect_exact(
            TokenKind::LBrace,
            "E104",
            "Expected '{' to start function body",
        )?;
        let body = self.parse_expr()?;
        self.expect_exact(
            TokenKind::RBrace,
            "E105",
            "Expected '}' to end function body",
        )?;
        Some(FunctionDecl {
            name,
            return_type,
            body,
        })
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        while self.match_exact(TokenKind::Plus) {
            let right = self.parse_primary()?;
            expr = Expr::Add {
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let token = self.peek()?.clone();
        match token.kind {
            TokenKind::Integer(v) => {
                self.pos += 1;
                Some(Expr::Integer(v))
            }
            TokenKind::True => {
                self.pos += 1;
                Some(Expr::Bool(true))
            }
            TokenKind::False => {
                self.pos += 1;
                Some(Expr::Bool(false))
            }
            TokenKind::Ident(ref name) => {
                self.pos += 1;
                if self.match_exact(TokenKind::LParen) {
                    self.expect_exact(
                        TokenKind::RParen,
                        "E107",
                        "Expected ')' after function call",
                    )?;
                    Some(Expr::Call {
                        callee: name.clone(),
                    })
                } else {
                    self.diagnostics.push(Diagnostic::new(
                        "E106",
                        "Only zero-argument function calls are supported in expressions",
                        token.start,
                        token.end,
                    ));
                    None
                }
            }
            _ => {
                self.diagnostics.push(Diagnostic::new(
                    "E108",
                    "Expected an expression",
                    token.start,
                    token.end,
                ));
                None
            }
        }
    }

    fn expect_ident(&mut self) -> Option<String> {
        let token = self.peek()?.clone();
        if let TokenKind::Ident(name) = token.kind {
            self.pos += 1;
            Some(name)
        } else {
            self.diagnostics.push(Diagnostic::new(
                "E109",
                "Expected an identifier",
                token.start,
                token.end,
            ));
            None
        }
    }

    fn expect_type(&mut self) -> Option<Type> {
        let token = self.peek()?.clone();
        match token.kind {
            TokenKind::Int => {
                self.pos += 1;
                Some(Type::Int)
            }
            TokenKind::Bool => {
                self.pos += 1;
                Some(Type::Bool)
            }
            _ => {
                self.diagnostics.push(Diagnostic::new(
                    "E110",
                    "Expected return type 'int' or 'bool'",
                    token.start,
                    token.end,
                ));
                None
            }
        }
    }

    fn expect_exact(&mut self, kind: TokenKind, code: &'static str, message: &str) -> Option<()> {
        let token = self.peek()?.clone();
        if token.kind == kind {
            self.pos += 1;
            Some(())
        } else {
            self.diagnostics
                .push(Diagnostic::new(code, message, token.start, token.end));
            None
        }
    }

    fn match_exact(&mut self, kind: TokenKind) -> bool {
        if let Some(token) = self.peek() {
            if token.kind == kind {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::Fn {
                break;
            }
            self.pos += 1;
        }
    }
}
