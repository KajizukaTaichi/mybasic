use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Refer(String),
    Oper(Box<Oper>),
}

impl Expr {
    pub fn parse(source: &str) -> Option<Expr> {
        let source = source.trim();
        let token_list: Vec<String> = tokenize(source.trim(), SPACE.as_ref(), true)?;
        if token_list.len() >= 2 {
            Some(Expr::Oper(Box::new(Oper::parse(source)?)))
        } else {
            let token = token_list.last()?.trim().to_string();
            Some(if let Ok(n) = token.parse::<f64>() {
                Expr::Value(Value::Num(n))
            // prioritize higher than others
            } else if token.starts_with("(") && token.ends_with(")") {
                let token = token.get(1..token.len() - 1)?.trim();
                Expr::parse(token)?
            // Variable reference
            } else {
                Expr::Refer(token)
            })
        }
    }

    pub fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        Some(match self {
            Expr::Oper(oper) => oper.compile(ctx)?,
            Expr::Value(Value::Num(n)) => n.to_string(),
            Expr::Refer(to) => format!("\tlda ar, {}\n", ctx.variables.get(to)?),
        })
    }
}
