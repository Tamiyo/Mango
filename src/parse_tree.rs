use crate::core::{LexerResult, PrimitiveType};

pub trait Node {
    fn eval(&self) -> String;
}

pub struct NodeMango{
}

impl node for NodeMango{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementSuite{
}

impl node for NodeStatementSuite{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementSuiteFunction{
}

impl node for NodeStatementSuiteFunction{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementSuiteClass{
}

impl node for NodeStatementSuiteClass{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementList{
}

impl node for NodeStatementList{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementListFunction{
}

impl node for NodeStatementListFunction{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementListClass{
}

impl node for NodeStatementListClass{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatement{
}

impl node for NodeStatement{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLimited{
}

impl node for NodeStatementLimited{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementRestricted{
}

impl node for NodeStatementRestricted{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementSimple{
}

impl node for NodeStatementSimple{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementComplex{
}

impl node for NodeStatementComplex{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementFunction{
}

impl node for NodeStatementFunction{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeFunctionParams{
}

impl node for NodeFunctionParams{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementClass{
}

impl node for NodeStatementClass{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpression{
}

impl node for NodeStatementExpression{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpressionP{
}

impl node for NodeStatementExpressionP{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpression2{
}

impl node for NodeStatementExpression2{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpression2p{
}

impl node for NodeStatementExpression2p{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementExpression3{
}

impl node for NodeStatementExpression3{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementAssignment{
}

impl node for NodeStatementAssignment{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional{
}

impl node for NodeStatementConditional{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional2{
}

impl node for NodeStatementConditional2{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditional3{
}

impl node for NodeStatementConditional3{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeConditionalExpression{
}

impl node for NodeConditionalExpression{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeComparisonOperator{
}

impl node for NodeComparisonOperator{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeComparisonOperatorUnary{
}

impl node for NodeComparisonOperatorUnary{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoop{
}

impl node for NodeStatementLoop{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoopFor{
}

impl node for NodeStatementLoopFor{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoopWhile{
}

impl node for NodeStatementLoopWhile{
	fn eval(&self) -> String { return "".to_string(); }
}

