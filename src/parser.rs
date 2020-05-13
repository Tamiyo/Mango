use crate::ast::Expr;
use crate::ast::Stmt;
use crate::distance::Distance;
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
            Symbol::LeftParen | Symbol::Dot => Precedence::Call,
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
            Err(ParseError::UnexpectedToken(token.clone()))
        } else {
            self.next()?;
            Ok(())
        }
    }

    fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.peek()?.symbol {
            Symbol::Class => self.parse_class(),
            Symbol::Fun => self.parse_function(),
            _ => self.parse_statement(),
        }
    }

    fn parse_method(&mut self) -> Result<Stmt, ParseError> {
        self.parse_function()
    }

    fn parse_class(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::Class)?;
        let next = self.next()?;
        let name = match next.symbol {
            Symbol::Identifier(name) => Ok(self.strings.get_or_intern(name)),
            _ => return Err(ParseError::ExpectedIdentifier(next.clone())),
        }?;

        self.consume(Symbol::LeftBrace)?;
        let mut methods: Vec<Stmt> = Vec::new();
        while self.peek()?.symbol != Symbol::RightBrace && self.peek()?.symbol != Symbol::Eof {
            let method = self.parse_method()?;
            methods.push(method);
        }
        self.consume(Symbol::RightBrace)?;
        Ok(Stmt::Class(name, methods))
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
        let next = self.next()?;
        let name = match next.symbol {
            Symbol::Identifier(name) => Ok(self.strings.get_or_intern(name)),
            _ => Err(ParseError::ExpectedIdentifier(self.peek()?.clone())),
        }?;

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
            Symbol::For => self.parse_for_statement(),
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
        let expr_list = self.parse_expression_list()?;
        self.consume(Symbol::RightParen)?;
        self.consume(Symbol::Semicolon)?;

        Ok(Stmt::Print(expr_list))
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

    fn parse_for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::For)?;

        let next = self.next()?;
        if let Symbol::Identifier(name) = next.symbol {
            self.consume(Symbol::In)?;
            let arr = self.parse_expression(Precedence::None)?;
            let len = Expr::Call(
                Box::new(Expr::Variable(self.strings.get_or_intern("len"))),
                vec![Expr::Variable(
                    self.strings.get_or_intern(format!("$r{}$", name)),
                )],
            );

            let body = self.parse_block_statement()?;

            let index_sym = self.strings.get_or_intern(format!("$i{}$", name));
            let index_assign = Stmt::Assign(index_sym, Box::new(Expr::Number(Distance::from(0.0))));

            let range_sym = self.strings.get_or_intern(format!("$r{}$", name));
            let range_assign = Stmt::Assign(range_sym, Box::new(arr.clone()));

            let element_sym = self.strings.get_or_intern(name);
            let element_index = Expr::Index(
                Box::new(Expr::Variable(range_sym)),
                Box::new(Expr::Pair(
                    Box::new(Expr::Variable(index_sym)),
                    Box::new(Expr::None),
                )),
            );
            let element_assign = Stmt::Assign(element_sym, Box::new(element_index));

            let condition = Expr::Binary(
                Box::new(Expr::Variable(index_sym)),
                Token::new(Symbol::Less, next.line, next.col),
                Box::new(len),
            );
            let adder = Stmt::Assign(
                index_sym,
                Box::new(Expr::Binary(
                    Box::new(Expr::Variable(index_sym)),
                    Token::new(Symbol::Plus, next.line, next.col),
                    Box::new(Expr::Number(Distance::from(1.0))),
                )),
            );

            let after_statements: Vec<Stmt> = vec![element_assign, body, adder];

            let combined: Vec<Stmt> = vec![
                index_assign,
                range_assign,
                Stmt::While(Box::new(condition), Box::new(Stmt::Block(after_statements))),
            ];

            Ok(Stmt::Block(combined))
        } else {
            return Err(ParseError::ExpectedIdentifier(next));
        }
    }

    fn parse_block_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Symbol::LeftBrace)?;

        let mut statements = Vec::new();
        while self.peek()?.symbol != Symbol::RightBrace && self.peek()?.symbol != Symbol::Eof {
            statements.push(self.parse_declaration()?);
        }

        self.consume(Symbol::RightBrace)?;
        Ok(Stmt::Block(statements))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expression(Precedence::None)?;
        self.consume(Symbol::Semicolon)?;
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
                Symbol::Dot => parser.parse_dot(left),

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
                Symbol::LeftSquare => parser.parse_multi_select_list(),
                Symbol::LeftParen => parser.parse_grouping(),
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
        if self.peek()?.symbol != Symbol::RightParen {
            expressions.push(self.parse_expression(Precedence::None)?);
        }

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

    fn parse_range_expression(&mut self, start: Expr) -> Result<Expr, ParseError> {
        let start: Box<Expr> = Box::new(start);
        let mut step: Option<Box<Expr>> = None;
        self.consume(Symbol::Colon)?;
        let stop = Box::new(self.parse_expression(Precedence::None)?);
        if self.peek()?.symbol == Symbol::Colon {
            self.consume(Symbol::Colon)?;
            step = Some(Box::new(self.parse_expression(Precedence::None)?))
        }
        Ok(Expr::Range(start, stop, step))
    }

    fn parse_call(&mut self, left: Expr) -> Result<Expr, ParseError> {
        self.consume(Symbol::LeftParen)?;
        let args = self.parse_expression_list()?;
        self.consume(Symbol::RightParen)?;
        Ok(Expr::Call(Box::new(left), args))
    }

    fn parse_dot(&mut self, left: Expr) -> Result<Expr, ParseError> {
        self.consume(Symbol::Dot)?;
        let next = self.next()?;
        let sym = match next.symbol {
            Symbol::Identifier(s) => self.strings.get_or_intern(s),
            _ => return Err(ParseError::ExpectedIdentifier(next.clone())),
        };

        match self.peek()?.symbol {
            Symbol::Equal => {
                self.consume(Symbol::Equal)?;
                let right = self.parse_expression(Precedence::None)?;
                Ok(Expr::Set(Box::new(left), sym, Box::new(right)))
            }
            _ => Ok(Expr::Get(Box::new(left), sym)),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.next()?;
        match token.symbol {
            Symbol::Number(n) => match self.peek()?.symbol {
                Symbol::Colon => self.parse_range_expression(Expr::Number(n)),
                _ => Ok(Expr::Number(n)),
            },
            Symbol::String(s) => Ok(Expr::String(self.strings.get_or_intern(s))),
            Symbol::True => Ok(Expr::Boolean(true)),
            Symbol::False => Ok(Expr::Boolean(false)),
            Symbol::Identifier(s) => Ok(Expr::Variable(self.strings.get_or_intern(s))),
            Symbol::LeftSquare => self.parse_multi_select_list(),
            _ => Err(ParseError::ExpectedLiteral(token.clone())),
        }
    }
}
