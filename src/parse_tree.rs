use crate::core::{LexerResult, PrimitiveType};

pub trait Node {
    fn eval(&self) -> String;
}

pub struct NodeMango {
    pub statement_suite: Box<Node>
}

impl Node for NodeMango {
    fn eval(&self) -> String {
        return self.statement_suite.eval();
    }
}

pub struct NodeStatementSuite {
    pub statement_list: Box<Node>
}

impl Node for NodeStatementSuite {
    fn eval(&self) -> String {
        return self.statement_list.eval();
    }
}

pub struct NodeStatementList {
    pub statement: Box<Node>
}

impl Node for NodeStatementList {
    fn eval(&self) -> String {
        return self.statement.eval();
    }
}

pub struct NodeStatement {
    pub statement_simple: Box<Node>
}

impl Node for NodeStatement {
    fn eval(&self) -> String {
        return self.statement_simple.eval();
    }
}

pub struct NodeStatementSimple {
    pub statement_assignment: Box<Node>
}

impl Node for NodeStatementSimple {
    fn eval(&self) -> String {
        return self.statement_assignment.eval();
    }
}

pub struct NodeStatementAssignment {
    pub identifier: Box<Node>,
    pub term: Box<Node>,
}

impl Node for NodeStatementAssignment {
    fn eval(&self) -> String {
        return format!("{} = {}", self.identifier.eval(), self.term.eval());
    }
}

pub struct NodeIdentifier {
    pub value: Box<LexerResult>
}

impl Node for NodeIdentifier {
    fn eval(&self) -> String {
        return self.value.token.clone();
    }
}

pub struct NodeTerm {
    pub value: Box<LexerResult>,
}

impl Node for NodeTerm {
    fn eval(&self) -> String {
        return self.value.token.clone();
    }
}