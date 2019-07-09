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

pub struct NodeStatementList{
}

impl node for NodeStatementList{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatement{
}

impl node for NodeStatement{
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

pub struct NodeStatementConditionalElif{
}

impl node for NodeStatementConditionalElif{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditionalElse{
}

impl node for NodeStatementConditionalElse{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementConditionalTest{
}

impl node for NodeStatementConditionalTest{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeOperatorBinary{
}

impl node for NodeOperatorBinary{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeOperatorUnary{
}

impl node for NodeOperatorUnary{
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

pub struct NodeStatementLoopForOptions{
}

impl node for NodeStatementLoopForOptions{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementLoopWhile{
}

impl node for NodeStatementLoopWhile{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementDefineFunction{
}

impl node for NodeStatementDefineFunction{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeFunctionParams{
}

impl node for NodeFunctionParams{
	fn eval(&self) -> String { return "".to_string(); }
}

pub struct NodeStatementDefineClass{
}

impl node for NodeStatementDefineClass{
	fn eval(&self) -> String { return "".to_string(); }
}

