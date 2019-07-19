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
		self.goto.insert(2, GotoNode{token_type: TokenType::StatementSuiteFunction, value: 2});
		self.goto.insert(3, GotoNode{token_type: TokenType::StatementSuiteClass, value: 2});
		self.goto.insert(4, GotoNode{token_type: TokenType::StatementList, value: 2});
		self.goto.insert(5, GotoNode{token_type: TokenType::StatementList, value: 1});
		self.goto.insert(6, GotoNode{token_type: TokenType::StatementListFunction, value: 2});
		self.goto.insert(7, GotoNode{token_type: TokenType::StatementListFunction, value: 1});
		self.goto.insert(8, GotoNode{token_type: TokenType::StatementListClass, value: 2});
		self.goto.insert(9, GotoNode{token_type: TokenType::StatementListClass, value: 1});
		self.goto.insert(10, GotoNode{token_type: TokenType::Statement, value: 1});
		self.goto.insert(11, GotoNode{token_type: TokenType::Statement, value: 1});
		self.goto.insert(12, GotoNode{token_type: TokenType::Statement, value: 1});
		self.goto.insert(13, GotoNode{token_type: TokenType::Statement, value: 1});
		self.goto.insert(14, GotoNode{token_type: TokenType::StatementLimited, value: 1});
		self.goto.insert(15, GotoNode{token_type: TokenType::StatementLimited, value: 1});
		self.goto.insert(16, GotoNode{token_type: TokenType::StatementRestricted, value: 1});
		self.goto.insert(17, GotoNode{token_type: TokenType::StatementSimple, value: 1});
		self.goto.insert(18, GotoNode{token_type: TokenType::StatementSimple, value: 1});
		self.goto.insert(19, GotoNode{token_type: TokenType::StatementSimple, value: 1});
		self.goto.insert(20, GotoNode{token_type: TokenType::StatementComplex, value: 1});
		self.goto.insert(21, GotoNode{token_type: TokenType::StatementComplex, value: 1});
		self.goto.insert(22, GotoNode{token_type: TokenType::StatementFunction, value: 7});
		self.goto.insert(23, GotoNode{token_type: TokenType::FunctionParams, value: 3});
		self.goto.insert(24, GotoNode{token_type: TokenType::FunctionParams, value: 1});
		self.goto.insert(25, GotoNode{token_type: TokenType::StatementClass, value: 5});
		self.goto.insert(26, GotoNode{token_type: TokenType::StatementExpression, value: 2});
		self.goto.insert(27, GotoNode{token_type: TokenType::StatementExpression, value: 1});
		self.goto.insert(28, GotoNode{token_type: TokenType::StatementExpressionP, value: 2});
		self.goto.insert(29, GotoNode{token_type: TokenType::StatementExpressionP, value: 2});
		self.goto.insert(30, GotoNode{token_type: TokenType::StatementExpression2, value: 2});
		self.goto.insert(31, GotoNode{token_type: TokenType::StatementExpression2, value: 1});
		self.goto.insert(32, GotoNode{token_type: TokenType::StatementExpression2p, value: 2});
		self.goto.insert(33, GotoNode{token_type: TokenType::StatementExpression2p, value: 2});
		self.goto.insert(34, GotoNode{token_type: TokenType::StatementExpression2p, value: 2});
		self.goto.insert(35, GotoNode{token_type: TokenType::StatementExpression3, value: 1});
		self.goto.insert(36, GotoNode{token_type: TokenType::StatementExpression3, value: 3});
		self.goto.insert(37, GotoNode{token_type: TokenType::StatementExpression3, value: 2});
		self.goto.insert(38, GotoNode{token_type: TokenType::StatementAssignment, value: 3});
		self.goto.insert(39, GotoNode{token_type: TokenType::StatementConditional, value: 5});
		self.goto.insert(40, GotoNode{token_type: TokenType::StatementConditional, value: 6});
		self.goto.insert(41, GotoNode{token_type: TokenType::StatementConditional, value: 6});
		self.goto.insert(42, GotoNode{token_type: TokenType::StatementConditional2, value: 6});
		self.goto.insert(43, GotoNode{token_type: TokenType::StatementConditional2, value: 6});
		self.goto.insert(44, GotoNode{token_type: TokenType::StatementConditional3, value: 4});
		self.goto.insert(45, GotoNode{token_type: TokenType::ConditionalExpression, value: 3});
		self.goto.insert(46, GotoNode{token_type: TokenType::ConditionalExpression, value: 2});
		self.goto.insert(47, GotoNode{token_type: TokenType::ComparisonOperator, value: 1});
		self.goto.insert(48, GotoNode{token_type: TokenType::ComparisonOperator, value: 1});
		self.goto.insert(49, GotoNode{token_type: TokenType::ComparisonOperator, value: 1});
		self.goto.insert(50, GotoNode{token_type: TokenType::ComparisonOperator, value: 1});
		self.goto.insert(51, GotoNode{token_type: TokenType::ComparisonOperator, value: 1});
		self.goto.insert(52, GotoNode{token_type: TokenType::ComparisonOperator, value: 1});
		self.goto.insert(53, GotoNode{token_type: TokenType::ComparisonOperatorUnary, value: 1});
		self.goto.insert(54, GotoNode{token_type: TokenType::StatementLoop, value: 1});
		self.goto.insert(55, GotoNode{token_type: TokenType::StatementLoop, value: 1});
		self.goto.insert(56, GotoNode{token_type: TokenType::StatementLoopFor, value: 7});
		self.goto.insert(57, GotoNode{token_type: TokenType::StatementLoopFor, value: 9});
		self.goto.insert(58, GotoNode{token_type: TokenType::StatementLoopWhile, value: 5});
		self.goto.insert(59, GotoNode{token_type: TokenType::StatementMatch, value: 5});
		self.goto.insert(60, GotoNode{token_type: TokenType::StatementMatchBody, value: 6});
		self.goto.insert(61, GotoNode{token_type: TokenType::StatementMatchBody, value: 4});
		self.action.insert((0, TokenType::StatementSuite), ActionNode{action: ParserAction::Goto, value: 1});
		self.action.insert((0, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 2});
		self.action.insert((0, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((0, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((0, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((0, TokenType::StatementFunction), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((0, TokenType::StatementClass), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((0, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((0, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((0, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((0, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((0, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((0, TokenType::At), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((0, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 15});
		self.action.insert((0, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 14});
		self.action.insert((0, TokenType::If), ActionNode{action: ParserAction::Shift, value: 20});
		self.action.insert((0, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 21});
		self.action.insert((0, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 22});
		self.action.insert((0, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 25});
		self.action.insert((0, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 17});
		self.action.insert((0, TokenType::For), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((0, TokenType::While), ActionNode{action: ParserAction::Shift, value: 24});
		self.action.insert((0, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((0, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((0, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((1, TokenType::EndOfFile), ActionNode{action: ParserAction::Accept, value: -1});
		self.action.insert((2, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 26});
		self.action.insert((3, TokenType::StatementList), ActionNode{action: ParserAction::Goto, value: 27});
		self.action.insert((3, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 5});
		self.action.insert((3, TokenType::Statement), ActionNode{action: ParserAction::Goto, value: 3});
		self.action.insert((3, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 4});
		self.action.insert((3, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 5});
		self.action.insert((3, TokenType::StatementFunction), ActionNode{action: ParserAction::Goto, value: 6});
		self.action.insert((3, TokenType::StatementClass), ActionNode{action: ParserAction::Goto, value: 7});
		self.action.insert((3, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 8});
		self.action.insert((3, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 9});
		self.action.insert((3, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 10});
		self.action.insert((3, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 11});
		self.action.insert((3, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 12});
		self.action.insert((3, TokenType::At), ActionNode{action: ParserAction::Shift, value: 13});
		self.action.insert((3, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 15});
		self.action.insert((3, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 14});
		self.action.insert((3, TokenType::If), ActionNode{action: ParserAction::Shift, value: 20});
		self.action.insert((3, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 21});
		self.action.insert((3, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 22});
		self.action.insert((3, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 25});
		self.action.insert((3, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 17});
		self.action.insert((3, TokenType::For), ActionNode{action: ParserAction::Shift, value: 23});
		self.action.insert((3, TokenType::While), ActionNode{action: ParserAction::Shift, value: 24});
		self.action.insert((3, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((3, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((3, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((4, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((4, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 10});
		self.action.insert((5, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((5, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 11});
		self.action.insert((6, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((6, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 12});
		self.action.insert((7, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((7, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 13});
		self.action.insert((8, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((8, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((9, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((9, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((10, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((10, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((11, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((11, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((12, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((12, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((13, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 28});
		self.action.insert((14, TokenType::Equals), ActionNode{action: ParserAction::Shift, value: 29});
		self.action.insert((15, TokenType::StatementExpressionP), ActionNode{action: ParserAction::Goto, value: 30});
		self.action.insert((15, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 32});
		self.action.insert((15, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((15, TokenType::Add), ActionNode{action: ParserAction::Shift, value: 31});
		self.action.insert((16, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 33});
		self.action.insert((16, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((16, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((16, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((17, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 34});
		self.action.insert((17, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((17, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((17, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 36});
		self.action.insert((17, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((18, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((18, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((19, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 38});
		self.action.insert((19, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 39});
		self.action.insert((19, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((19, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((19, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((19, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((20, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 45});
		self.action.insert((20, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((20, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((20, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((21, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((21, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((22, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((22, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((23, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 48});
		self.action.insert((24, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 49});
		self.action.insert((24, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((24, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((24, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((25, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 50});
		self.action.insert((26, TokenType::EndOfFile), ActionNode{action: ParserAction::Reduce, value: 1});
		self.action.insert((27, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 4});
		self.action.insert((28, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 51});
		self.action.insert((28, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 52});
		self.action.insert((29, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 53});
		self.action.insert((29, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 15});
		self.action.insert((29, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 17});
		self.action.insert((29, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((29, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((29, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((30, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((30, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((31, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 54});
		self.action.insert((31, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 15});
		self.action.insert((31, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 17});
		self.action.insert((31, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((31, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((31, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((32, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 55});
		self.action.insert((32, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 15});
		self.action.insert((32, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 17});
		self.action.insert((32, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((32, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((32, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((33, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((33, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((34, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((34, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((35, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 56});
		self.action.insert((35, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 57});
		self.action.insert((35, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((35, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((35, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((36, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 58});
		self.action.insert((36, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 57});
		self.action.insert((36, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((36, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((36, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((37, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 59});
		self.action.insert((37, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 57});
		self.action.insert((37, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 18});
		self.action.insert((37, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 19});
		self.action.insert((37, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 16});
		self.action.insert((38, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 60});
		self.action.insert((39, TokenType::StatementExpressionP), ActionNode{action: ParserAction::Goto, value: 61});
		self.action.insert((39, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((39, TokenType::Add), ActionNode{action: ParserAction::Shift, value: 62});
		self.action.insert((39, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 63});
		self.action.insert((40, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 64});
		self.action.insert((40, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((40, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((40, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((41, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 65});
		self.action.insert((41, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((41, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((41, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((41, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 66});
		self.action.insert((41, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 67});
		self.action.insert((41, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 68});
		self.action.insert((42, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((42, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((42, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((42, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((42, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((42, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((43, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 69});
		self.action.insert((43, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 39});
		self.action.insert((43, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((43, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((43, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((43, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((44, TokenType::ComparisonOperator), ActionNode{action: ParserAction::Goto, value: 70});
		self.action.insert((44, TokenType::LessThan), ActionNode{action: ParserAction::Shift, value: 71});
		self.action.insert((44, TokenType::LessThanEquals), ActionNode{action: ParserAction::Shift, value: 72});
		self.action.insert((44, TokenType::GreaterThan), ActionNode{action: ParserAction::Shift, value: 73});
		self.action.insert((44, TokenType::GreaterThanEquals), ActionNode{action: ParserAction::Shift, value: 74});
		self.action.insert((44, TokenType::DoubleEquals), ActionNode{action: ParserAction::Shift, value: 75});
		self.action.insert((44, TokenType::TripleEquals), ActionNode{action: ParserAction::Shift, value: 76});
		self.action.insert((45, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 77});
		self.action.insert((46, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 78});
		self.action.insert((47, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 53});
		self.action.insert((48, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 79});
		self.action.insert((49, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 80});
		self.action.insert((50, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 81});
		self.action.insert((51, TokenType::FunctionParams), ActionNode{action: ParserAction::Goto, value: 83});
		self.action.insert((51, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 82});
		self.action.insert((52, TokenType::StatementSuiteClass), ActionNode{action: ParserAction::Goto, value: 84});
		self.action.insert((52, TokenType::StatementListClass), ActionNode{action: ParserAction::Goto, value: 85});
		self.action.insert((52, TokenType::StatementRestricted), ActionNode{action: ParserAction::Goto, value: 86});
		self.action.insert((52, TokenType::StatementFunction), ActionNode{action: ParserAction::Goto, value: 87});
		self.action.insert((52, TokenType::At), ActionNode{action: ParserAction::Shift, value: 88});
		self.action.insert((53, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((53, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((54, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((54, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((55, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((55, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((56, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((56, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((57, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 34});
		self.action.insert((57, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((57, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 35});
		self.action.insert((57, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 36});
		self.action.insert((57, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 37});
		self.action.insert((58, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((58, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((59, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((59, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((60, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((60, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((61, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((62, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 89});
		self.action.insert((62, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 39});
		self.action.insert((62, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((62, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((62, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((62, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((63, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 90});
		self.action.insert((63, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 39});
		self.action.insert((63, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((63, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((63, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((63, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((64, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((64, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((64, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((64, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((64, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((64, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((65, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((65, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((65, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((66, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 91});
		self.action.insert((66, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 92});
		self.action.insert((66, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((66, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((66, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((67, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 93});
		self.action.insert((67, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 92});
		self.action.insert((67, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((67, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((67, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((68, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 94});
		self.action.insert((68, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 92});
		self.action.insert((68, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((68, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((68, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((69, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 95});
		self.action.insert((70, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 96});
		self.action.insert((71, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 47});
		self.action.insert((72, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 48});
		self.action.insert((73, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 49});
		self.action.insert((74, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 50});
		self.action.insert((75, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 51});
		self.action.insert((76, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 52});
		self.action.insert((77, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 97});
		self.action.insert((77, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((77, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((77, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((77, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((77, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((77, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((77, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((77, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((77, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((77, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((77, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((77, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((77, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((77, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((77, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((77, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((77, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((77, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((77, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((77, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((77, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((78, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 46});
		self.action.insert((79, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 119});
		self.action.insert((80, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 120});
		self.action.insert((80, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((80, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((80, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((80, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((80, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((80, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((80, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((80, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((80, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((80, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((80, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((80, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((80, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((80, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((80, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((80, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((80, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((80, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((80, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((80, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((80, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((81, TokenType::StatementMatchBody), ActionNode{action: ParserAction::Goto, value: 127});
		self.action.insert((81, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 121});
		self.action.insert((81, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 122});
		self.action.insert((81, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 124});
		self.action.insert((81, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((81, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((81, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((82, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 24});
		self.action.insert((82, TokenType::Comma), ActionNode{action: ParserAction::Reduce, value: 24});
		self.action.insert((83, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 128});
		self.action.insert((83, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 129});
		self.action.insert((84, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 130});
		self.action.insert((85, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 131});
		self.action.insert((86, TokenType::StatementListClass), ActionNode{action: ParserAction::Goto, value: 132});
		self.action.insert((86, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 9});
		self.action.insert((86, TokenType::StatementRestricted), ActionNode{action: ParserAction::Goto, value: 86});
		self.action.insert((86, TokenType::StatementFunction), ActionNode{action: ParserAction::Goto, value: 87});
		self.action.insert((86, TokenType::At), ActionNode{action: ParserAction::Shift, value: 88});
		self.action.insert((87, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((87, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 16});
		self.action.insert((88, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 133});
		self.action.insert((89, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((90, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((91, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((91, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((91, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((92, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 65});
		self.action.insert((92, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((92, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((92, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((92, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 66});
		self.action.insert((92, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 67});
		self.action.insert((92, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 68});
		self.action.insert((93, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((93, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((93, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((94, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((94, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((94, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((95, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((95, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((95, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((95, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((95, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((95, TokenType::RightParenthesis), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((96, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 45});
		self.action.insert((97, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 134});
		self.action.insert((98, TokenType::Newline), ActionNode{action: ParserAction::Shift, value: 135});
		self.action.insert((99, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 136});
		self.action.insert((99, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 7});
		self.action.insert((99, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((99, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((99, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((99, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((99, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((99, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((99, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((99, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((99, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((99, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((99, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((99, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((99, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((99, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((99, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((99, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((99, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((99, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((99, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((99, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((100, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((100, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 14});
		self.action.insert((101, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((101, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 15});
		self.action.insert((102, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((102, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 17});
		self.action.insert((103, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((103, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 18});
		self.action.insert((104, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((104, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 19});
		self.action.insert((105, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((105, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 20});
		self.action.insert((106, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((106, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 21});
		self.action.insert((107, TokenType::Equals), ActionNode{action: ParserAction::Shift, value: 137});
		self.action.insert((108, TokenType::StatementExpressionP), ActionNode{action: ParserAction::Goto, value: 138});
		self.action.insert((108, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 140});
		self.action.insert((108, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((108, TokenType::Add), ActionNode{action: ParserAction::Shift, value: 139});
		self.action.insert((109, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 141});
		self.action.insert((109, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((109, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((109, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((110, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 142});
		self.action.insert((110, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((110, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 143});
		self.action.insert((110, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 144});
		self.action.insert((110, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 145});
		self.action.insert((111, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((111, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((112, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 146});
		self.action.insert((112, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 39});
		self.action.insert((112, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((112, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((112, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((112, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((113, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 147});
		self.action.insert((113, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((113, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((113, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((114, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((114, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 54});
		self.action.insert((115, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((115, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 55});
		self.action.insert((116, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 148});
		self.action.insert((117, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 149});
		self.action.insert((117, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((117, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((117, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((118, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 150});
		self.action.insert((119, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 151});
		self.action.insert((119, TokenType::Ellipsis), ActionNode{action: ParserAction::Shift, value: 152});
		self.action.insert((120, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 153});
		self.action.insert((121, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 154});
		self.action.insert((122, TokenType::StatementExpressionP), ActionNode{action: ParserAction::Goto, value: 155});
		self.action.insert((122, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 27});
		self.action.insert((122, TokenType::Add), ActionNode{action: ParserAction::Shift, value: 156});
		self.action.insert((122, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 157});
		self.action.insert((123, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 158});
		self.action.insert((123, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((123, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((123, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((124, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 159});
		self.action.insert((124, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((124, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((124, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((124, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 160});
		self.action.insert((124, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 161});
		self.action.insert((124, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 162});
		self.action.insert((125, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((125, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((125, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((125, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((125, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((125, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 35});
		self.action.insert((126, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 163});
		self.action.insert((126, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 39});
		self.action.insert((126, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 41});
		self.action.insert((126, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 42});
		self.action.insert((126, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 43});
		self.action.insert((126, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 40});
		self.action.insert((127, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 164});
		self.action.insert((127, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 165});
		self.action.insert((128, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 166});
		self.action.insert((128, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((128, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((128, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((128, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((128, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((128, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((128, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((128, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((128, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((128, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((128, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((128, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((128, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((128, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((128, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((128, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((128, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((128, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((128, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((128, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((128, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((129, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 167});
		self.action.insert((130, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((130, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 25});
		self.action.insert((131, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 3});
		self.action.insert((132, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 8});
		self.action.insert((133, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 168});
		self.action.insert((134, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::StatementConditional2), ActionNode{action: ParserAction::Goto, value: 169});
		self.action.insert((134, TokenType::StatementConditional3), ActionNode{action: ParserAction::Goto, value: 170});
		self.action.insert((134, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((134, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 171});
		self.action.insert((134, TokenType::Else), ActionNode{action: ParserAction::Shift, value: 172});
		self.action.insert((135, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 2});
		self.action.insert((136, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 6});
		self.action.insert((137, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 173});
		self.action.insert((137, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((137, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((137, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((137, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((137, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((138, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((138, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((139, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 174});
		self.action.insert((139, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((139, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((139, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((139, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((139, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((140, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 175});
		self.action.insert((140, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((140, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((140, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((140, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((140, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((141, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((141, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((142, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((142, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((143, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 176});
		self.action.insert((143, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 177});
		self.action.insert((143, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((143, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((143, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((144, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 178});
		self.action.insert((144, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 177});
		self.action.insert((144, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((144, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((144, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((145, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 179});
		self.action.insert((145, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 177});
		self.action.insert((145, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((145, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((145, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((146, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 180});
		self.action.insert((147, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 181});
		self.action.insert((148, TokenType::Colon), ActionNode{action: ParserAction::Shift, value: 182});
		self.action.insert((149, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 183});
		self.action.insert((150, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 184});
		self.action.insert((151, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 185});
		self.action.insert((151, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((151, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((151, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((151, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((151, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((151, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((151, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((151, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((151, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((151, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((151, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((151, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((151, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((151, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((151, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((151, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((151, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((151, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((151, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((151, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((151, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((152, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 186});
		self.action.insert((153, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((153, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((154, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 187});
		self.action.insert((154, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((154, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((154, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((154, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((154, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((154, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((154, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((154, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((154, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((154, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((154, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((154, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((154, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((154, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((154, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((154, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((154, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((154, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((154, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((154, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((154, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((155, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 26});
		self.action.insert((156, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 188});
		self.action.insert((156, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 122});
		self.action.insert((156, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 124});
		self.action.insert((156, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((156, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((156, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((157, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 189});
		self.action.insert((157, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 122});
		self.action.insert((157, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 124});
		self.action.insert((157, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((157, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((157, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((158, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((158, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((158, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((158, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((158, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((158, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 37});
		self.action.insert((159, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((159, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((159, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 30});
		self.action.insert((160, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 190});
		self.action.insert((160, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 191});
		self.action.insert((160, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((160, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((160, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((161, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 192});
		self.action.insert((161, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 191});
		self.action.insert((161, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((161, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((161, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((162, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 193});
		self.action.insert((162, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 191});
		self.action.insert((162, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((162, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((162, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((163, TokenType::RightParenthesis), ActionNode{action: ParserAction::Shift, value: 194});
		self.action.insert((164, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((164, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((165, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 195});
		self.action.insert((165, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 122});
		self.action.insert((165, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 124});
		self.action.insert((165, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((165, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((165, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((166, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 196});
		self.action.insert((167, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 23});
		self.action.insert((167, TokenType::Comma), ActionNode{action: ParserAction::Reduce, value: 23});
		self.action.insert((168, TokenType::FunctionParams), ActionNode{action: ParserAction::Goto, value: 197});
		self.action.insert((168, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 82});
		self.action.insert((169, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((169, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 198});
		self.action.insert((170, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((170, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((171, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 199});
		self.action.insert((171, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((171, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((171, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((172, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 200});
		self.action.insert((173, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((173, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 38});
		self.action.insert((174, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((174, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((175, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((175, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((176, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((176, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((177, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 142});
		self.action.insert((177, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((177, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 143});
		self.action.insert((177, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 144});
		self.action.insert((177, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 145});
		self.action.insert((178, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((178, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((179, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((179, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((180, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((180, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((181, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 201});
		self.action.insert((181, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((181, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((181, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((181, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((181, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((181, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((181, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((181, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((181, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((181, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((181, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((181, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((181, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((181, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((181, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((181, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((181, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((181, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((181, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((181, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((181, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((182, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 202});
		self.action.insert((183, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 203});
		self.action.insert((183, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((183, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((183, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((183, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((183, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((183, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((183, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((183, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((183, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((183, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((183, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((183, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((183, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((183, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((183, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((183, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((183, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((183, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((183, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((183, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((183, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((184, TokenType::StatementMatchBody), ActionNode{action: ParserAction::Goto, value: 204});
		self.action.insert((184, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 121});
		self.action.insert((184, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 122});
		self.action.insert((184, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 124});
		self.action.insert((184, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 125});
		self.action.insert((184, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 126});
		self.action.insert((184, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 123});
		self.action.insert((185, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 205});
		self.action.insert((186, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 206});
		self.action.insert((187, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 207});
		self.action.insert((188, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 28});
		self.action.insert((189, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 29});
		self.action.insert((190, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((190, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((190, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 32});
		self.action.insert((191, TokenType::StatementExpression2p), ActionNode{action: ParserAction::Goto, value: 159});
		self.action.insert((191, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((191, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((191, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 31});
		self.action.insert((191, TokenType::Multiply), ActionNode{action: ParserAction::Shift, value: 160});
		self.action.insert((191, TokenType::Divide), ActionNode{action: ParserAction::Shift, value: 161});
		self.action.insert((191, TokenType::Modulo), ActionNode{action: ParserAction::Shift, value: 162});
		self.action.insert((192, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((192, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((192, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 33});
		self.action.insert((193, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((193, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((193, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 34});
		self.action.insert((194, TokenType::Modulo), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((194, TokenType::Divide), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((194, TokenType::Multiply), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((194, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((194, TokenType::Add), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((194, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 36});
		self.action.insert((195, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 208});
		self.action.insert((196, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((196, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((197, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 209});
		self.action.insert((197, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 129});
		self.action.insert((198, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 210});
		self.action.insert((198, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((198, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((198, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((199, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 211});
		self.action.insert((200, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 212});
		self.action.insert((200, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((200, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((200, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((200, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((200, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((200, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((200, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((200, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((200, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((200, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((200, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((200, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((200, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((200, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((200, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((200, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((200, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((200, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((200, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((200, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((200, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((201, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 213});
		self.action.insert((202, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 214});
		self.action.insert((202, TokenType::Ellipsis), ActionNode{action: ParserAction::Shift, value: 215});
		self.action.insert((203, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 216});
		self.action.insert((204, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 217});
		self.action.insert((204, TokenType::Comma), ActionNode{action: ParserAction::Shift, value: 165});
		self.action.insert((205, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((205, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((206, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 218});
		self.action.insert((206, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((206, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((206, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((206, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((206, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((206, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((206, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((206, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((206, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((206, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((206, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((206, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((206, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((206, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((206, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((206, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((206, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((206, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((206, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((206, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((206, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((207, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 61});
		self.action.insert((207, TokenType::Comma), ActionNode{action: ParserAction::Reduce, value: 61});
		self.action.insert((208, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 219});
		self.action.insert((208, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((208, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((208, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((208, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((208, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((208, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((208, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((208, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((208, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((208, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((208, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((208, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((208, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((208, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((208, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((208, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((208, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((208, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((208, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((208, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((208, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((209, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 220});
		self.action.insert((209, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((209, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((209, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((209, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((209, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((209, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((209, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((209, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((209, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((209, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((209, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((209, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((209, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((209, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((209, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((209, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((209, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((209, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((209, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((209, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((209, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((210, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 221});
		self.action.insert((211, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 222});
		self.action.insert((211, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((211, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((211, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((211, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((211, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((211, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((211, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((211, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((211, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((211, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((211, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((211, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((211, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((211, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((211, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((211, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((211, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((211, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((211, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((211, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((211, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((212, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 223});
		self.action.insert((213, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::StatementConditional2), ActionNode{action: ParserAction::Goto, value: 224});
		self.action.insert((213, TokenType::StatementConditional3), ActionNode{action: ParserAction::Goto, value: 225});
		self.action.insert((213, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 39});
		self.action.insert((213, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 226});
		self.action.insert((213, TokenType::Else), ActionNode{action: ParserAction::Shift, value: 227});
		self.action.insert((214, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 228});
		self.action.insert((214, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((214, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((214, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((214, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((214, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((214, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((214, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((214, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((214, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((214, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((214, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((214, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((214, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((214, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((214, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((214, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((214, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((214, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((214, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((214, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((214, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((215, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 229});
		self.action.insert((216, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((216, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 58});
		self.action.insert((217, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((217, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 59});
		self.action.insert((218, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 230});
		self.action.insert((219, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 231});
		self.action.insert((220, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 232});
		self.action.insert((221, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 233});
		self.action.insert((221, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((221, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((221, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((221, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((221, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((221, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((221, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((221, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((221, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((221, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((221, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((221, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((221, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((221, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((221, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((221, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((221, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((221, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((221, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((221, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((221, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((222, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 234});
		self.action.insert((223, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((223, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((224, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 40});
		self.action.insert((224, TokenType::Elif), ActionNode{action: ParserAction::Shift, value: 235});
		self.action.insert((225, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((225, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 41});
		self.action.insert((226, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 236});
		self.action.insert((226, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((226, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((226, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((227, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 237});
		self.action.insert((228, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 238});
		self.action.insert((229, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 239});
		self.action.insert((230, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((230, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((231, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Reduce, value: 60});
		self.action.insert((231, TokenType::Comma), ActionNode{action: ParserAction::Reduce, value: 60});
		self.action.insert((232, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((232, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 22});
		self.action.insert((233, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 240});
		self.action.insert((234, TokenType::StatementConditional3), ActionNode{action: ParserAction::Goto, value: 241});
		self.action.insert((234, TokenType::Else), ActionNode{action: ParserAction::Shift, value: 242});
		self.action.insert((235, TokenType::ConditionalExpression), ActionNode{action: ParserAction::Goto, value: 243});
		self.action.insert((235, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 44});
		self.action.insert((235, TokenType::ComparisonOperatorUnary), ActionNode{action: ParserAction::Goto, value: 46});
		self.action.insert((235, TokenType::Not), ActionNode{action: ParserAction::Shift, value: 47});
		self.action.insert((236, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 244});
		self.action.insert((237, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 245});
		self.action.insert((237, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((237, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((237, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((237, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((237, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((237, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((237, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((237, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((237, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((237, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((237, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((237, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((237, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((237, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((237, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((237, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((237, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((237, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((237, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((237, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((237, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((238, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((238, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 56});
		self.action.insert((239, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 246});
		self.action.insert((239, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((239, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((239, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((239, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((239, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((239, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((239, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((239, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((239, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((239, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((239, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((239, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((239, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((239, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((239, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((239, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((239, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((239, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((239, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((239, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((239, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((240, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((240, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((241, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((241, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((242, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 247});
		self.action.insert((243, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 248});
		self.action.insert((244, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 249});
		self.action.insert((244, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((244, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((244, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((244, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((244, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((244, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((244, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((244, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((244, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((244, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((244, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((244, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((244, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((244, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((244, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((244, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((244, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((244, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((244, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((244, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((244, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((245, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 250});
		self.action.insert((246, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 251});
		self.action.insert((247, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 252});
		self.action.insert((247, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((247, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((247, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((247, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((247, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((247, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((247, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((247, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((247, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((247, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((247, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((247, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((247, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((247, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((247, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((247, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((247, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((247, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((247, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((247, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((247, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((248, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 253});
		self.action.insert((248, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((248, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((248, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((248, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((248, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((248, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((248, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((248, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((248, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((248, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((248, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((248, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((248, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((248, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((248, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((248, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((248, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((248, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((248, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((248, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((248, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((249, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 254});
		self.action.insert((250, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((250, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((251, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((251, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 57});
		self.action.insert((252, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 255});
		self.action.insert((253, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 256});
		self.action.insert((254, TokenType::StatementConditional3), ActionNode{action: ParserAction::Goto, value: 257});
		self.action.insert((254, TokenType::Else), ActionNode{action: ParserAction::Shift, value: 258});
		self.action.insert((255, TokenType::At), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((255, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((256, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((256, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 42});
		self.action.insert((257, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((257, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 43});
		self.action.insert((258, TokenType::LeftCurlyBrace), ActionNode{action: ParserAction::Shift, value: 259});
		self.action.insert((259, TokenType::StatementSuiteFunction), ActionNode{action: ParserAction::Goto, value: 260});
		self.action.insert((259, TokenType::StatementListFunction), ActionNode{action: ParserAction::Goto, value: 98});
		self.action.insert((259, TokenType::StatementLimited), ActionNode{action: ParserAction::Goto, value: 99});
		self.action.insert((259, TokenType::StatementSimple), ActionNode{action: ParserAction::Goto, value: 100});
		self.action.insert((259, TokenType::StatementComplex), ActionNode{action: ParserAction::Goto, value: 101});
		self.action.insert((259, TokenType::StatementExpression), ActionNode{action: ParserAction::Goto, value: 102});
		self.action.insert((259, TokenType::StatementAssignment), ActionNode{action: ParserAction::Goto, value: 103});
		self.action.insert((259, TokenType::StatementConditional), ActionNode{action: ParserAction::Goto, value: 104});
		self.action.insert((259, TokenType::StatementLoop), ActionNode{action: ParserAction::Goto, value: 105});
		self.action.insert((259, TokenType::StatementMatch), ActionNode{action: ParserAction::Goto, value: 106});
		self.action.insert((259, TokenType::StatementExpression2), ActionNode{action: ParserAction::Goto, value: 108});
		self.action.insert((259, TokenType::Identifier), ActionNode{action: ParserAction::Shift, value: 107});
		self.action.insert((259, TokenType::If), ActionNode{action: ParserAction::Shift, value: 113});
		self.action.insert((259, TokenType::StatementLoopFor), ActionNode{action: ParserAction::Goto, value: 114});
		self.action.insert((259, TokenType::StatementLoopWhile), ActionNode{action: ParserAction::Goto, value: 115});
		self.action.insert((259, TokenType::Match), ActionNode{action: ParserAction::Shift, value: 118});
		self.action.insert((259, TokenType::StatementExpression3), ActionNode{action: ParserAction::Goto, value: 110});
		self.action.insert((259, TokenType::For), ActionNode{action: ParserAction::Shift, value: 116});
		self.action.insert((259, TokenType::While), ActionNode{action: ParserAction::Shift, value: 117});
		self.action.insert((259, TokenType::Term), ActionNode{action: ParserAction::Shift, value: 111});
		self.action.insert((259, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Shift, value: 112});
		self.action.insert((259, TokenType::Subtract), ActionNode{action: ParserAction::Shift, value: 109});
		self.action.insert((260, TokenType::RightCurlyBrace), ActionNode{action: ParserAction::Shift, value: 261});
		self.action.insert((261, TokenType::Match), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::While), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::For), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::If), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::Identifier), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::Subtract), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::LeftParenthesis), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::Term), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::Newline), ActionNode{action: ParserAction::Reduce, value: 44});
		self.action.insert((261, TokenType::Elif), ActionNode{action: ParserAction::Reduce, value: 44});

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
								//NTS_STATEMENT_SUITE_FUNCTION -> NTS_STATEMENT_LIST_FUNCTION TS_NEWLINE
							}
							3 => {
								//NTS_STATEMENT_SUITE_CLASS -> NTS_STATEMENT_LIST_CLASS TS_NEWLINE
							}
							4 => {
								//NTS_STATEMENT_LIST -> NTS_STATEMENT NTS_STATEMENT_LIST
							}
							5 => {
								//NTS_STATEMENT_LIST -> NTS_STATEMENT
							}
							6 => {
								//NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED NTS_STATEMENT_LIST_FUNCTION
							}
							7 => {
								//NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED
							}
							8 => {
								//NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED NTS_STATEMENT_LIST_CLASS
							}
							9 => {
								//NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED
							}
							10 => {
								//NTS_STATEMENT -> NTS_STATEMENT_SIMPLE
							}
							11 => {
								//NTS_STATEMENT -> NTS_STATEMENT_COMPLEX
							}
							12 => {
								//NTS_STATEMENT -> NTS_STATEMENT_FUNCTION
							}
							13 => {
								//NTS_STATEMENT -> NTS_STATEMENT_CLASS
							}
							14 => {
								//NTS_STATEMENT_LIMITED -> NTS_STATEMENT_SIMPLE
							}
							15 => {
								//NTS_STATEMENT_LIMITED -> NTS_STATEMENT_COMPLEX
							}
							16 => {
								//NTS_STATEMENT_RESTRICTED -> NTS_STATEMENT_FUNCTION
							}
							17 => {
								//NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_EXPRESSION
							}
							18 => {
								//NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_ASSIGNMENT
							}
							19 => {
								//NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_CONDITIONAL
							}
							20 => {
								//NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_LOOP
							}
							21 => {
								//NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_MATCH
							}
							22 => {
								//NTS_STATEMENT_FUNCTION -> TS_AT TS_IDENTIFIER TS_COLON NTS_FUNCTION_PARAMS TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							23 => {
								//NTS_FUNCTION_PARAMS -> NTS_FUNCTION_PARAMS TS_COMMA TS_IDENTIFIER
							}
							24 => {
								//NTS_FUNCTION_PARAMS -> TS_IDENTIFIER
							}
							25 => {
								//NTS_STATEMENT_CLASS -> TS_AT TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_CLASS TS_RIGHT_CURLY_BRACE
							}
							26 => {
								//NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2 NTS_STATEMENT_EXPRESSION_P
							}
							27 => {
								//NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2
							}
							28 => {
								//NTS_STATEMENT_EXPRESSION_P -> TS_ADD NTS_STATEMENT_EXPRESSION
							}
							29 => {
								//NTS_STATEMENT_EXPRESSION_P -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION
							}
							30 => {
								//NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3 NTS_STATEMENT_EXPRESSION_2P
							}
							31 => {
								//NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3
							}
							32 => {
								//NTS_STATEMENT_EXPRESSION_2P -> TS_MULTIPLY NTS_STATEMENT_EXPRESSION_2
							}
							33 => {
								//NTS_STATEMENT_EXPRESSION_2P -> TS_DIVIDE NTS_STATEMENT_EXPRESSION_2
							}
							34 => {
								//NTS_STATEMENT_EXPRESSION_2P -> TS_MODULO NTS_STATEMENT_EXPRESSION_2
							}
							35 => {
								//NTS_STATEMENT_EXPRESSION_3 -> TS_TERM
							}
							36 => {
								//NTS_STATEMENT_EXPRESSION_3 -> TS_LEFT_PARENTHESIS NTS_STATEMENT_EXPRESSION TS_RIGHT_PARENTHESIS
							}
							37 => {
								//NTS_STATEMENT_EXPRESSION_3 -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION_3
							}
							38 => {
								//NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS NTS_STATEMENT_EXPRESSION
							}
							39 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							40 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_2
							}
							41 => {
								//NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
							}
							42 => {
								//NTS_STATEMENT_CONDITIONAL_2 -> NTS_STATEMENT_CONDITIONAL_2 TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							43 => {
								//NTS_STATEMENT_CONDITIONAL_2 -> TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
							}
							44 => {
								//NTS_STATEMENT_CONDITIONAL_3 -> TS_ELSE TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							45 => {
								//NTS_CONDITIONAL_EXPRESSION -> TS_TERM NTS_COMPARISON_OPERATOR TS_TERM
							}
							46 => {
								//NTS_CONDITIONAL_EXPRESSION -> NTS_COMPARISON_OPERATOR_UNARY TS_TERM
							}
							47 => {
								//NTS_COMPARISON_OPERATOR -> TS_LESS_THAN
							}
							48 => {
								//NTS_COMPARISON_OPERATOR -> TS_LESS_THAN_EQUALS
							}
							49 => {
								//NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN
							}
							50 => {
								//NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN_EQUALS
							}
							51 => {
								//NTS_COMPARISON_OPERATOR -> TS_DOUBLE_EQUALS
							}
							52 => {
								//NTS_COMPARISON_OPERATOR -> TS_TRIPLE_EQUALS
							}
							53 => {
								//NTS_COMPARISON_OPERATOR_UNARY -> TS_NOT
							}
							54 => {
								//NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_FOR
							}
							55 => {
								//NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_WHILE
							}
							56 => {
								//NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON TS_TERM TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							57 => {
								//NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON TS_TERM TS_ELLIPSIS TS_TERM TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							58 => {
								//NTS_STATEMENT_LOOP_WHILE -> TS_WHILE NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							59 => {
								//NTS_STATEMENT_MATCH -> TS_MATCH TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_MATCH_BODY TS_RIGHT_CURLY_BRACE
							}
							60 => {
								//NTS_STATEMENT_MATCH_BODY -> NTS_STATEMENT_MATCH_BODY TS_COMMA NTS_STATEMENT_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
							}
							61 => {
								//NTS_STATEMENT_MATCH_BODY -> NTS_STATEMENT_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
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
