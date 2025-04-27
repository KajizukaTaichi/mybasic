mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;
mod value;

use expr::Expr;
use lexer::tokenize;
use oper::Oper;
use ruka_vm::{RukaVM, asm};
use stmt::Stmt;
use util::{OPERATOR, SPACE, include_letter};
use value::Value;

fn main() {
    let mut ctx = Compiler {};
    let output = &Expr::parse("1 + 2").unwrap().compile(&mut ctx);
    dbg!(output);
    let bytecodes = asm(output).unwrap();
    let mut vm = RukaVM::new(bytecodes);
    vm.run();
    vm.dump();
}

struct Compiler {}
impl Compiler {
    fn build(source: &str) -> Option<String> {
        let mut result = String::new();
        let mut ctx = Compiler {};
        for (line, code) in source.lines().enumerate() {
            result.push_str(&format!(
                "line_{line}:\n{}\n\n",
                Stmt::parse(code)?.compile(&mut ctx)
            ));
        }
        Some(result)
    }
}
