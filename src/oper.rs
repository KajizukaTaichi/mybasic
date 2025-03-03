use crate::*;

#[derive(Debug, Clone)]
pub enum Oper {
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Mod(Expr, Expr),
    Pow(Expr, Expr),
    Equal(Expr, Expr),
    NotEq(Expr, Expr),
    Less(Expr, Expr),
    LessEq(Expr, Expr),
    Greater(Expr, Expr),
    GreaterEq(Expr, Expr),
    And(Expr, Expr),
    Or(Expr, Expr),
    Not(Expr),
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
            "/" => Oper::Div(has_lhs(2)?, token),
            "%" => Oper::Mod(has_lhs(2)?, token),
            "^" => Oper::Pow(has_lhs(2)?, token),
            "==" => Oper::Equal(has_lhs(2)?, token),
            "!=" => Oper::NotEq(has_lhs(2)?, token),
            "<" => Oper::Less(has_lhs(2)?, token),
            ">" => Oper::Greater(has_lhs(2)?, token),
            "<=" => Oper::LessEq(has_lhs(2)?, token),
            ">=" => Oper::GreaterEq(has_lhs(2)?, token),
            "&&" => Oper::And(has_lhs(2)?, token),
            "||" => Oper::Or(has_lhs(2)?, token),
            "-" => {
                if let Some(lhs) = has_lhs(2) {
                    Oper::Sub(lhs, token)
                } else if token_list.len() == 2 {
                    Oper::Sub(Expr::Value(Value::Num(0.0)), token)
                } else {
                    Oper::parse(&format!(
                        "{} (0 - {})",
                        &join!(token_list.get(..token_list.len() - 2)?),
                        token_list.last()?
                    ))?
                }
            }
            "!" => {
                if token_list.len() == 2 {
                    Oper::Not(token)
                } else {
                    Oper::parse(&format!(
                        "{} (!{})",
                        join!(token_list.get(..token_list.len() - 2)?),
                        token_list.last()?
                    ))?
                }
            }
            _ => return None,
        })
    }
}
