use crate::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    If(Expr),
    Else,
    EndIf,
    While(Expr),
    EndWhile,
    ExitWhile,
    Goto(String),
    Call(String),
    Sub(String),
    Return,
    ExitProgram,
}

impl Stmt {
    pub fn parse(source: &str) -> Option<Stmt> {
        let source = source.trim();
        if let Some(line) = source.strip_prefix("goto") {
            Some(Stmt::Goto(line.trim().to_string()))
        } else if let Some(name) = source.strip_prefix("sub") {
            Some(Stmt::Sub(name.trim().to_string()))
        } else if let Some(name) = source.strip_prefix("call") {
            Some(Stmt::Call(name.trim().to_string()))
        } else if let Some(code) = source.strip_prefix("if") {
            Some(Stmt::If(Expr::parse(code)?))
        } else if let Some(code) = source.strip_prefix("let") {
            let (name, code) = code.split_once("=")?;
            Some(Stmt::Let(name.trim().to_string(), Expr::parse(code)?))
        } else if source == "exit program" {
            Some(Stmt::ExitProgram)
        } else if source == "exit while" {
            Some(Stmt::ExitWhile)
        } else if let Some(name) = source.strip_prefix("while") {
            Some(Stmt::While(Expr::parse(name)?))
        } else if source == "end while" {
            Some(Stmt::EndWhile)
        } else if source == "end sub" || source == "exit sub" {
            Some(Stmt::Return)
        } else if source == "else" {
            Some(Stmt::Else)
        } else if source == "end if" {
            Some(Stmt::EndIf)
        } else {
            None
        }
    }

    pub fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Stmt::Let(name, expr) => {
                let addr = ctx.variables.get(name).cloned();
                let addr = addr.unwrap_or(ctx.variables.len());
                ctx.variables.insert(name.to_string(), addr);
                let expr = expr.compile(ctx)?;
                if expr.contains("\n") {
                    format!("{expr}\tsta {addr}, ar\n")
                } else {
                    format!("\tsta {addr}, {expr}\n")
                }
            }
            Stmt::If(expr) => {
                let expr = expr.compile(ctx)?;
                let result = format!(
                    "{expr}\tjmp cr, if_then_{label}\n\tjmp 1, if_else_{label}\n\tjmp 1, if_end_{label}\nif_then_{label}:\n",
                    expr = cond!(expr),
                    label = ctx.if_label_index
                );
                ctx.if_label_index += 1;
                result
            }
            Stmt::Else => {
                format!(
                    "\tjmp 1, if_end_{label}\nif_else_{label}:\n",
                    label = ctx.if_label_index - 1
                )
            }
            Stmt::EndIf => {
                ctx.if_label_index -= 1;
                format!("if_end_{}:\n", ctx.if_label_index)
            }
            Stmt::While(expr) => {
                let expr = expr.compile(ctx)?;
                let result = format!(
                    "while_start_{label}:\n{expr}\tnor cr, cr\n\tjmp cr, while_end_{label}\n",
                    expr = cond!(expr),
                    label = ctx.while_label_index
                );
                ctx.while_label_index += 1;
                result
            }
            Stmt::EndWhile => {
                format!(
                    "\tjmp 1, while_start_{label}\nwhile_end_{label}:\n",
                    label = ctx.while_label_index - 1
                )
            }
            Stmt::ExitWhile => {
                format!(
                    "\tjmp 1, while_end_{label}\n",
                    label = ctx.while_label_index - 1
                )
            }
            Stmt::Goto(line) => {
                format!("\tjmp 1, line_{line}\n")
            }
            Stmt::Sub(name) => {
                format!("subroutine_{name}:\n")
            }
            Stmt::Call(name) => format!("cal subroutine_{name}\n"),
            Stmt::Return => "\tret\n".to_owned(),
            Stmt::ExitProgram => "\thlt\n".to_owned(),
        })
    }
}
