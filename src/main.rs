mod ast;
mod asm;
mod irgen;
mod eval;
use crate::irgen::IR;
use crate::asm::GenerateAsm;
use koopa::front::Driver;

use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::fs::write;
use std::io::Result;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    let mut args=args();
    args.next();
    let mode=args.next().unwrap();
    let input=args.next().unwrap();
    args.next();
    let output=args.next().unwrap();

    let input=read_to_string(input)?;
    let ast=sysy::CompUnitParser::new().parse(&input).unwrap();

    match &mode as &str {
        "-ast" => println!("{:#?}",ast),
        "-koopa" => write(&output, ast.irdump().irstr)?,
        "-riscv" => write(&output, Driver::from(ast.irdump().irstr).generate_program().unwrap().generate(None))?,
        _ => {}
    }

    //println!("{}",ast.irdump().irstr);

    Ok(())
}