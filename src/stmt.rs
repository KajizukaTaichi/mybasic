use crate::*;

#[derive(Debug, Clone)]
enum Stmt {
    Print(Expr),
    Let(String, Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Goto(Expr),
    Call(Expr),
    Return,
    End,
}

impl Stmt {
    fn parse(source: &str) -> Option<Stmt> {
        let source = source.trim();
        if let Some(code) = source.strip_prefix("print") {
            Some(Stmt::Print(Expr::parse(code)?))
        } else if let Some(code) = source.strip_prefix("goto") {
            Some(Stmt::Goto(Expr::parse(code)?))
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
            return None;
        }
    }
}
