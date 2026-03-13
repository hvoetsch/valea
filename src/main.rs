use std::{env, fs, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    match run(env::args().collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

fn run(args: Vec<String>) -> Result<(), ExitCode> {
    if args.len() < 3 {
        eprintln!("usage: valea <check|ast|fmt|emit-c> <path> [--json] [--check]");
        return Err(ExitCode::from(2));
    }

    let command = &args[1];
    let path = PathBuf::from(&args[2]);
    let json = args.iter().any(|a| a == "--json");
    let check_only = args.iter().any(|a| a == "--check");

    match command.as_str() {
        "check" => {
            let source = read_source(&path)?;
            match valea::check_source(&source) {
                Ok(_) => {
                    if json {
                        println!("[]");
                    } else {
                        println!("ok");
                    }
                    Ok(())
                }
                Err(diags) => {
                    if json {
                        println!("{}", valea::json::diagnostics_json(&diags));
                    } else {
                        for d in &diags {
                            println!("{}:{}", path.display(), d.render_human_with_source(&source));
                        }
                    }
                    Err(ExitCode::from(1))
                }
            }
        }
        "ast" => {
            if !json {
                eprintln!("ast requires --json");
                return Err(ExitCode::from(2));
            }
            let source = read_source(&path)?;
            match valea::parse_source(&source) {
                Ok(program) => {
                    println!("{}", valea::json::ast_json(&program));
                    Ok(())
                }
                Err(diags) => {
                    println!("{}", valea::json::diagnostics_json(&diags));
                    Err(ExitCode::from(1))
                }
            }
        }
        "fmt" => {
            let source = read_source(&path)?;
            match valea::parse_source(&source) {
                Ok(program) => {
                    let formatted = valea::formatter::format_program(&program);
                    if check_only {
                        if formatted == source {
                            println!("ok");
                            Ok(())
                        } else {
                            eprintln!("{}: not formatted", path.display());
                            Err(ExitCode::from(1))
                        }
                    } else {
                        fs::write(path, formatted).map_err(|_| ExitCode::from(2))?;
                        Ok(())
                    }
                }
                Err(diags) => {
                    for d in &diags {
                        println!("{}:{}", path.display(), d.render_human_with_source(&source));
                    }
                    Err(ExitCode::from(1))
                }
            }
        }
        "emit-c" => {
            let source = read_source(&path)?;
            match valea::check_source(&source) {
                Ok(program) => {
                    println!("{}", valea::codegen::emit_c(&program));
                    Ok(())
                }
                Err(diags) => {
                    if json {
                        println!("{}", valea::json::diagnostics_json(&diags));
                    } else {
                        for d in &diags {
                            println!("{}:{}", path.display(), d.render_human_with_source(&source));
                        }
                    }
                    Err(ExitCode::from(1))
                }
            }
        }
        _ => {
            eprintln!("unknown command: {}", command);
            Err(ExitCode::from(2))
        }
    }
}

fn read_source(path: &PathBuf) -> Result<String, ExitCode> {
    fs::read_to_string(path).map_err(|err| {
        eprintln!("failed to read {}: {}", path.display(), err);
        ExitCode::from(2)
    })
}
