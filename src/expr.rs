use crate::stmt::Stmt;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr {
    I64Lit(i64),
    Ident(String),
    UnaryOp(Op, Box<Expr>),
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(String, Vec<Expr>),
    Error,
}

#[derive(Debug)]
pub struct FuncDef {
    pub name: String, 
    pub params: Vec<ParamDef>, 
    pub return_type: Option<TypeDef>,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct ParamDef {
    pub name: String, 
    pub type_name: TypeDef
}

#[derive(Debug)]
pub struct TypeDef {
    pub name: String
}