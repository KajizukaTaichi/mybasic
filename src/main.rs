mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;
mod value;

use expr::Expr;
use lexer::{str_escape, tokenize};
use oper::Oper;
use stmt::Stmt;
use util::{OPERATOR, SPACE, include_letter};
use value::Value;

use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
}

struct Compiler {
    variables: BTreeMap<String, Value>,
    precode: String,
}

fn compile_main(source: &str) -> Option<String> {
    let mut program = BTreeMap::new();
    for line in source.lines() {
        let line = line.trim();
        let (ln, code) = line.split_once(SPACE[0])?;
        let ln: usize = ln.trim().parse().unwrap();
        program.insert(ln, Stmt::parse(code.trim())?);
    }
    todo!()
}
