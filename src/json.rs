use crate::{
    ast::{Expr, Program, Type},
    diagnostics::Diagnostic,
};

pub fn diagnostics_json(diags: &[Diagnostic]) -> String {
    let items: Vec<String> = diags
        .iter()
        .map(|d| {
            format!(
                "{{\"code\":\"{}\",\"message\":\"{}\",\"span\":{{\"start\":{},\"end\":{}}}}}",
                d.code,
                escape(&d.message),
                d.span.start,
                d.span.end
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

pub fn ast_json(program: &Program) -> String {
    let functions: Vec<String> = program
        .functions
        .iter()
        .map(|f| {
            format!(
                "{{\"name\":\"{}\",\"return_type\":\"{}\",\"body\":{}}}",
                escape(&f.name),
                type_name(&f.return_type),
                expr_json(&f.body)
            )
        })
        .collect();
    format!("{{\"functions\":[{}]}}", functions.join(","))
}

fn expr_json(expr: &Expr) -> String {
    match expr {
        Expr::Integer(v) => format!("{{\"kind\":\"Integer\",\"value\":{}}}", v),
        Expr::Bool(v) => format!("{{\"kind\":\"Bool\",\"value\":{}}}", v),
        Expr::Call { callee } => {
            format!("{{\"kind\":\"Call\",\"callee\":\"{}\"}}", escape(callee))
        }
        Expr::Add { left, right } => format!(
            "{{\"kind\":\"Add\",\"left\":{},\"right\":{}}}",
            expr_json(left),
            expr_json(right)
        ),
    }
}

fn type_name(ty: &Type) -> &'static str {
    match ty {
        Type::Int => "int",
        Type::Bool => "bool",
    }
}

fn escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}
