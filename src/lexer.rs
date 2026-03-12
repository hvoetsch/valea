use crate::diagnostics::Diagnostic;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Fn,
    True,
    False,
    Int,
    Bool,
    Ident(String),
    Integer(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Arrow,
    Plus,
}

pub fn lex(source: &str) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    let bytes = source.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        let start = i;
        match c {
            '(' => {
                tokens.push(tok(TokenKind::LParen, i, i + 1));
                i += 1;
            }
            ')' => {
                tokens.push(tok(TokenKind::RParen, i, i + 1));
                i += 1;
            }
            '{' => {
                tokens.push(tok(TokenKind::LBrace, i, i + 1));
                i += 1;
            }
            '}' => {
                tokens.push(tok(TokenKind::RBrace, i, i + 1));
                i += 1;
            }
            '+' => {
                tokens.push(tok(TokenKind::Plus, i, i + 1));
                i += 1;
            }
            '-' => {
                if i + 1 < bytes.len() && bytes[i + 1] as char == '>' {
                    tokens.push(tok(TokenKind::Arrow, i, i + 2));
                    i += 2;
                } else {
                    diagnostics.push(Diagnostic::new(
                        "E001",
                        "Unexpected '-' (did you mean '->'?)",
                        i,
                        i + 1,
                    ));
                    i += 1;
                }
            }
            d if d.is_ascii_digit() => {
                i += 1;
                while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                    i += 1;
                }
                let text = &source[start..i];
                match text.parse::<i64>() {
                    Ok(value) => tokens.push(tok(TokenKind::Integer(value), start, i)),
                    Err(_) => diagnostics.push(Diagnostic::new(
                        "E002",
                        "Integer literal is out of range for i64",
                        start,
                        i,
                    )),
                }
            }
            a if is_ident_start(a) => {
                i += 1;
                while i < bytes.len() && is_ident_continue(bytes[i] as char) {
                    i += 1;
                }
                let text = &source[start..i];
                let kind = match text {
                    "fn" => TokenKind::Fn,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "int" => TokenKind::Int,
                    "bool" => TokenKind::Bool,
                    _ => TokenKind::Ident(text.to_string()),
                };
                tokens.push(tok(kind, start, i));
            }
            _ => {
                diagnostics.push(Diagnostic::new(
                    "E003",
                    format!("Unexpected character '{}'", c),
                    i,
                    i + 1,
                ));
                i += 1;
            }
        }
    }

    if diagnostics.is_empty() {
        Ok(tokens)
    } else {
        Err(diagnostics)
    }
}

fn tok(kind: TokenKind, start: usize, end: usize) -> Token {
    Token { kind, start, end }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
