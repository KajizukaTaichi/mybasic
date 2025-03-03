use crate::*;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
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
        if let Some(code) = source.strip_prefix("print") {
            Some(Stmt::Print(Expr::parse(code)?))
        } else if let Some(code) = source.strip_prefix("goto") {
            Some(Stmt::Goto(code.to_string()))
        } else if let Some(code) = source.strip_prefix("sub") {
            Some(Stmt::Sub(code.to_string()))
        } else if let Some(code) = source.strip_prefix("call") {
            Some(Stmt::Call(code.to_string()))
        } else if let Some(code) = source.strip_prefix("if") {
            let (cond, body) = code.split_once("then")?;
            if let Some((then, r#else)) = body.split_once("else") {
                Some(Stmt::If(
                    Expr::parse(cond)?,
                    Box::new(Stmt::parse(then)?),
                    Some(Box::new(Stmt::parse(r#else)?)),
                ))
            } else {
                Some(Stmt::If(
                    Expr::parse(cond)?,
                    Box::new(Stmt::parse(code)?),
                    None,
                ))
            }
        } else if let Some(code) = source.strip_prefix("let") {
            let (name, code) = code.split_once("=")?;
            Some(Stmt::Let(name.trim().to_string(), Expr::parse(code)?))
        } else if source == "end" {
            Some(Stmt::End)
        } else if source == "return" {
            Some(Stmt::Return)
        } else {
            Some(Stmt::Expr(Expr::parse(source)?))
        }
    }

    pub fn compile(&self, ctx: &mut Compiler) -> String {
        match self {
            Stmt::Let(name, expr) => {
                ctx.variables.push(name.to_string());
                let expr = expr.compile(ctx);
                if expr.contains("\n") {
                    format!("{expr}mov dword [{name}], eax\n")
                } else {
                    format!("mov byte [{name}], {expr}\n")
                }
            }
            Stmt::Expr(expr) => expr.compile(ctx),
            _ => todo!(),
        }
    }
}
