use valea::{
    ast::{Expr, Type},
    check_source,
    codegen::emit_c,
    diagnostics::Diagnostic,
    formatter::format_program,
    json::diagnostics_json,
    lexer::{lex, TokenKind},
    parse_source,
};

#[test]
fn lexes_basic_program() {
    let tokens = lex("fn a() -> int { 1 + 2 }").expect("lex should succeed");
    assert!(matches!(tokens[0].kind, TokenKind::Fn));
    assert!(matches!(tokens[1].kind, TokenKind::Ident(_)));
    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Plus)));
}

#[test]
fn parses_function_and_addition() {
    let program = parse_source("fn a() -> int { 1 + 2 }").expect("parse should succeed");
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].return_type, Type::Int);
    assert!(matches!(program.functions[0].body, Expr::Add { .. }));
}

#[test]
fn formats_canonically() {
    let source = "fn a() -> int {1+2}";
    let program = parse_source(source).expect("parse should succeed");
    let formatted = format_program(&program);
    assert_eq!(formatted, "fn a() -> int {\n    1 + 2\n}\n");
}

#[test]
fn detects_type_errors() {
    let diags = check_source("fn a() -> int { true }").expect_err("must fail");
    assert!(diags.iter().any(|d| d.code == "E201"));
}

#[test]
fn diagnostics_emit_json() {
    let d = Diagnostic::new("E999", "example", 1, 2);
    let json = diagnostics_json(&[d]);
    assert!(json.contains("\"E999\""));
}

#[test]
fn emits_c_for_valid_program() {
    let program = check_source("fn a() -> int { 1 + 2 }").expect("type check should pass");
    let c = emit_c(&program);
    assert!(c.contains("long a(void)"));
    assert!(c.contains("(1 + 2)"));
}
