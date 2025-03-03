mod expr;
mod lexer;
mod oper;
mod stmt;
mod util;
mod value;

use expr::Expr;
use lexer::{str_escape, tokenize};
use oper::Oper;
use util::{OPERATOR, SPACE, include_letter};
use value::Value;

fn main() {
    println!("Hello, world!");
}
