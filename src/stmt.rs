use crate::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Goto(String),
    Call(String),
    Sub(String),
    Expr(Expr),
    Return,
    End,
}

impl Stmt {
    pub fn parse(source: &str) -> Option<Stmt> {
        let source = source.trim();
        if let Some(code) = source.strip_prefix("GOTO") {
            Some(Stmt::Goto(code.to_string()))
        } else if let Some(code) = source.strip_prefix("SUB") {
            Some(Stmt::Sub(code.to_string()))
        } else if let Some(code) = source.strip_prefix("CALL") {
            Some(Stmt::Call(code.to_string()))
        } else if let Some(code) = source.strip_prefix("IF") {
            let (cond, body) = code.split_once("THEN")?;
            if let Some((then, r#else)) = body.split_once("ELSE") {
                Some(Stmt::If(
                    Expr::parse(cond)?,
                    Box::new(Stmt::parse(then)?),
                    Some(Box::new(Stmt::parse(r#else)?)),
                ))
            } else {
                Some(Stmt::If(
                    Expr::parse(cond)?,
                    Box::new(Stmt::parse(body)?),
                    None,
                ))
            }
        } else if let Some(code) = source.strip_prefix("LET") {
            let (name, code) = code.split_once("=")?;
            Some(Stmt::Let(name.trim().to_string(), Expr::parse(code)?))
        } else if source == "END" {
            Some(Stmt::End)
        } else if source == "RETURN" {
            Some(Stmt::Return)
        } else {
            Some(Stmt::Expr(Expr::parse(source)?))
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
                    format!("\t{expr}sta {addr}, ar\n")
                } else {
                    format!("\tsta {addr}, {expr}\n")
                }
            }
            Stmt::If(expr, then, None) => {
                let expr = expr.compile(ctx)?;
                let then = then.compile(ctx)?;
                let result = format!(
                    "{expr}\tjmp cr, if_then_{label}\n\tjmp 1, if_end_{label}\nif_then_{label}:\n{then}if_end_{label}:\n",
                    expr = if expr.contains("\n") {
                        format!("{expr}\tmov cr, ar\n")
                    } else {
                        format!("\tmov cr, {expr}\n")
                    },
                    label = ctx.label_index
                );
                ctx.label_index += 1;
                result
            }
            Stmt::Goto(line) => {
                format!("\tjmp 1, {line}\n")
            }
            Stmt::Expr(expr) => expr.compile(ctx)?,
            _ => return None,
        })
    }
}
