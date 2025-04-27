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
    let code = "let a = 1\nif 0 then let a = 2\na * 10";
    let output = ctx.build(code).unwrap();
    println!("{output}");
    let bytecodes = asm(&output).unwrap();
    let mut vm = RukaVM::new(bytecodes);
    vm.run();
    vm.dump();
}

struct Compiler {
    label_index: usize,
    variables: IndexMap<String, usize>,
}
impl Compiler {
    fn build(&mut self, source: &str) -> Option<String> {
        let mut result = String::new();
        for (line, code) in source.lines().enumerate() {
            result.push_str(&format!(
                "line_{line}:\n{}\n",
                Stmt::parse(code)?.compile(self)?
            ));
        }
        Some(result)
    }
}
