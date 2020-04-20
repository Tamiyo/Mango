use crate::scanner::Scanner;
use crate::error::ParseError;
use crate::ast::{Expr, Stmt};
use crate::token::{Symbol, Token};
use crate::parser::Precedence::Primary;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Precedence {
    None,
    // Assign,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Power,
    Unary,
    Call,
    Primary,
}

impl<'a> From<&'a Symbol> for Precedence {
    fn from(symbol: &Symbol) -> Precedence {
        match *symbol {
            // Symbol::Equal => Precedence::Assign,
            Symbol::Or => Precedence::Or,
            Symbol::And => Precedence::And,
            Symbol::NotEqual | Symbol::EqualEqual => Precedence::Equality,
            Symbol::Less | Symbol::LessEqual | Symbol::Greater | Symbol::GreaterEqual => {
                Precedence::Comparison
            }
            Symbol::Plus | Symbol::Minus => Precedence::Term,
            Symbol::Star | Symbol::Slash | Symbol::Modulo => Precedence::Factor,
            Symbol::Carat => Precedence::Power,
            Symbol::Not => Precedence::Unary,
            Symbol::LeftParen => Precedence::Call,
            Symbol::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(buf: &str) -> Self {
        let mut sc = Scanner::new(buf);
        let mut tokens = sc.tokenize();
        tokens.reverse();

        Parser {
            tokens,
        }
    }

    // helper
    fn check(&self) -> Option<&Symbol> {
        match self.tokens.last() {
            Some(t) => Some(&t.value),
            None => None
        }
    }

    fn peek(&self) -> Result<&Symbol, String> {
        match self.tokens.last() {
            Some(token) => Ok(&token.value),
            None => Err(String::from("peek: No more tokens"))
        }
    }

    fn next(&mut self) -> Result<Symbol, String> {
        match self.tokens.pop() {
            Some(token) => Ok(token.value),
            None => Err(String::from("next: No more tokens"))
        }
    }

    fn consume(&mut self, expected: Symbol, message: &str) -> Result<(), ParseError> {
        let token = self.peek().expect("expected peek symbol");
        if token != &expected {
            Err(ParseError::from(message))
        } else {
            self.next().expect("expected next symbol");
            Ok(())
        }
    }

    // statement
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts: Vec<Stmt> = vec![];
        while self.check().unwrap_or(&Symbol::Eof) != &Symbol::Eof {
            stmts.push(self.declaration()?);
        }
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        self.statement()
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.check() {
            Some(Symbol::Identifier(name)) => self.assign_statement(name.clone()),
            Some(Symbol::Print) => self.print_statement(),
            Some(Symbol::If) => self.if_statement(),
            Some(Symbol::LeftBrace) => self.block_statement(),
            Some(Symbol::While) => self.while_statement(),
            Some(Symbol::For) => self.for_statement(),
            _ => self.expression_statement()
        }
    }

    fn assign_statement(&mut self, name: String) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Identifier(name.to_string()), "Expect some identifier in assign")?;
        self.consume(Symbol::Equal, "Expect '=' after identifier")?;
        let expr = self.expression(Precedence::None)?;
        self.consume(Symbol::Semicolon, "Expect ';' after assign.")?;
        Ok(Stmt::Assign(name.clone(), Box::new(expr)))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Print, "Expect print keyword.")?;
        self.consume(Symbol::LeftParen, "Expect '(' after print.")?;
        let expr = self.expression(Precedence::None)?;
        self.consume(Symbol::RightParen, "Expect ') after expression.")?;
        self.consume(Symbol::Semicolon, "Expect ';' after print.")?;

        Ok(Stmt::Print(Box::new(expr)))
    }

    fn else_statement(&mut self) -> Result<Option<Box<Stmt>>, ParseError> {
        let mut else_block: Option<Stmt> = None;
        if self.check().unwrap() == &Symbol::Else {
            self.consume(Symbol::Else, "Expect else keyword")?;
            else_block = Some(self.block_statement()?);
        }
        Ok(else_block.map(Box::new))
    }

    fn elif_statement(&mut self) -> Result<Option<Box<Stmt>>, ParseError> {
        if self.check().unwrap() == &Symbol::Elif {
            self.consume(Symbol::Elif, "Expect elif keyword")?;
            self.consume(Symbol::LeftParen, "Expect '(' after elif.")?;
            let elif_condition = self.expression(Precedence::None)?;
            self.consume(Symbol::RightParen, "Expect ')' after condition.")?;
            let elif_block = self.block_statement()?;
            Ok(Some(Box::new(Stmt::If(Box::new(elif_condition), Box::new(elif_block), self.elif_statement()?))))
        } else {
            self.else_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::If, "Expect if keyword.")?;
        self.consume(Symbol::LeftParen, "Expect '(' after if.")?;
        let if_condition = self.expression(Precedence::None)?;
        self.consume(Symbol::RightParen, "Expect ')' after condition.")?;

        let if_block = self.block_statement()?;

        let rest = self.elif_statement()?;

        Ok(Stmt::If(Box::new(if_condition), Box::new(if_block), rest))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::While, "Expect while keyword.")?;
        self.consume(Symbol::LeftParen, "Expect '(' after while.")?;
        let while_condition = self.expression(Precedence::None)?;
        self.consume(Symbol::RightParen, "Expect ')' after condition.")?;

        let while_block = self.block_statement()?;

        Ok(Stmt::While(Box::new(while_condition), Box::new(while_block)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::For, "Expect for keyword.")?;

        let identifier = match self.next()? {
            Symbol::Identifier(name) => name,
            _ => panic!("Expected identifier in for loop.")
        };

        self.consume(Symbol::Equal, "Expect '=' after identifier")?;
        let start = self.expression(Precedence::None)?;
        let initializer = Stmt::Assign(String::from(identifier.clone()), Box::new(start.clone()));

        self.consume(Symbol::Colon, "Expect ':' after ident assign.")?;
        let limit = self.expression(Precedence::None)?;

        let condition = Expr::Binary(Box::new(Expr::Variable(identifier.clone())), Symbol::Less, Box::new(limit));

        let mut increment: Option<Expr> = None;
        if self.peek()? == &Symbol::Colon {
            self.consume(Symbol::Colon, "Expect ':' before increment.")?;
            increment = Some(self.expression(Precedence::None)?);
        }

        let increment_expression = match increment {
            Some(expr) => Expr::Binary(Box::new(Expr::Variable(identifier.clone())), Symbol::Plus, Box::new(expr)),
            _ => Expr::Binary(Box::new(Expr::Variable(identifier.clone())), Symbol::Plus, Box::new(Expr::Number(1 as f64))),
        };

        let body = self.block_statement()?;
        let body = Stmt::Block(vec![body, Stmt::Expression(Box::new(increment_expression))]);
        let body = Stmt::While(Box::new(condition), Box::new(body));
        let body = Stmt::Block(vec![initializer, body]);

        Ok(body)
    }

    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::LeftBrace, "Expect '{' before block")?;

        let mut statements: Vec<Stmt> = Vec::new();
        while self.peek()? != &Symbol::RightBrace && self.peek()? != &Symbol::Eof {
            statements.push(self.declaration()?);
        }
        self.consume(Symbol::RightBrace, "Expect '}' after block")?;
        Ok(Stmt::Block(statements))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression(Precedence::None)?;
        self.consume(Symbol::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expression(Box::new(expr)))
    }

    // expression
    fn infix(&mut self, left: Expr) -> Result<Expr, ParseError> {
        match self.peek()? {
            Symbol::NotEqual
            | Symbol::EqualEqual
            | Symbol::Less
            | Symbol::LessEqual
            | Symbol::Greater
            | Symbol::GreaterEqual
            | Symbol::Plus
            | Symbol::Minus
            | Symbol::Star
            | Symbol::Slash
            | Symbol::Modulo
            | Symbol::Carat => self.binary(left),

            Symbol::And
            | Symbol::Or => self.logical(left),

            // Symbol::Equal => self.assign(left),

            _ => panic!(format!("unexpected infix operator {:?}", self.peek()))
        }
    }

    fn prefix(&mut self) -> Result<Expr, ParseError> {
        match self.peek()? {
            Symbol::Number(_)
            | Symbol::None
            | Symbol::My
            | Symbol::String(_)
            | Symbol::True
            | Symbol::False
            | Symbol::Identifier(_) => self.primary(),

            Symbol::Not => self.unary(),

            Symbol::LeftParen => self.grouping(),
            _ => panic!(format!("invalid prefix token: {:?}", self.peek()?))
        }
    }

    fn expression(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
        let mut expr = self.prefix()?;
        while let Some(token) = self.check() {
            let next = Precedence::from(token);
            if precedence >= next {
                break;
            }
            expr = self.infix(expr)?;
        }
        Ok(expr)
    }

    fn binary(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let precedence = Precedence::from(self.peek()?);
        let op = match self.next()? {
            Symbol::Plus => Symbol::Plus,
            Symbol::Minus => Symbol::Minus,
            Symbol::Star => Symbol::Star,
            Symbol::Slash => Symbol::Slash,
            Symbol::Modulo => Symbol::Modulo,
            Symbol::Carat => Symbol::Carat,
            Symbol::Less => Symbol::Less,
            Symbol::LessEqual => Symbol::LessEqual,
            Symbol::Greater => Symbol::Greater,
            Symbol::GreaterEqual => Symbol::GreaterEqual,
            Symbol::NotEqual => Symbol::NotEqual,
            Symbol::EqualEqual => Symbol::EqualEqual,
            _ => panic!("Expected binary op.")
        };
        let right = self.expression(precedence)?;
        Ok(Expr::Binary(Box::new(left), op, Box::new(right)))
    }

    fn logical(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let precedence = Precedence::from(self.peek()?);
        let op = match self.next()? {
            Symbol::And => Symbol::And,
            Symbol::Or => Symbol::Or,
            _ => panic!("Expected logical binary op.")
        };
        let right = self.expression(precedence)?;
        Ok(Expr::Logical(Box::new(left), op, Box::new(right)))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.next()?;
        match token {
            Symbol::None => Ok(Expr::None),
            Symbol::Number(n) => Ok(Expr::Number(n)),
            Symbol::True => Ok(Expr::Boolean(true)),
            Symbol::False => Ok(Expr::Boolean(false)),
            Symbol::String(s) => Ok(Expr::String(s)),
            Symbol::Identifier(s) => Ok(Expr::Variable(s)),
            _ => panic!("Expected primary!")
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        let op = match self.next()? {
            Symbol::Not => Symbol::Not,
            _ => panic!("Expected unary op.")
        };
        let right = self.expression(Precedence::Unary)?;
        Ok(Expr::Unary(op, Box::new(right)))
    }

    fn grouping(&mut self) -> Result<Expr, ParseError> {
        self.consume(Symbol::LeftParen, "Expect '(' at beginning of expression.")?;
        let expr = self.expression(Precedence::None)?;
        self.consume(Symbol::RightParen, "Expect ')' after expression.")?;
        Ok(Expr::Grouping(Box::new(expr)))
    }
}

