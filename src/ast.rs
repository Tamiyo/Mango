use crate::token::Symbol;

pub type Identifier = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Symbol, Box<Expr>),
    Logical(Box<Expr>, Symbol, Box<Expr>),
    Grouping(Box<Expr>),
    Number(f64),
    Boolean(bool),
    None,
    String(String),
    Unary(Symbol, Box<Expr>),
    Variable(Identifier),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Block(Vec<Stmt>),
    Assign(Identifier, Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Function(Identifier, Vec<Identifier>, Vec<Stmt>),
    Return(Option<Box<Expr>>),
}