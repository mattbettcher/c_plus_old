use crate::expr::*;

#[derive(Debug)]
pub enum Stmt {
    // Name of variable, optional type - if none then it must be inferred, optional init expression
    DecVar(String, Option<TypeDef>, Option<Expr>),
    Call(String, Vec<Expr>),    // need to add return type
    Assign(String, Expr),
    Error,
}