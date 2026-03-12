use crate::ast::{Expr, FunctionDecl, Program, Type};

pub fn format_program(program: &Program) -> String {
    let mut out = String::new();
    for (i, f) in program.functions.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&format_function(f));
        out.push('\n');
    }
    out
}

fn format_function(function: &FunctionDecl) -> String {
    format!(
        "fn {}() -> {} {{\n    {}\n}}",
        function.name,
        format_type(&function.return_type),
        format_expr(&function.body)
    )
}

fn format_type(ty: &Type) -> &'static str {
    match ty {
        Type::Int => "int",
        Type::Bool => "bool",
    }
}

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Integer(v) => v.to_string(),
        Expr::Bool(v) => v.to_string(),
        Expr::Call { callee } => format!("{}()", callee),
        Expr::Add { left, right } => format!("{} + {}", format_expr(left), format_expr(right)),
    }
}
