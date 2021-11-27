#![allow(unused)]
#[macro_use] extern crate lalrpop_util;
mod expr;
mod stmt;

use expr::*;
use stmt::*;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    use parser::FuncDefParser as ep;
    let mut errors = Vec::new();

    let i = ep::new().parse(&mut errors, "fn main(a:u32) -> i32 { a = -someFunc(-10 + -10); otherFunc(a, b, 10); var a: i32; var b = hello(); }").unwrap();
    println!("{:}", &format!("{:#?}", i));
    println!("{:#?}", errors);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stmt_test() {
        use parser::FuncDefParser as fp;
        let mut errors = Vec::new();

        assert!(fp::new().parse(&mut errors, "fn main(a:u32) { var a }").is_ok());
        assert!(fp::new().parse(&mut errors, "fn main(a:u32) -> i32 { var a \n var b }").is_ok());
    }

    #[test]
    fn func_def_test() {
        use parser::FuncDefParser as fp;
        let mut errors = Vec::new();

        assert!(fp::new().parse(&mut errors, "fn main(a:u32) {}").is_ok());
        assert!(fp::new().parse(&mut errors, "fn main(a:u32) -> i32 {}").is_ok());
        assert!(fp::new().parse(&mut errors, "fn main(a:u32, b:someTypeName) -> i32 {}").is_ok());
        assert!(fp::new().parse(&mut errors, "fn main(a:u32, b:someTypeName) {}").is_ok());
    }

    #[test]
    fn call_test() {
        use parser::CallParser as fp;
        let mut errors = Vec::new();
        assert!(fp::new().parse(&mut errors, "main(1 + 1)").is_ok());
        assert!(fp::new().parse(&mut errors, "main(1 + 1, 2 - 1)").is_ok());
        assert!(fp::new().parse(&mut errors, "main(1 + 1, sin(cos(11)))").is_ok());
    }

    #[test]
    fn expr_test() {
        use parser::ExprParser as ep;
        let mut errors = Vec::new();

        assert!(ep::new().parse(&mut errors, "5 + -10").is_ok());
        assert!(ep::new().parse(&mut errors, "10 + 10").is_ok());
        assert!(ep::new().parse(&mut errors, "10 + 10 * 8").is_ok());
    }

    #[test]
    fn whole_num_test() {
        use parser::NumParser as ip;
        let mut errors = Vec::new();

        assert!(ip::new().parse(&mut errors, "10").is_ok());
        assert!(ip::new().parse(&mut errors, "12345").is_ok());
    }

    #[test]
    fn ident_test() {
        use parser::IdentParser as ip;
        let mut errors = Vec::new();

        assert!(ip::new().parse(&mut errors, "main").is_ok());
        assert!(ip::new().parse(&mut errors, "_").is_ok());
        assert!(ip::new().parse(&mut errors, "_0").is_ok());
        assert!(ip::new().parse(&mut errors, "abYZ").is_ok());
        assert!(ip::new().parse(&mut errors, "a_0").is_ok());
    }

}