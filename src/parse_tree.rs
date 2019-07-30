use downcast_rs::Downcast;

use crate::core::{LexerResult, PrimitiveType, TokenType};

pub trait Node {
    fn eval(&self) -> String;
}

pub struct NodeMango {
    pub statement_suite: Box<Node>
}

impl Node for NodeMango {
    fn eval(&self) -> String { return self.statement_suite.eval(); }
}

pub struct NodeStatementSuite {
    pub statement_list: Box<Node>
}

impl Node for NodeStatementSuite {
    fn eval(&self) -> String { return self.statement_list.eval(); }
}

pub struct NodeStatementSuiteFunction {
    pub statement_list_function: Box<Node>
}

impl Node for NodeStatementSuiteFunction {
    fn eval(&self) -> String { return self.statement_list_function.eval(); }
}

pub struct NodeStatementSuiteClass {
    pub statement_list_class: Box<Node>
}

impl Node for NodeStatementSuiteClass {
    fn eval(&self) -> String { return self.statement_list_class.eval(); }
}

pub struct NodeStatementListRecursive {
    pub statement: Box<Node>,
    pub statement_list: Box<Node>,
}

impl Node for NodeStatementListRecursive {
    fn eval(&self) -> String { return "".to_string(); }
}


pub struct NodeStatementList {
    pub statement: Box<Node>
}

impl Node for NodeStatementList {
    fn eval(&self) -> String { return self.statement.eval(); }
}

pub struct NodeStatementListFunctionRecursive {
    pub statement_limited: Box<Node>,
    pub statement_list_function: Box<Node>,
}

impl Node for NodeStatementListFunctionRecursive {
    fn eval(&self) -> String { return "".to_string(); }
}


pub struct NodeStatementListFunction {
    pub statement_limited: Box<Node>,
}

impl Node for NodeStatementListFunction {
    fn eval(&self) -> String { return self.statement_limited.eval(); }
}

pub struct NodeStatementListClassRecursive {
    pub statement_restricted: Box<Node>,
    pub statement_list_class: Box<Node>,
}

impl Node for NodeStatementListClassRecursive {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementListClass {
    pub statement_restricted: Box<Node>,
}

impl Node for NodeStatementListClass {
    fn eval(&self) -> String { return self.statement_restricted.eval(); }
}

pub struct NodeStatement {
    pub statement_x: Box<Node>
}

impl Node for NodeStatement {
    fn eval(&self) -> String { return self.statement_x.eval(); }
}

pub struct NodeStatementLimited {
    pub statement_x: Box<Node>,
}

impl Node for NodeStatementLimited {
    fn eval(&self) -> String { return self.statement_x.eval(); }
}

pub struct NodeStatementRestricted {
    pub statement_x: Box<Node>,
}

impl Node for NodeStatementRestricted {
    fn eval(&self) -> String { return self.statement_x.eval(); }
}

pub struct NodeStatementSimple {
    pub statement_x: Box<Node>
}

impl Node for NodeStatementSimple {
    fn eval(&self) -> String { return self.statement_x.eval(); }
}

pub struct NodeStatementComplex {
    pub statement_x: Box<Node>
}

impl Node for NodeStatementComplex {
    fn eval(&self) -> String { return self.statement_x.eval(); }
}

pub struct NodeStatementFunction {
    pub identifier: Box<Node>,
    pub function_params: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementFunction {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeFunctionParamsRecursive {
    pub function_params: Box<Node>,
    pub identifier: Box<Node>,
}

impl Node for NodeFunctionParamsRecursive {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeFunctionParams {
    pub identifier: Box<Node>,
}

impl Node for NodeFunctionParams {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementClass {
    pub identifier: Box<Node>,
    pub statement_suite_class: Box<Node>,
}

impl Node for NodeStatementClass {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpressionRecursive {
    pub statement_expression_2: Box<Node>,
    pub statement_expression_p: Box<NodeStatementExpressionP>,
}

impl Node for NodeStatementExpressionRecursive {
    fn eval(&self) -> String {
        return "".to_string();
//        TODO
//        match self.statement_expression_p {
//            0 => {
//                return self.statement_expression_2.eval() + self.statement_expression_p.eval().as_str();
//            }
//            1 => {
//                return (self.statement_expression_2.eval().parse::<i32>().unwrap() - self.statement_expression_p.eval().parse::<i32>().unwrap()).to_string();
//            }
//            _ => {
//                return "".to_string();
//            }
//        }
    }
}

pub struct NodeStatementExpression {
    pub statement_expression_2: Box<Node>,
}

impl Node for NodeStatementExpression {
    fn eval(&self) -> String { return self.statement_expression_2.eval(); }
}

pub struct NodeStatementExpressionP {
    pub statement_expression: Box<Node>,
    pub operator: TokenType,
}

impl Node for NodeStatementExpressionP {
    fn eval(&self) -> String { return self.statement_expression.eval(); }
}

pub struct NodeStatementExpression2Recursive {
    pub statement_expression_3: Box<Node>,
    pub statement_expression_2p: Box<Node>,
}

impl Node for NodeStatementExpression2Recursive {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpression2 {
    pub statement_expression_3: Box<Node>,
}

impl Node for NodeStatementExpression2 {
    fn eval(&self) -> String { return self.statement_expression_3.eval(); }
}

pub struct NodeStatementExpression2p {
    pub statement_expression_2: Box<Node>
}

impl Node for NodeStatementExpression2p {
    fn eval(&self) -> String { return self.statement_expression_2.eval(); }
}

pub struct NodeStatementExpression3Negation {
    pub statement_expression_3: Box<Node>
}

impl Node for NodeStatementExpression3Negation {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpression3Paren {
    pub statement_expression: Box<Node>
}

impl Node for NodeStatementExpression3Paren {
    fn eval(&self) -> String { return self.statement_expression.eval(); }
}

pub struct NodeStatementExpression3 {
    pub term: Box<Node>
}

impl Node for NodeStatementExpression3 {
    fn eval(&self) -> String { return self.term.eval(); }
}

pub struct NodeStatementAssignment {
    pub identifier: Box<Node>,
    pub statement_expression: Box<Node>,
}

impl Node for NodeStatementAssignment {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementConditional {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditionalW2 {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
    pub statement_conditional_2: Box<Node>,
}

impl Node for NodeStatementConditionalW2 {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditionalW3 {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
    pub statement_conditional_3: Box<Node>,
}

impl Node for NodeStatementConditionalW3 {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional2Recursive {
    pub statement_conditional_2: Box<Node>,
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementConditional2Recursive {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional2 {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
    pub statement_conditional_3: Box<Node>,
}

impl Node for NodeStatementConditional2 {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional3 {
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementConditional3 {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeConditionalExpression {
    pub term1: Box<Node>,
    pub comparison_operator: Box<Node>,
    pub term2: Box<Node>,
}

impl Node for NodeConditionalExpression {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeConditionalExpressionUnary {
    pub comparison_operator_unary: Box<Node>,
    pub term: Box<Node>,
}

impl Node for NodeConditionalExpressionUnary {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeComparisonOperator {
    pub operator: Box<Node>
}

impl Node for NodeComparisonOperator {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeComparisonOperatorUnary {
    pub operator: Box<Node>
}

impl Node for NodeComparisonOperatorUnary {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoop {
    pub statement_loop: Box<Node>
}

impl Node for NodeStatementLoop {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoopFor {
    pub identifier: Box<Node>,
    pub term: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementLoopFor {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoopFor2 {
    pub identifier: Box<Node>,
    pub term1: Box<Node>,
    pub term2: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementLoopFor2 {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoopWhile {
    pub conditional_expression: Box<Node>,
    pub statement_suite_function: Box<Node>,
}

impl Node for NodeStatementLoopWhile {
    fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeTerm {
    pub payload: LexerResult,
}

impl Node for NodeTerm {
    fn eval(&self) -> String {
        let token = &self.payload.token;
        return token.clone();
    }
}

pub struct NodeIdentifier {
    pub payload: LexerResult,
}

impl Node for NodeIdentifier {
    fn eval(&self) -> String {
        let token = &self.payload.token;
        return token.clone();
    }
}

