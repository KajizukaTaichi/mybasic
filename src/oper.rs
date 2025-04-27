use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Mul(Expr, Expr),
}

impl Oper {
    pub fn parse(source: &str) -> Option<Self> {
        let token_list: Vec<String> = tokenize(source, SPACE.as_ref(), true)?;
        let token = Expr::parse(token_list.last()?)?;
        let operator = token_list.get(token_list.len().checked_sub(2)?)?;
        let has_lhs = |len: usize| Expr::parse(&join!(token_list.get(..token_list.len() - len)?));
        Some(match operator.as_str() {
            "+" => Oper::Add(has_lhs(2)?, token),
            "*" => Oper::Mul(has_lhs(2)?, token),
            _ => return None,
        })
    }

    pub fn compile(&self, ctx: &mut Compiler) -> String {
        let codegen = |lhs: &Expr, rhs: &Expr, opecode: &str, ctx: &mut Compiler| {
            let lhs = lhs.compile(ctx);
            let rhs = rhs.compile(ctx);
            if lhs.contains("\n") && rhs.contains("\n") {
                format!("{lhs}psh ar\n{rhs}mov dr, ar\npop ar\n{opecode} ar, dr\n")
            } else if lhs.contains("\n") {
                format!("{lhs}{opecode} ar, {rhs}\n")
            } else if rhs.contains("\n") {
                format!("{rhs}\nmov dr, ar\nmov ar, {lhs}\n{opecode} ar, dr\n")
            } else {
                format!("mov ar, {lhs}\n{opecode} ar, {rhs}\n")
            }
        };
        match self {
            Oper::Add(lhs, rhs) => codegen(lhs, rhs, "add", ctx),
            Oper::Mul(lhs, rhs) => codegen(lhs, rhs, "mul", ctx),
        }
    }
}
