use std::any::Any;

use crate::core::{LexerResult, PrimitiveType, TokenType};

pub trait Node {
    fn eval(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn debug(&self);
}

pub struct NodeMango {
    pub statement_suite: Box<Node>
}

impl Node for NodeMango {
    fn eval(&self) -> String { return self.statement_suite.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeMango"); }
}

pub struct NodeStatementSuite {
    pub statement_list: Box<Node>
}

impl Node for NodeStatementSuite {
    fn eval(&self) -> String { return self.statement_list.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementSuite"); }
}

pub struct NodeStatementSuiteFunction {
    pub statement_list_function: Box<Node>
}

impl Node for NodeStatementSuiteFunction {
    fn eval(&self) -> String { return self.statement_list_function.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementSuiteFunction"); }
}

pub struct NodeStatementSuiteClass {
    pub statement_list_class: Box<Node>
}

impl Node for NodeStatementSuiteClass {
    fn eval(&self) -> String { return self.statement_list_class.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementSuiteClass"); }
}

pub struct NodeStatementListRecursive {
    pub statement: Box<Node>,
    pub statement_list: Box<Node>,
}

impl Node for NodeStatementListRecursive {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementListRecursive"); }
}


pub struct NodeStatementList {
    pub statement: Box<Node>
}

impl Node for NodeStatementList {
    fn eval(&self) -> String { return self.statement.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementList"); }
}

pub struct NodeStatementListFunctionRecursive {
    pub statement_limited: Box<Node>,
    pub statement_list_function: Box<Node>,
}

impl Node for NodeStatementListFunctionRecursive {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementListFunctionRecursive"); }
}


pub struct NodeStatementListFunction {
    pub statement_limited: Box<Node>,
}

impl Node for NodeStatementListFunction {
    fn eval(&self) -> String { return self.statement_limited.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementListFunction"); }
}

pub struct NodeStatementListClassRecursive {
    pub statement_restricted: Box<Node>,
    pub statement_list_class: Box<Node>,
}

impl Node for NodeStatementListClassRecursive {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementListClassRecursive"); }
}

pub struct NodeStatementListClass {
    pub statement_restricted: Box<Node>,
}

impl Node for NodeStatementListClass {
    fn eval(&self) -> String { return self.statement_restricted.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementListClass"); }
}

pub struct NodeStatement {
    pub statement_x: Box<Node>
}

impl Node for NodeStatement {
    fn eval(&self) -> String { return self.statement_x.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatement"); }
}

pub struct NodeStatementLimited {
    pub statement_x: Box<Node>,
}

impl Node for NodeStatementLimited {
    fn eval(&self) -> String { return self.statement_x.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementLimited"); }
}

pub struct NodeStatementRestricted {
    pub statement_x: Box<Node>,
}

impl Node for NodeStatementRestricted {
    fn eval(&self) -> String { return self.statement_x.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementRestricted"); }
}

pub struct NodeStatementSimple {
    pub statement_x: Box<Node>
}

impl Node for NodeStatementSimple {
    fn eval(&self) -> String { return self.statement_x.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementSimple"); }
}

pub struct NodeStatementComplex {
    pub statement_x: Box<Node>
}

impl Node for NodeStatementComplex {
    fn eval(&self) -> String { return self.statement_x.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementComplex"); }
}

pub struct NodeStatementFunction {
    pub identifier: Box<Node>,
    pub function_params: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementFunction {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementFunction"); }
}

pub struct NodeFunctionParamsRecursive {
    pub function_params: Box<Node>,
    pub identifier: Box<Node>,
}

impl Node for NodeFunctionParamsRecursive {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeFunctionParamsRecursive"); }
}

pub struct NodeFunctionParams {
    pub identifier: Box<Node>,
}

impl Node for NodeFunctionParams {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeFunctionParams"); }
}

pub struct NodeStatementClass {
    pub identifier: Box<Node>,
    pub statement_suite_class: Box<Node>,
}

impl Node for NodeStatementClass {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementClass"); }
}

pub struct NodeStatementExpressionRecursive {
    pub statement_expression_2: Box<Node>,
    pub statement_expression_p: Box<Node>,
}

impl Node for NodeStatementExpressionRecursive {
    fn eval(&self) -> String {
        let statement_expression_p: &NodeStatementExpressionP = match self.statement_expression_p.as_any().downcast_ref::<NodeStatementExpressionP>() {
            Some(statement_expression_p) => statement_expression_p,
            None => panic!("Node Downcast Error: Node -> NodeStatementExpressionP"),
        };

        match statement_expression_p.operator {
            TokenType::Add => {
                (self.statement_expression_2.eval().parse::<i32>().unwrap() + self.statement_expression_p.eval().parse::<i32>().unwrap()).to_string()
            }
            TokenType::Subtract => {
                (self.statement_expression_2.eval().parse::<i32>().unwrap() - self.statement_expression_p.eval().parse::<i32>().unwrap()).to_string()
            }
            _ => { panic!("Operator Type Error") }
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpressionRecursive"); }
}

pub struct NodeStatementExpression {
    pub statement_expression_2: Box<Node>,
}

impl Node for NodeStatementExpression {
    fn eval(&self) -> String { return self.statement_expression_2.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression"); }
}

pub struct NodeStatementExpressionP {
    pub statement_expression: Box<Node>,
    pub operator: TokenType,
}

impl Node for NodeStatementExpressionP {
    fn eval(&self) -> String { return self.statement_expression.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpressionP"); }
}

pub struct NodeStatementExpression2Recursive {
    pub statement_expression_3: Box<Node>,
    pub statement_expression_2p: Box<Node>,
}

impl Node for NodeStatementExpression2Recursive {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression2Recursive"); }
}

pub struct NodeStatementExpression2 {
    pub statement_expression_3: Box<Node>,
}

impl Node for NodeStatementExpression2 {
    fn eval(&self) -> String { return self.statement_expression_3.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression2"); }
}

pub struct NodeStatementExpression2p {
    pub statement_expression_2: Box<Node>
}

impl Node for NodeStatementExpression2p {
    fn eval(&self) -> String { return self.statement_expression_2.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression2p"); }
}

pub struct NodeStatementExpression3Negation {
    pub statement_expression_3: Box<Node>
}

impl Node for NodeStatementExpression3Negation {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression3Negation"); }
}

pub struct NodeStatementExpression3Paren {
    pub statement_expression: Box<Node>
}

impl Node for NodeStatementExpression3Paren {
    fn eval(&self) -> String { return self.statement_expression.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression3Paren"); }
}

pub struct NodeStatementExpression3 {
    pub term: Box<Node>
}

impl Node for NodeStatementExpression3 {
    fn eval(&self) -> String { return self.term.eval(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementExpression3"); }
}

pub struct NodeStatementAssignment {
    pub identifier: Box<Node>,
    pub statement_expression: Box<Node>,
}

impl Node for NodeStatementAssignment {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementAssignment"); }
}

pub struct NodeStatementConditional {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementConditional {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementConditional"); }
}

pub struct NodeStatementConditionalW2 {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
    pub statement_conditional_2: Box<Node>,
}

impl Node for NodeStatementConditionalW2 {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementConditionalW2"); }
}

pub struct NodeStatementConditionalW3 {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
    pub statement_conditional_3: Box<Node>,
}

impl Node for NodeStatementConditionalW3 {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementConditionalW3"); }
}

pub struct NodeStatementConditional2Recursive {
    pub statement_conditional_2: Box<Node>,
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementConditional2Recursive {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementConditional2Recursive"); }
}

pub struct NodeStatementConditional2 {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
    pub statement_conditional_3: Box<Node>,
}

impl Node for NodeStatementConditional2 {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementConditional2"); }
}

pub struct NodeStatementConditional3 {
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementConditional3 {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementConditional3"); }
}

pub struct NodeConditionalExpression {
    pub term1: Box<Node>,
    pub comparison_operator: Box<Node>,
    pub term2: Box<Node>,
}

impl Node for NodeConditionalExpression {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeConditionalExpression"); }
}

pub struct NodeConditionalExpressionUnary {
    pub comparison_operator_unary: Box<Node>,
    pub term: Box<Node>,
}

impl Node for NodeConditionalExpressionUnary {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeConditionalExpressionUnary"); }
}

pub struct NodeComparisonOperator {
    pub operator: Box<Node>
}

impl Node for NodeComparisonOperator {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeComparisonOperator"); }
}

pub struct NodeComparisonOperatorUnary {
    pub operator: Box<Node>
}

impl Node for NodeComparisonOperatorUnary {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeComparisonOperatorUnary"); }
}

pub struct NodeStatementLoop {
    pub statement_loop: Box<Node>
}

impl Node for NodeStatementLoop {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementLoop"); }
}

pub struct NodeStatementLoopFor {
    pub identifier: Box<Node>,
    pub term: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementLoopFor {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementLoopFor"); }
}

pub struct NodeStatementLoopFor2 {
    pub identifier: Box<Node>,
    pub term1: Box<Node>,
    pub term2: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementLoopFor2 {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementLoopFor2"); }
}

pub struct NodeStatementLoopWhile {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementLoopWhile {
    fn eval(&self) -> String { return "".to_string(); }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeStatementLoopWhile"); }
}

pub struct NodeTerm {
    pub payload: LexerResult,
}

impl Node for NodeTerm {
    fn eval(&self) -> String {
        let token = &self.payload.token;
        return token.clone();
    }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeTerm"); }
}

pub struct NodeIdentifier {
    pub payload: LexerResult,
}

impl Node for NodeIdentifier {
    fn eval(&self) -> String {
        let token = &self.payload.token;
        return token.clone();
    }
    fn as_any(&self) -> &dyn Any { self }
    fn debug(&self) { println!("NodeIdentifier"); }
}

