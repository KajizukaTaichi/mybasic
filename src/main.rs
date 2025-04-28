mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;

use indexmap::IndexMap;
use ruka_vm::{RukaVM, asm};
use util::{OPERATOR, SPACE, include_letter};
use {expr::Expr, lexer::tokenize, oper::Oper, stmt::Stmt};

fn main() {
    let mut compiler = Compiler {
        if_label_index: 0,
        while_label_index: 0,
        variables: IndexMap::new(),
    };
    let code = include_str!("../example.bas").trim();
    compiler.run(code).unwrap();
}

struct Compiler {
    if_label_index: usize,
    while_label_index: usize,
    variables: IndexMap<String, usize>,
}

impl Compiler {
    fn run(&mut self, source: &str) -> Option<()> {
        let assembly = &self.build(source)?;
        println!("{}", &assembly);
        let bytecodes = asm(assembly).unwrap();
        let mut vm = RukaVM::new(bytecodes);
        vm.run()?;
        vm.dump();
        Some(())
    }

    fn build(&mut self, source: &str) -> Option<String> {
        let mut result = String::new();
        let source = source.trim().to_lowercase();
        for (line, code) in source.lines().enumerate() {
            let (line, code) = code
                .trim()
                .split_once(" ")
                .map(|(line, code)| Some((ok!(line.parse::<usize>())?, code)))
                .flatten()
                .unwrap_or((line, code));
            if code.is_empty() || code.trim().starts_with("rem") {
                continue;
            }
            let stmt = Stmt::parse(code)?.compile(self)?;
            result.push_str(&format!("line_{line}:\n{stmt}\n"));
        }
        Some(result)
    }
}
