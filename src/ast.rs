use crate::distance::Distance;
use crate::tokens::Token;
use string_interner::Sym;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(Distance),
    String(Sym),
    Boolean(bool),
    None,
    Variable(Sym),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Pair(Box<Expr>, Box<Expr>),
    Range(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Slice(Option<Box<Expr>>, Option<Box<Expr>>, Option<Box<Expr>>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Get(Box<Expr>, Sym),
    Set(Box<Expr>, Sym, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Vec<Expr>),
    Return(Option<Box<Expr>>),
    Assign(Sym, Box<Expr>),
    Block(Vec<Stmt>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Function(Sym, Vec<Sym>, Vec<Stmt>),
    Class(Sym, Vec<Stmt>),
}
