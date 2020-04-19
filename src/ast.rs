use crate::token::Symbol;

pub type Identifier = String;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Symbol, Box<Expr>),
    Grouping(Box<Expr>),
    Number(f64),
    Boolean(bool),
    None,
    String(String),
    Unary(Symbol, Box<Expr>),
    Variable(Identifier),
    // Assign(Identifier, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Block(Vec<Stmt>),
    Assign(Identifier, Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
}