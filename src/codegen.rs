use crate::ast::{Expr, Program, Type};

pub fn emit_c(program: &Program) -> String {
    let mut out = String::from("#include <stdbool.h>\n\n");
    for function in &program.functions {
        out.push_str(&format!(
            "{} {}(void) {{ return {}; }}\n",
            c_type(&function.return_type),
            function.name,
            emit_expr(&function.body)
        ));
    }
    out
}

fn c_type(ty: &Type) -> &'static str {
    match ty {
        Type::Int => "long",
        Type::Bool => "bool",
    }
}

fn emit_expr(expr: &Expr) -> String {
    match expr {
        Expr::Integer(v) => v.to_string(),
        Expr::Bool(v) => {
            if *v {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Expr::Call { callee } => format!("{}()", callee),
        Expr::Add { left, right } => format!("({} + {})", emit_expr(left), emit_expr(right)),
    }
}
