use core::borrow::Borrow;
use std::collections::HashMap;
use std::ptr::null;
use std::slice::Iter;
use std::vec::Vec;
use crate::core::{ActionNode, GotoNode, LexerResult, ParserAction, PrimitiveType, TokenType};
use crate::core::TokenType::{StatementSuite, Term};
use crate::parse_tree::{Node, NodeIdentifier, NodeMango, NodeStatement, NodeStatementAssignment, NodeStatementList, NodeStatementSimple, NodeStatementSuite, NodeTerm};

pub struct Parser { pub token_stack: Vec<LexerResult>, pub action: HashMap<(i32, TokenType), ActionNode>, pub goto: HashMap<i32, GotoNode> }

impl Default for Parser { fn default() -> Parser { Parser { token_stack: Vec::new(), action: HashMap::new(), goto: HashMap::new() } } }

impl Parser {
    fn initialize(&mut self) {
		self.goto.insert(1, GotoNode{token_type: TokenType::StatementSuite, value: 2});
		self.goto.insert(2, GotoNode{token_type: TokenType::StatementList, value: 3});
		self.goto.insert(3, GotoNode{token_type: TokenType::StatementList, value: 1});
		self.goto.insert(4, GotoNode{token_type: TokenType::Statement, value: 1});
		self.goto.insert(5, GotoNode{token_type: TokenType::Statement, value: 1});
		self.goto.insert(6, GotoNode{token_type: TokenType::StatementSimple, value: 1});
		self.goto.insert(7, GotoNode{token_type: TokenType::StatementSimple, value: 1});
		self.goto.insert(8, GotoNode{token_type: TokenType::StatementComplex, value: 1});
		self.goto.insert(9, GotoNode{token_type: TokenType::StatementComplex, value: 1});
		self.goto.insert(10, GotoNode{token_type: TokenType::StatementComplex, value: 1});
		self.goto.insert(11, GotoNode{token_type: TokenType::StatementComplex, value: 1});
		self.goto.insert(12, GotoNode{token_type: TokenType::StatementExpression, value: 2});
		self.goto.insert(13, GotoNode{token_type: TokenType::StatementExpression, value: 1});
		self.goto.insert(14, GotoNode{token_type: TokenType::StatementExpressionP, value: 2});
		self.goto.insert(15, GotoNode{token_type: TokenType::StatementExpressionP, value: 2});
		self.goto.insert(16, GotoNode{token_type: TokenType::StatementExpression2, value: 2});
		self.goto.insert(17, GotoNode{token_type: TokenType::StatementExpression2, value: 1});
		self.goto.insert(18, GotoNode{token_type: TokenType::StatementExpression2p, value: 2});
		self.goto.insert(19, GotoNode{token_type: TokenType::StatementExpression2p, value: 2});
		self.goto.insert(20, GotoNode{token_type: TokenType::StatementExpression3, value: 1});
		self.goto.insert(21, GotoNode{token_type: TokenType::StatementExpression3, value: 3});
		self.goto.insert(22, GotoNode{token_type: TokenType::StatementExpression3, value: 2});
		self.goto.insert(23, GotoNode{token_type: TokenType::StatementAssignment, value: 3});
		self.goto.insert(24, GotoNode{token_type: TokenType::StatementConditional, value: 4});
		self.goto.insert(25, GotoNode{token_type: TokenType::StatementConditional, value: 5});
		self.goto.insert(26, GotoNode{token_type: TokenType::StatementConditionalElif, value: 4});
		self.goto.insert(27, GotoNode{token_type: TokenType::StatementConditionalElif, value: 5});
		self.goto.insert(28, GotoNode{token_type: TokenType::StatementConditionalElif, value: 5});
		self.goto.insert(29, GotoNode{token_type: TokenType::StatementConditionalElse, value: 4});
		self.goto.insert(30, GotoNode{token_type: TokenType::StatementConditional, value: 5});
		self.goto.insert(31, GotoNode{token_type: TokenType::StatementConditional, value: 6});
		self.goto.insert(32, GotoNode{token_type: TokenType::StatementConditionalElif, value: 5});
		self.goto.insert(33, GotoNode{token_type: TokenType::StatementConditionalElif, value: 6});
		self.goto.insert(34, GotoNode{token_type: TokenType::StatementConditionalElif, value: 6});
		self.goto.insert(35, GotoNode{token_type: TokenType::StatementConditionalElse, value: 5});
		self.goto.insert(36, GotoNode{token_type: TokenType::StatementConditionalTest, value: 3});
		self.goto.insert(37, GotoNode{token_type: TokenType::StatementConditionalTest, value: 2});
		self.goto.insert(38, GotoNode{token_type: TokenType::StatementConditionalTest, value: 1});
		self.goto.insert(39, GotoNode{token_type: TokenType::OperatorBinary, value: 1});
		self.goto.insert(40, GotoNode{token_type: TokenType::OperatorBinary, value: 1});
		self.goto.insert(41, GotoNode{token_type: TokenType::OperatorBinary, value: 1});
		self.goto.insert(42, GotoNode{token_type: TokenType::OperatorBinary, value: 1});
		self.goto.insert(43, GotoNode{token_type: TokenType::OperatorBinary, value: 1});
		self.goto.insert(44, GotoNode{token_type: TokenType::OperatorBinary, value: 1});
		self.goto.insert(45, GotoNode{token_type: TokenType::OperatorUnary, value: 1});
		self.goto.insert(46, GotoNode{token_type: TokenType::StatementLoop, value: 1});
		self.goto.insert(47, GotoNode{token_type: TokenType::StatementLoop, value: 1});
		self.goto.insert(48, GotoNode{token_type: TokenType::StatementLoopFor, value: 6});
		self.goto.insert(49, GotoNode{token_type: TokenType::StatementLoopForOptions, value: 1});
		self.goto.insert(50, GotoNode{token_type: TokenType::StatementLoopForOptions, value: 3});
		self.goto.insert(51, GotoNode{token_type: TokenType::StatementLoopForOptions, value: 5});
		self.goto.insert(52, GotoNode{token_type: TokenType::StatementLoopWhile, value: 5});
		self.goto.insert(53, GotoNode{token_type: TokenType::StatementDefineFunction, value: 7});
		self.goto.insert(54, GotoNode{token_type: TokenType::FunctionParams, value: 3});
		self.goto.insert(55, GotoNode{token_type: TokenType::FunctionParams, value: 2});
		self.goto.insert(56, GotoNode{token_type: TokenType::FunctionParams, value: 1});
		self.goto.insert(57, GotoNode{token_type: TokenType::StatementDefineClass, value: 5});
		self.action.insert((0, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 1});
		self.action.insert((0, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 2});
		self.action.insert((0, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((0, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((0, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((0, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((0, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((0, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((0, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((0, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((0, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((0, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((0, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((0, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((0, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((0, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((0, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((0, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((0, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((0, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((0, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((0, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((0, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((1, TokenType::EndOfFile), ActionNode{action: ParserAction::Accept, value: -1});
		self.action.insert((2, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 24});
		self.action.insert((3, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 3});
		self.action.insert((4, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 4});
		self.action.insert((5, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 5});
		self.action.insert((6, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 6});
		self.action.insert((7, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 7});
		self.action.insert((8, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 8});
		self.action.insert((9, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 9});
		self.action.insert((10, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((11, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((12, TokenType::StatementExpressionP), ActionNode{action: ParserAction::Goto, value: 26});
		self.action.insert((12, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((12, TokenType::Add), ActionNode{action: ParserAction::Shift, value: 27});
		self.action.insert((12, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 28});
		self.action.insert((13, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 29});
		self.action.insert((13, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((13, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((13, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((14, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 30});
		self.action.insert((14, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((14, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((14, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((14, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 31});
		self.action.insert((14, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 32});
		self.action.insert((15, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((15, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((15, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((15, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((15, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((16, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 33});
		self.action.insert((16, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 34});
		self.action.insert((16, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 36});
		self.action.insert((16, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((16, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((16, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((17, TokenType::Equals), ActionNode{action: ParserAction::Shift, value: 39});
		self.action.insert((18, TokenType::StatementConditionalTest), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((18, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((18, TokenType::OperatorUnary), ActionNode{action: ParserAction::Goto, value: 42});
		self.action.insert((18, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((19, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 46});
		self.action.insert((20, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 47});
		self.action.insert((21, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((22, TokenType::StatementConditionalTest), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((22, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 45});
		self.action.insert((22, TokenType::OperatorUnary), ActionNode{action: ParserAction::Goto, value: 47});
		self.action.insert((22, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((23, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 48});
		self.action.insert((24, TokenType::EndOfFile), ActionNode{action: ParserAction::Reduce, value: 1});
		self.action.insert((25, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 49});
		self.action.insert((25, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((25, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((25, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((25, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((25, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((25, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((25, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((25, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((25, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((25, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((25, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((25, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((25, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((25, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((25, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((25, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((25, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((25, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((25, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((25, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((25, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((26, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((27, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 50});
		self.action.insert((27, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((27, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((27, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((27, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((27, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((28, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 51});
		self.action.insert((28, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((28, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((28, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((28, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((28, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((29, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((29, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((29, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((29, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((29, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((30, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((30, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((30, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((31, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 52});
		self.action.insert((31, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 53});
		self.action.insert((31, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((31, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((31, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((32, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 54});
		self.action.insert((32, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 53});
		self.action.insert((32, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((32, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((32, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((33, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 55});
		self.action.insert((34, TokenType::StatementExpressionP), ActionNode{action: ParserAction::Goto, value: 56});
		self.action.insert((34, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((34, TokenType::Add), ActionNode{action: ParserAction::Shift, value: 57});
		self.action.insert((34, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 58});
		self.action.insert((35, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 59});
		self.action.insert((35, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((35, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((35, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((36, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 60});
		self.action.insert((36, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((36, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((36, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((36, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 61});
		self.action.insert((36, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 62});
		self.action.insert((37, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((37, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((37, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((37, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((37, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((38, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 63});
		self.action.insert((38, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 34});
		self.action.insert((38, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 36});
		self.action.insert((38, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((38, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((38, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((39, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 64});
		self.action.insert((39, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((39, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((39, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((39, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((39, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((40, TokenType::OperatorBinary), ActionNode{action: ParserAction::Goto, value: 65});
		self.action.insert((40, TokenType::Colon), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((40, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((40, TokenType::LessThan), ActionNode{action: ParserAction::Shift, value: 66});
		self.action.insert((40, TokenType::LessThanEquals), ActionNode{action: ParserAction::Shift, value: 67});
		self.action.insert((40, TokenType::GreaterThan), ActionNode{action: ParserAction::Shift, value: 68});
		self.action.insert((40, TokenType::GreaterThanEquals), ActionNode{action: ParserAction::Shift, value: 69});
		self.action.insert((40, TokenType::DoubleEquals), ActionNode{action: ParserAction::Shift, value: 70});
		self.action.insert((40, TokenType::TripleEquals), ActionNode{action: ParserAction::Shift, value: 71});
		self.action.insert((41, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 72});
		self.action.insert((41, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 73});
		self.action.insert((42, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 74});
		self.action.insert((43, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 45});
		self.action.insert((44, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 75});
		self.action.insert((45, TokenType::OperatorBinary), ActionNode{action: ParserAction::Goto, value: 76});
		self.action.insert((45, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((45, TokenType::LessThan), ActionNode{action: ParserAction::Shift, value: 66});
		self.action.insert((45, TokenType::LessThanEquals), ActionNode{action: ParserAction::Shift, value: 67});
		self.action.insert((45, TokenType::GreaterThan), ActionNode{action: ParserAction::Shift, value: 68});
		self.action.insert((45, TokenType::GreaterThanEquals), ActionNode{action: ParserAction::Shift, value: 69});
		self.action.insert((45, TokenType::DoubleEquals), ActionNode{action: ParserAction::Shift, value: 70});
		self.action.insert((45, TokenType::TripleEquals), ActionNode{action: ParserAction::Shift, value: 71});
		self.action.insert((46, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 77});
		self.action.insert((47, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 78});
		self.action.insert((48, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 79});
		self.action.insert((48, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 80});
		self.action.insert((49, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 2});
		self.action.insert((50, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((51, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((52, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((52, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((52, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((53, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 30});
		self.action.insert((53, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((53, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((53, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((53, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 31});
		self.action.insert((53, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 32});
		self.action.insert((54, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((54, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((54, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((55, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((55, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((55, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((55, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((55, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((56, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((57, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 81});
		self.action.insert((57, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 34});
		self.action.insert((57, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 36});
		self.action.insert((57, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((57, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((57, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((58, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 82});
		self.action.insert((58, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 34});
		self.action.insert((58, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 36});
		self.action.insert((58, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((58, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((58, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((59, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((59, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((59, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((59, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((59, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((60, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((60, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((60, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((61, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 83});
		self.action.insert((61, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 84});
		self.action.insert((61, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((61, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((61, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((62, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 85});
		self.action.insert((62, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 84});
		self.action.insert((62, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((62, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 38});
		self.action.insert((62, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((63, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 86});
		self.action.insert((64, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 23});
		self.action.insert((65, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 87});
		self.action.insert((66, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((67, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((68, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((69, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((70, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((71, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((72, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 88});
		self.action.insert((72, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 89});
		self.action.insert((72, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((72, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((72, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((72, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((72, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((72, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((72, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((72, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((72, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((72, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((72, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((72, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((72, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((72, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((72, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((72, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((72, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((72, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((72, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((72, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((72, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((73, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 90});
		self.action.insert((73, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((73, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((73, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((73, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((73, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((73, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((73, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((73, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((73, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((73, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((73, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((73, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((73, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((73, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((73, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((73, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((73, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((73, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((73, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((73, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((73, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((73, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((74, TokenType::Colon), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((74, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((75, TokenType::StatementLoopForOptions), ActionNode{action: ParserAction::Goto, value: 93});
		self.action.insert((75, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 92});
		self.action.insert((76, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 94});
		self.action.insert((77, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 95});
		self.action.insert((77, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((77, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((77, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((77, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((77, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((77, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((77, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((77, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((77, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((77, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((77, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((77, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((77, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((77, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((77, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((77, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((77, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((77, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((77, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((77, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((77, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((77, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((78, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((79, TokenType::FunctionParams), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((79, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 97});
		self.action.insert((79, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 96});
		self.action.insert((80, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((80, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((80, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((80, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((80, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((80, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((80, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((80, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((80, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((80, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((80, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((80, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((80, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((80, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((80, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((80, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((80, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((80, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((80, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((80, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((80, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((80, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((80, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((81, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((82, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((83, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((83, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((83, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((84, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 60});
		self.action.insert((84, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((84, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((84, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((84, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 61});
		self.action.insert((84, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 62});
		self.action.insert((85, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((85, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((85, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((86, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((86, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((86, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((86, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((86, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((87, TokenType::Colon), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((87, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((88, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 24});
		self.action.insert((88, TokenType::StatementConditionalElif), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((88, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 101});
		self.action.insert((89, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 102});
		self.action.insert((90, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 103});
		self.action.insert((91, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 104});
		self.action.insert((92, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 49});
		self.action.insert((92, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 105});
		self.action.insert((93, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 106});
		self.action.insert((94, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((95, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((96, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((97, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((97, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 108});
		self.action.insert((98, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 110});
		self.action.insert((99, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((100, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((101, TokenType::StatementConditionalTest), ActionNode{action: ParserAction::Goto, value: 112});
		self.action.insert((101, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((101, TokenType::OperatorUnary), ActionNode{action: ParserAction::Goto, value: 42});
		self.action.insert((101, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((102, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 1});
		self.action.insert((102, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 1});
		self.action.insert((103, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((103, TokenType::StatementConditionalElif), ActionNode{action: ParserAction::Goto, value: 113});
		self.action.insert((103, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 101});
		self.action.insert((104, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 1});
		self.action.insert((105, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 114});
		self.action.insert((106, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((106, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((106, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((106, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((106, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((106, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((106, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((106, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((106, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((106, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((106, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((106, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((106, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((106, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((106, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((106, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((106, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((106, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((106, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((106, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((106, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((107, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 52});
		self.action.insert((108, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((109, TokenType::FunctionParams), ActionNode{action: ParserAction::Goto, value: 116});
		self.action.insert((109, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 97});
		self.action.insert((109, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 96});
		self.action.insert((110, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 117});
		self.action.insert((110, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((110, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((110, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((110, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((110, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((110, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((110, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((110, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((110, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((110, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((110, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((110, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((110, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((110, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((110, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((110, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((110, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((110, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((110, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((110, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((110, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((110, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((111, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((112, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((112, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 119});
		self.action.insert((113, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((114, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 50});
		self.action.insert((114, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 120});
		self.action.insert((115, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 48});
		self.action.insert((116, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((117, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 121});
		self.action.insert((118, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 122});
		self.action.insert((118, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 89});
		self.action.insert((118, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((118, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((118, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((118, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((118, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((118, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((118, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((118, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((118, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((118, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((118, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((118, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((118, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((118, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((118, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((118, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((118, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((118, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((118, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((118, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((118, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((119, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 123});
		self.action.insert((119, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((119, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((119, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((119, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((119, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((119, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((119, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((119, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((119, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((119, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((119, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((119, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((119, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((119, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((119, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((119, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((119, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((119, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((119, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((119, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((119, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((119, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((120, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 124});
		self.action.insert((121, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 53});
		self.action.insert((122, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((122, TokenType::StatementConditionalElif), ActionNode{action: ParserAction::Goto, value: 125});
		self.action.insert((122, TokenType::StatementConditionalElse), ActionNode{action: ParserAction::Goto, value: 127});
		self.action.insert((122, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((123, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 128});
		self.action.insert((124, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 51});
		self.action.insert((125, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((126, TokenType::StatementConditionalTest), ActionNode{action: ParserAction::Goto, value: 129});
		self.action.insert((126, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((126, TokenType::OperatorUnary), ActionNode{action: ParserAction::Goto, value: 42});
		self.action.insert((126, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((127, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((128, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((128, TokenType::StatementConditionalElif), ActionNode{action: ParserAction::Goto, value: 130});
		self.action.insert((128, TokenType::StatementConditionalElse), ActionNode{action: ParserAction::Goto, value: 131});
		self.action.insert((128, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((129, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 132});
		self.action.insert((129, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 133});
		self.action.insert((130, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((131, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((132, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 134});
		self.action.insert((132, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 89});
		self.action.insert((132, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((132, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((132, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((132, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((132, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((132, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((132, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((132, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((132, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((132, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((132, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((132, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((132, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((132, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((132, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((132, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((132, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((132, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((132, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((132, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((132, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((133, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 135});
		self.action.insert((133, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((133, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((133, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((133, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((133, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((133, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((133, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((133, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((133, TokenType::StatementDefineFunction), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((133, TokenType::StatementDefineClass), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((133, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((133, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 17});
		self.action.insert((133, TokenType::If), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((133, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 19});
		self.action.insert((133, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 20});
		self.action.insert((133, TokenType::Define), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((133, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 14});
		self.action.insert((133, TokenType::For), ActionNode{action: ParserAction::Shift, value: 21});
		self.action.insert((133, TokenType::While), ActionNode{action: ParserAction::Shift, value: 22});
		self.action.insert((133, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 15});
		self.action.insert((133, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((133, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((134, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((134, TokenType::StatementConditionalElif), ActionNode{action: ParserAction::Goto, value: 125});
		self.action.insert((134, TokenType::StatementConditionalElse), ActionNode{action: ParserAction::Goto, value: 127});
		self.action.insert((134, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((135, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 136});
		self.action.insert((136, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((136, TokenType::StatementConditionalElif), ActionNode{action: ParserAction::Goto, value: 130});
		self.action.insert((136, TokenType::StatementConditionalElse), ActionNode{action: ParserAction::Goto, value: 131});
		self.action.insert((136, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 126});

    }
    pub fn parse(&mut self) -> Box<Node> {
        self.initialize();
        let mut stack: Vec<i32> = Vec::new();
        stack.push(0);
        let mut node_stack: Vec<Box<Node>> = Vec::new();
        let token_stack = self.token_stack.clone();
        let mut iterator = token_stack.iter();
        let mut token = iterator.next().unwrap();
        let mut item_stack: Vec<LexerResult> = Vec::new();
        loop {
            let mut state = *stack.last().unwrap();
            let action_node = self.action.get(&(state, token.token_type)).unwrap();
            println!("State: {}, TokenType: {:?} -> {:?} {}", state, token.token_type, action_node.action, action_node.value);
            match action_node.action {
                ParserAction::Shift => {
                    if token.token_type == TokenType::Term { self.token_stack.push(token.clone()) }
                    stack.push(action_node.value);
                    if token.token_type == TokenType::Term || token.token_type == TokenType::Identifier { item_stack.push(token.clone()); }
                    token = iterator.next().unwrap();
                }
                ParserAction::Reduce => {
                    let goto_node = self.goto.get(&action_node.value).unwrap();
                    for _ in 0..goto_node.value { stack.pop(); }
                    state = *stack.last().unwrap();
                    let goto_action = self.action.get(&(state, goto_node.token_type)).unwrap();
                    stack.push(goto_action.value);
                    {
                        match action_node.value {
							1 => {
								//NTS_STATEMENT_SUITE -> NTS_STATEMENT_LIST TS_NEWLINE
							}
							2 => {
								//NTS_STATEMENT_LIST -> NTS_STATEMENT TS_NEWLINE NTS_STATEMENT_LIST
							}
							3 => {
								//NTS_STATEMENT_LIST -> NTS_STATEMENT
							}
							4 => {
								//NTS_STATEMENT -> NTS_STATEMENT_SIMPLE
							}
							5 => {
								//NTS_STATEMENT -> NTS_STATEMENT_COMPLEX
							}
							6 => {
								//NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_EXPRESSION
							}
							7 => {
								//NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_ASSIGNMENT
							}
							8 => {
								//NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_CONDITIONAL
							}
							9 => {
								//NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_LOOP
							}
							10 => {
								//NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_DEFINE_FUNCTION
							}
							11 => {
								//NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_DEFINE_CLASS
							}
							12 => {
								//NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2 NTS_STATEMENT_EXPRESSION_P
							}
							13 => {
								//NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2
							}
							14 => {
								//NTS_STATEMENT_EXPRESSION_P -> TS_ADD NTS_STATEMENT_EXPRESSION
							}
							15 => {
								//NTS_STATEMENT_EXPRESSION_P -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION
							}
							16 => {
								//NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3 NTS_STATEMENT_EXPRESSION_2P
							}
							17 => {
								//NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3
							}
							18 => {
								//NTS_STATEMENT_EXPRESSION_2P -> TS_MULTIPLY NTS_STATEMENT_EXPRESSION_2
							}
							19 => {
								//NTS_STATEMENT_EXPRESSION_2P -> TS_DIVIDE NTS_STATEMENT_EXPRESSION_2
							}
							20 => {
								//NTS_STATEMENT_EXPRESSION_3 -> TS_TERM
							}
							21 => {
								//NTS_STATEMENT_EXPRESSION_3 -> TS_LEFT_PARENTHESIS NTS_STATEMENT_EXPRESSION TS_RIGHT_PARENTHESIS
							}
							22 => {
								//NTS_STATEMENT_EXPRESSION_3 -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION_3
							}
							23 => {
								//NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS NTS_STATEMENT_EXPRESSION
							}
							24 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_STATEMENT_CONDITIONAL_TEST TS_COLON NTS_STATEMENT_SUITE
							}
							25 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_STATEMENT_CONDITIONAL_TEST TS_COLON NTS_STATEMENT_SUITE NTS_STATEMENT_CONDITIONAL_ELIF
							}
							26 => {
								//NTS_STATEMENT_CONDITIONAL_ELIF -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_COLON NTS_STATEMENT_SUITE
							}
							27 => {
								//NTS_STATEMENT_CONDITIONAL_ELIF -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_COLON NTS_STATEMENT_SUITE NTS_STATEMENT_CONDITIONAL_ELIF
							}
							28 => {
								//NTS_STATEMENT_CONDITIONAL_ELIF -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_COLON NTS_STATEMENT_SUITE NTS_STATEMENT_CONDITIONAL_ELSE
							}
							29 => {
								//NTS_STATEMENT_CONDITIONAL_ELSE -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_COLON NTS_STATEMENT_SUITE
							}
							30 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE
							}
							31 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_ELIF
							}
							32 => {
								//NTS_STATEMENT_CONDITIONAL_ELIF -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE
							}
							33 => {
								//NTS_STATEMENT_CONDITIONAL_ELIF -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_ELIF
							}
							34 => {
								//NTS_STATEMENT_CONDITIONAL_ELIF -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_ELSE
							}
							35 => {
								//NTS_STATEMENT_CONDITIONAL_ELSE -> TS_ELIF NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE
							}
							36 => {
								//NTS_STATEMENT_CONDITIONAL_TEST -> TS_TERM NTS_OPERATOR_BINARY TS_TERM
							}
							37 => {
								//NTS_STATEMENT_CONDITIONAL_TEST -> NTS_OPERATOR_UNARY TS_TERM
							}
							38 => {
								//NTS_STATEMENT_CONDITIONAL_TEST -> TS_TERM
							}
							39 => {
								//NTS_OPERATOR_BINARY -> TS_LESS_THAN
							}
							40 => {
								//NTS_OPERATOR_BINARY -> TS_LESS_THAN_EQUALS
							}
							41 => {
								//NTS_OPERATOR_BINARY -> TS_GREATER_THAN
							}
							42 => {
								//NTS_OPERATOR_BINARY -> TS_GREATER_THAN_EQUALS
							}
							43 => {
								//NTS_OPERATOR_BINARY -> TS_DOUBLE_EQUALS
							}
							44 => {
								//NTS_OPERATOR_BINARY -> TS_TRIPLE_EQUALS
							}
							45 => {
								//NTS_OPERATOR_UNARY -> TS_NOT
							}
							46 => {
								//NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_FOR
							}
							47 => {
								//NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_WHILE
							}
							48 => {
								//NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COMMA NTS_STATEMENT_LOOP_FOR_OPTIONS TS_NEWLINE NTS_STATEMENT
							}
							49 => {
								//NTS_STATEMENT_LOOP_FOR_OPTIONS -> TS_TERM
							}
							50 => {
								//NTS_STATEMENT_LOOP_FOR_OPTIONS -> TS_TERM TS_COMMA TS_TERM
							}
							51 => {
								//NTS_STATEMENT_LOOP_FOR_OPTIONS -> TS_TERM TS_COMMA TS_TERM TS_COMMA TS_TERM
							}
							52 => {
								//NTS_STATEMENT_LOOP_WHILE -> TS_WHILE NTS_STATEMENT_CONDITIONAL_TEST TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE
							}
							53 => {
								//NTS_STATEMENT_DEFINE_FUNCTION -> TS_DEFINE TS_IDENTIFIER TS_LEFT_PARENTHESIS NTS_FUNCTION_PARAMS TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE
							}
							54 => {
								//NTS_FUNCTION_PARAMS -> TS_IDENTIFIER TS_COMMA NTS_FUNCTION_PARAMS
							}
							55 => {
								//NTS_FUNCTION_PARAMS -> TS_IDENTIFIER TS_RIGHT_PARENTHESIS
							}
							56 => {
								//NTS_FUNCTION_PARAMS -> TS_RIGHT_PARENTHESIS
							}
							57 => {
								//NTS_STATEMENT_DEFINE_CLASS -> TS_DEFINE TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE TS_RIGHT_CURLY_BRACE
							}
							_ => {
								//exhaustive
							}

                        }
                    }
                }
                ParserAction::Accept => {
                    println!("Parse Accepted!");
                    return Box::from(NodeMango { statement_suite: node_stack.pop().unwrap() });
                }
                _ => {
                    println!("Parse Error!");
                    return Box::from(NodeMango { statement_suite: node_stack.pop().unwrap() });
                }
            }
        }
    }
}
