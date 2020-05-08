use crate::ast::Expr;
use crate::ast::Stmt;
use crate::error::ParseError;
use crate::scanner::Scanner;
use crate::tokens::Symbol;
use crate::tokens::Token;
use string_interner::StringInterner;
use string_interner::Sym;
#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Power,
    Unary,
    Index,
    Call,
}

impl From<&Token> for Precedence {
    fn from(token: &Token) -> Precedence {
        match token.symbol {
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
            Symbol::LeftSquare => Precedence::Index,
            Symbol::LeftParen => Precedence::Call,
            Symbol::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pub strings: StringInterner<Sym>,
}

impl Parser {
    pub fn new(buf: &str) -> Self {
        let mut sc = Scanner::new(buf);
        let mut tokens = sc.tokenize();
        tokens.reverse();

        Parser {
            tokens,
            strings: StringInterner::default(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];
        while self.peek()?.symbol != Symbol::Eof {
            statements.push(self.parse_declaration()?);
        }
        Ok(statements)
    }

    fn peek(&self) -> Result<&Token, ParseError> {
        match self.tokens.last() {
            Some(token) => Ok(token),
            None => Err(ParseError::TokenStreamEmpty),
        }
    }

    fn peek_n(&self, n: usize) -> Result<&Token, ParseError> {
        match self.tokens.get(self.tokens.len() - n - 1) {
            Some(token) => Ok(token),
            None => Err(ParseError::TokenStreamEmpty),
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        match self.tokens.pop() {
            Some(token) => Ok(token),
            None => Err(ParseError::TokenStreamEmpty),
        }
    }

    fn consume(&mut self, expected: Symbol) -> Result<(), ParseError> {
        let token = self.peek()?;
        if token.symbol != expected {
            println!("{:?}", expected);
            Err(ParseError::UnexpectedToken(token.clone()))
        } else {
            self.next()?;
            Ok(())
        }
    }

    fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.peek()?.symbol {
            // Symbol::Struct => {}
            Symbol::Fun => self.parse_function(),
            _ => self.parse_statement(),
        }
    }

    fn parse_identifier_list(&mut self) -> Result<Vec<Sym>, ParseError> {
        let mut identifiers = Vec::new();
        let mut next = self.next()?;
        identifiers.push(match next.symbol {
            Symbol::Identifier(name) => Ok(self.strings.get_or_intern(name)),
            _ => return Err(ParseError::ExpectedIdentifier(next.clone())),
        }?);

        while self.peek()?.symbol == Symbol::Comma {
            self.consume(Symbol::Comma)?;
            next = self.next()?;
            identifiers.push(match next.symbol {
                Symbol::Identifier(name) => Ok(self.strings.get_or_intern(name)),
                _ => return Err(ParseError::ExpectedIdentifier(next.clone())),
            }?);
        }
        Ok(identifiers)
    }

    fn parse_function(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Fun)?;
        let name = match self.peek()?.symbol.clone() {
            Symbol::Identifier(name) => Ok(self.strings.get_or_intern(name)),
            _ => Err(ParseError::ExpectedIdentifier(self.peek()?.clone())),
        }?;
        self.next()?;

        self.consume(Symbol::LeftParen)?;
        let params: Vec<Sym> = if self.peek()?.symbol != Symbol::RightParen {
            self.parse_identifier_list()?
        } else {
            Vec::new()
        };
        self.consume(Symbol::RightParen)?;

        self.consume(Symbol::LeftBrace)?;
        let mut statements: Vec<Stmt> = Vec::new();
        while self.peek()?.symbol != Symbol::RightBrace && self.peek()?.symbol != Symbol::Eof {
            statements.push(self.parse_declaration()?);
        }
        self.consume(Symbol::RightBrace)?;

        Ok(Stmt::Function(name, params, statements))
    }

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek()?.symbol {
            Symbol::Var => self.parse_assign_statement(),
            Symbol::Print => self.parse_print_statement(),
            Symbol::Return => self.parse_return_statement(),
            Symbol::If => self.parse_if_statement(),
            Symbol::While => self.parse_while_statement(),
            // Symbol::For => {}
            Symbol::LeftBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_assign_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Var)?;
        let name = match self.peek()?.symbol.clone() {
            Symbol::Identifier(name) => Ok(name),
            _ => Err(ParseError::ExpectedIdentifier(self.peek()?.clone())),
        }?;

        self.consume(Symbol::Identifier(name.clone()))?;
        self.consume(Symbol::Equal)?;

        let expr = self.parse_expression(Precedence::None)?;
        self.consume(Symbol::Semicolon)?;

        Ok(Stmt::Assign(
            self.strings.get_or_intern(name),
            Box::new(expr),
        ))
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Print)?;
        self.consume(Symbol::LeftParen)?;
        let expr = self.parse_expression(Precedence::None)?;
        self.consume(Symbol::RightParen)?;
        self.consume(Symbol::Semicolon)?;

        Ok(Stmt::Print(Box::new(expr)))
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Return)?;
        let expr = if self.peek()?.symbol != Symbol::Semicolon {
            Some(Box::new(self.parse_expression(Precedence::None)?))
        } else {
            None
        };
        self.consume(Symbol::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::If)?;
        self.consume(Symbol::LeftParen)?;
        let if_condition = self.parse_expression(Precedence::None)?;
        self.consume(Symbol::RightParen)?;

        let if_block = self.parse_block_statement()?;
        let rest = self.parse_elif_statement()?;

        Ok(Stmt::If(Box::new(if_condition), Box::new(if_block), rest))
    }

    fn parse_elif_statement(&mut self) -> Result<Option<Box<Stmt>>, ParseError> {
        if self.peek()?.symbol == Symbol::Elif {
            self.consume(Symbol::Elif)?;
            self.consume(Symbol::LeftParen)?;
            let elif_condition = self.parse_expression(Precedence::None)?;
            self.consume(Symbol::RightParen)?;
            let elif_block = self.parse_block_statement()?;
            Ok(Some(Box::new(Stmt::If(
                Box::new(elif_condition),
                Box::new(elif_block),
                self.parse_elif_statement()?,
            ))))
        } else {
            self.parse_else_statement()
        }
    }

    fn parse_else_statement(&mut self) -> Result<Option<Box<Stmt>>, ParseError> {
        let else_block = if self.peek()?.symbol == Symbol::Else {
            self.consume(Symbol::Else)?;
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Ok(else_block.map(Box::new))
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::While)?;
        self.consume(Symbol::LeftParen)?;
        let while_condition = self.parse_expression(Precedence::None)?;
        self.consume(Symbol::RightParen)?;

        let while_block = self.parse_block_statement()?;

        Ok(Stmt::While(
            Box::new(while_condition),
            Box::new(while_block),
        ))
    }

    fn parse_block_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::LeftBrace)?;

        let mut statements = Vec::new();
        while self.peek()?.symbol != Symbol::RightBrace && self.peek()?.symbol != Symbol::Eof {
            statements.push(self.parse_statement()?);
        }

        self.consume(Symbol::RightBrace)?;
        Ok(Stmt::Block(statements))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = match self.peek()?.symbol {
            // Symbol::LeftSquare => self.parse_multi_select_list(),
            _ => self.parse_expression(Precedence::None),
        }?;
        Ok(Stmt::Expression(Box::new(expr)))
    }

    fn parse_multi_select_list(&mut self) -> Result<Expr, ParseError> {
        self.consume(Symbol::LeftSquare)?;
        let expressions = if self.peek()?.symbol != Symbol::RightSquare {
            self.parse_expression_list()?
        } else {
            vec![]
        };
        self.consume(Symbol::RightSquare)?;
        Ok(Expr::List(expressions))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, ParseError> {
        fn infix(parser: &mut Parser, left: Expr) -> Result<Expr, ParseError> {
            match parser.peek()?.symbol {
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
                | Symbol::Carat => parser.parse_binary(left),

                Symbol::And | Symbol::Or => parser.parse_logical(left),
                Symbol::LeftSquare => parser.parse_index(left),
                Symbol::LeftParen => parser.parse_call(left),

                _ => Err(ParseError::UnexpectedOperator(parser.peek()?.clone())),
            }
        }
        fn prefix(parser: &mut Parser) -> Result<Expr, ParseError> {
            match parser.peek()?.symbol {
                Symbol::Number(_)
                | Symbol::None
                | Symbol::My
                | Symbol::String(_)
                | Symbol::True
                | Symbol::False
                | Symbol::Identifier(_) => parser.parse_primary(),
                Symbol::Not | Symbol::Minus => parser.parse_unary(),
                Symbol::LeftParen => parser.parse_grouping(),
                Symbol::LeftSquare => parser.parse_multi_select_list(),
                _ => Err(ParseError::UnexpectedOperator(parser.peek()?.clone())),
            }
        }
        let mut expr = prefix(self)?;
        while let Ok(token) = self.peek() {
            let next = Precedence::from(token);
            if precedence >= next {
                break;
            }
            expr = infix(self, expr)?;
        }
        Ok(expr)
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expressions = Vec::new();
        expressions.push(self.parse_expression(Precedence::None)?);

        while self.peek()?.symbol == Symbol::Comma {
            self.consume(Symbol::Comma)?;
            expressions.push(self.parse_expression(Precedence::None)?);
        }
        Ok(expressions)
    }

    fn parse_binary(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let precedence = Precedence::from(self.peek()?);
        let op = self.next()?;
        match op.symbol {
            Symbol::Plus
            | Symbol::Minus
            | Symbol::Star
            | Symbol::Slash
            | Symbol::Modulo
            | Symbol::Carat
            | Symbol::Less
            | Symbol::LessEqual
            | Symbol::Greater
            | Symbol::GreaterEqual
            | Symbol::NotEqual
            | Symbol::EqualEqual => Ok(()),
            _ => Err(ParseError::UnexpectedToken(op.clone())),
        }?;
        let right = self.parse_expression(precedence)?;

        // Do Constant Folding Here...

        Ok(Expr::Binary(Box::new(left), op, Box::new(right)))
    }

    fn parse_logical(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let precedence = Precedence::from(self.peek()?);
        let op = self.next()?;
        match op.symbol {
            Symbol::And | Symbol::Or => Ok(()),
            _ => Err(ParseError::UnexpectedToken(op.clone())),
        }?;
        let right = self.parse_expression(precedence)?;

        Ok(Expr::Logical(Box::new(left), op, Box::new(right)))
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        let op = self.next()?;
        match op.symbol {
            Symbol::Not | Symbol::Minus => Ok(()),
            _ => Err(ParseError::UnexpectedToken(op.clone())),
        }?;
        let right = self.parse_expression(Precedence::None)?;
        Ok(Expr::Unary(op, Box::new(right)))
    }

    fn parse_grouping(&mut self) -> Result<Expr, ParseError> {
        self.consume(Symbol::LeftParen)?;
        let expr = self.parse_expression(Precedence::None)?;
        self.consume(Symbol::RightParen)?;
        Ok(Expr::Grouping(Box::new(expr)))
    }

    fn parse_index(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let right = self.parse_bracket_specifier()?;
        Ok(Expr::Index(Box::new(left), Box::new(right)))
    }

    fn parse_bracket_specifier(&mut self) -> Result<Expr, ParseError> {
        let mut slice = Expr::None;

        while self.peek()?.symbol == Symbol::LeftSquare {
            self.consume(Symbol::LeftSquare)?;
            slice = Expr::Pair(Box::new(slice), Box::new(self.parse_slice_expression()?));
            self.consume(Symbol::RightSquare)?;
        }

        Ok(slice)
    }

    fn parse_slice_expression(&mut self) -> Result<Expr, ParseError> {
        let mut start: Option<Box<Expr>> = None;
        let mut stop: Option<Box<Expr>> = None;
        let mut step: Option<Box<Expr>> = None;

        if self.peek()?.symbol != Symbol::Colon {
            start = Some(Box::new(self.parse_expression(Precedence::None)?));
        }

        if self.peek()?.symbol == Symbol::Colon && self.peek_n(1)?.symbol == Symbol::Colon {
            self.consume(Symbol::Colon)?;
            self.consume(Symbol::Colon)?;
            step = Some(Box::new(self.parse_expression(Precedence::None)?));
        } else if self.peek()?.symbol == Symbol::Colon {
            self.consume(Symbol::Colon)?;
            if self.peek()?.symbol != Symbol::RightSquare {
                stop = Some(Box::new(self.parse_expression(Precedence::None)?));
                if self.peek()?.symbol == Symbol::Colon {
                    self.consume(Symbol::Colon)?;
                    step = Some(Box::new(self.parse_expression(Precedence::None)?));
                }
            }
        } else {
            return Ok(*start.unwrap());
        }

        Ok(Expr::Slice(start, stop, step))
    }

    fn parse_call(&mut self, left: Expr) -> Result<Expr, ParseError> {
        self.consume(Symbol::LeftParen)?;
        let args = self.parse_expression_list()?;
        self.consume(Symbol::RightParen)?;
        Ok(Expr::Call(Box::new(left), args))
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.next()?;
        match token.symbol {
            Symbol::Number(n) => Ok(Expr::Number(n)),
            Symbol::String(s) => Ok(Expr::String(self.strings.get_or_intern(s))),
            Symbol::True => Ok(Expr::Boolean(true)),
            Symbol::False => Ok(Expr::Boolean(false)),
            Symbol::Identifier(s) => Ok(Expr::Variable(self.strings.get_or_intern(s))),
            Symbol::LeftSquare => self.parse_multi_select_list(),
            _ => Err(ParseError::ExpectedLiteral(token.clone())),
        }
    }
}
