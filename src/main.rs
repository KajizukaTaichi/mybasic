mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;
mod value;

use indexmap::IndexMap;
use ruka_vm::{RukaVM, asm};
use util::{OPERATOR, SPACE, include_letter};
use {expr::Expr, lexer::tokenize, oper::Oper, stmt::Stmt, value::Value};

fn main() {
    let mut ctx = Compiler {
        label_index: 0,
        variables: IndexMap::new(),
    };
    let output = ctx.build(include_str!("../example.bas")).unwrap();
    println!("{output}");
    let bytecodes = asm(&output).unwrap();
    let mut vm = RukaVM::new(bytecodes);
    vm.run();
    dbg!(vm);
}

struct Compiler {
    label_index: usize,
    variables: IndexMap<String, usize>,
}
impl Compiler {
    fn build(&mut self, source: &str) -> Option<String> {
        let mut result = String::new();
        for code in source.lines() {
            let (line, code) = code.split_once(" ")?;
            let stmt = Stmt::parse(code)?.compile(self)?;
            result.push_str(&format!("line_{line}:\n{stmt}\n",));
        }
        Some(result)
    }
}
