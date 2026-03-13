use valea::{
    ast::{Expr, Type},
    check_source,
    codegen::emit_c,
    diagnostics::{offset_to_line_col, Diagnostic},
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

#[test]
fn emits_c_with_forward_declarations() {
    // b() is defined before a(), so without a forward declaration the C compiler
    // would see an undeclared function call inside b.
    let src = "fn b() -> int { a() }\nfn a() -> int { 1 }";
    let program = check_source(src).expect("type check should pass");
    let c = emit_c(&program);
    // Forward declaration must appear before the first function body.
    let fwd_pos = c.find("long b(void);").expect("forward decl for b");
    let body_pos = c.find("long b(void) {").expect("definition of b");
    assert!(fwd_pos < body_pos, "forward declarations must precede definitions");
    assert!(c.contains("long a(void);"), "forward decl for a");
}

#[test]
fn offset_to_line_col_basic() {
    let src = "fn a() -> int {\n    1\n}";
    assert_eq!(offset_to_line_col(src, 0), (1, 1));
    assert_eq!(offset_to_line_col(src, 16), (2, 1)); // char after first '\n'
}

#[test]
fn render_human_with_source_shows_line_col() {
    let src = "fn a() -> bool { 1 }";
    let d = check_source(src).expect_err("should have type error");
    let rendered = d[0].render_human_with_source(src);
    // Should contain "line:col:" prefix, not raw byte offsets.
    assert!(rendered.starts_with("1:"), "expected line number at start: {rendered}");
}
