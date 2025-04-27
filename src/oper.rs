use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Mul(Expr, Expr),
    Eql(Expr, Expr),
    Les(Expr, Expr),
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
            "=" => Oper::Eql(has_lhs(2)?, token),
            "<" => Oper::Les(has_lhs(2)?, token),
            ">" => Oper::Les(token, has_lhs(2)?),
            _ => return None,
        })
    }

    pub fn compile(&self, ctx: &mut Compiler) -> Option<String> {
        let codegen = |lhs: &Expr, rhs: &Expr, opecode: &str, ctx: &mut Compiler| {
            let lhs = lhs.compile(ctx)?;
            let rhs = rhs.compile(ctx)?;
            Some(if lhs.contains("\n") && rhs.contains("\n") {
                format!("{lhs}\tpsh ar\n{rhs}\tmov dr, ar\n\tpop ar\n\t{opecode} ar, dr\n")
            } else if lhs.contains("\n") {
                format!("{lhs}\t{opecode} ar, {rhs}\n")
            } else if rhs.contains("\n") {
                format!("{rhs}\tmov dr, ar\n\tmov ar, {lhs}\n\t{opecode} ar, dr\n")
            } else {
                format!("\tmov ar, {lhs}\n\t{opecode} ar, {rhs}\n")
            })
        };
        Some(match self {
            Oper::Add(lhs, rhs) => codegen(lhs, rhs, "add", ctx)?,
            Oper::Mul(lhs, rhs) => codegen(lhs, rhs, "mul", ctx)?,
            Oper::Eql(lhs, rhs) => codegen(lhs, rhs, "eql", ctx)?,
            Oper::Les(lhs, rhs) => codegen(lhs, rhs, "les", ctx)?,
        })
    }
}
