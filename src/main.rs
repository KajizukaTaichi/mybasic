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
    variables: Vec<String>,
}

fn compile_main(source: &str) -> Option<String> {
    let program = {
        let mut program = BTreeMap::new();
        for line in source.lines() {
            let line = line.trim();
            let (ln, code) = line.split_once(SPACE[0])?;
            let ln: usize = ln.trim().parse().unwrap();
            program.insert(ln, Stmt::parse(code.trim())?);
        }
        program
    };

    let mut result = String::new();
    let mut ctx = Compiler { variables: vec![] };
    for (line, code) in program {
        result.push_str(&format!("line_{line}:\n{}\n\n", code.compile(&mut ctx)));
    }
    todo!()
}
