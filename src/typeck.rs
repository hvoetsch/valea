use std::collections::HashMap;

use crate::{
    ast::{Expr, Program, Type},
    diagnostics::Diagnostic,
};

pub fn check(program: &Program) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut signatures: HashMap<&str, &Type> = HashMap::new();

    for function in &program.functions {
        if signatures
            .insert(&function.name, &function.return_type)
            .is_some()
        {
            diagnostics.push(Diagnostic::new(
                "E200",
                format!("Duplicate function '{}'", function.name),
                function.name_span.start,
                function.name_span.end,
            ));
        }
    }

    for function in &program.functions {
        let body_ty = infer_type(&function.body, &signatures, &mut diagnostics);
        if let Some(body_ty) = body_ty {
            if body_ty != function.return_type {
                diagnostics.push(Diagnostic::new(
                    "E201",
                    format!(
                        "Function '{}' returns {} but declared {}",
                        function.name,
                        type_display(&body_ty),
                        type_display(&function.return_type),
                    ),
                    function.name_span.start,
                    function.name_span.end,
                ));
            }
        }
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn type_display(ty: &Type) -> &'static str {
    match ty {
        Type::Int => "int",
        Type::Bool => "bool",
    }
}

fn infer_type(
    expr: &Expr,
    signatures: &HashMap<&str, &Type>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Type> {
    match expr {
        Expr::Integer(_) => Some(Type::Int),
        Expr::Bool(_) => Some(Type::Bool),
        Expr::Call { callee } => signatures
            .get(callee.as_str())
            .map(|t| (*t).clone())
            .or_else(|| {
                diagnostics.push(Diagnostic::new(
                    "E202",
                    format!("Unknown function '{}'", callee),
                    0,
                    0,
                ));
                None
            }),
        Expr::Add { left, right } => {
            let lt = infer_type(left, signatures, diagnostics);
            let rt = infer_type(right, signatures, diagnostics);
            match (lt, rt) {
                (Some(Type::Int), Some(Type::Int)) => Some(Type::Int),
                (Some(a), Some(b)) => {
                    diagnostics.push(Diagnostic::new(
                        "E203",
                        format!(
                            "'+' requires int operands, got {} and {}",
                            type_display(&a),
                            type_display(&b)
                        ),
                        0,
                        0,
                    ));
                    None
                }
                _ => None,
            }
        }
    }
}
