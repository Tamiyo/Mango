use crate::memory::Distance;
use crate::tokens::Token;
use string_interner::Sym;
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(Distance),
    String(Sym),
    Boolean(bool),
    None,
    Variable(Sym),
    List(Vec<Expr>),
    Index(Sym, Box<Expr>),
    Slice(Option<Box<Expr>>, Option<Box<Expr>>, Option<Box<Expr>>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Unary(Token, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Return(Option<Box<Expr>>),
    Assign(Sym, Box<Expr>),
    Block(Vec<Stmt>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
<<<<<<< HEAD
    Function(Sym, Vec<Sym>, Vec<Stmt>),
=======
    Function(Sym, Vec<Sym>, Box<Stmt>),
>>>>>>> 382353fd91b0585622e95c4ebfd4e877abef4353
}