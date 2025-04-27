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
        variables: IndexMap::new(),
    };
    let code = "let x = 10 + 2 * 2\nlet y = 1 + x\nx + (y + x)";
    let output = ctx.build(code).unwrap();
    println!("{output}");
    let bytecodes = asm(&output).unwrap();
    let mut vm = RukaVM::new(bytecodes);
    vm.run();
    vm.dump();
}

struct Compiler {
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
