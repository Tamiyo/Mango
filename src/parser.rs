use core::borrow::Borrow;
use std::collections::HashMap;
use std::ptr::null;
use std::slice::Iter;
use std::vec::Vec;
use crate::core::{ActionNode, GotoNode, LexerResult, ParserAction, PrimitiveType, TokenType};
use crate::core::TokenType::{StatementSuite, Term, StatementList};
use crate::parse_tree::{Node, NodeMango, NodeStatementSuite};

pub struct Parser { pub token_stack: Vec<LexerResult>, pub action: HashMap<(i32, TokenType), ActionNode>, pub goto: HashMap<i32, GotoNode> }

impl Default for Parser { fn default() -> Parser { Parser { token_stack: Vec::new(), action: HashMap::new(), goto: HashMap::new() } } }

impl Parser {
    fn initialize(&mut self) {
        self.goto.insert(1, GotoNode { token_type: TokenType::StatementSuite, value: 2 });
        self.goto.insert(2, GotoNode { token_type: TokenType::StatementSuiteFunction, value: 2 });
        self.goto.insert(3, GotoNode { token_type: TokenType::StatementSuiteClass, value: 2 });
        self.goto.insert(4, GotoNode { token_type: TokenType::StatementList, value: 2 });
        self.goto.insert(5, GotoNode { token_type: TokenType::StatementList, value: 1 });
        self.goto.insert(6, GotoNode { token_type: TokenType::StatementListFunction, value: 2 });
        self.goto.insert(7, GotoNode { token_type: TokenType::StatementListFunction, value: 1 });
        self.goto.insert(8, GotoNode { token_type: TokenType::StatementListClass, value: 2 });
        self.goto.insert(9, GotoNode { token_type: TokenType::StatementListClass, value: 1 });
        self.goto.insert(10, GotoNode { token_type: TokenType::Statement, value: 1 });
        self.goto.insert(11, GotoNode { token_type: TokenType::Statement, value: 1 });
        self.goto.insert(12, GotoNode { token_type: TokenType::Statement, value: 1 });
        self.goto.insert(13, GotoNode { token_type: TokenType::Statement, value: 1 });
        self.goto.insert(14, GotoNode { token_type: TokenType::StatementLimited, value: 1 });
        self.goto.insert(15, GotoNode { token_type: TokenType::StatementLimited, value: 1 });
        self.goto.insert(16, GotoNode { token_type: TokenType::StatementRestricted, value: 1 });
        self.goto.insert(17, GotoNode { token_type: TokenType::StatementSimple, value: 1 });
        self.goto.insert(18, GotoNode { token_type: TokenType::StatementSimple, value: 1 });
        self.goto.insert(19, GotoNode { token_type: TokenType::StatementSimple, value: 1 });
        self.goto.insert(20, GotoNode { token_type: TokenType::StatementComplex, value: 1 });
        self.goto.insert(21, GotoNode { token_type: TokenType::StatementFunction, value: 7 });
        self.goto.insert(22, GotoNode { token_type: TokenType::FunctionParams, value: 3 });
        self.goto.insert(23, GotoNode { token_type: TokenType::FunctionParams, value: 1 });
        self.goto.insert(24, GotoNode { token_type: TokenType::StatementClass, value: 5 });
        self.goto.insert(25, GotoNode { token_type: TokenType::StatementExpression, value: 2 });
        self.goto.insert(26, GotoNode { token_type: TokenType::StatementExpression, value: 1 });
        self.goto.insert(27, GotoNode { token_type: TokenType::StatementExpressionP, value: 2 });
        self.goto.insert(28, GotoNode { token_type: TokenType::StatementExpressionP, value: 2 });
        self.goto.insert(29, GotoNode { token_type: TokenType::StatementExpression2, value: 2 });
        self.goto.insert(30, GotoNode { token_type: TokenType::StatementExpression2, value: 1 });
        self.goto.insert(31, GotoNode { token_type: TokenType::StatementExpression2p, value: 2 });
        self.goto.insert(32, GotoNode { token_type: TokenType::StatementExpression2p, value: 2 });
        self.goto.insert(33, GotoNode { token_type: TokenType::StatementExpression2p, value: 2 });
        self.goto.insert(34, GotoNode { token_type: TokenType::StatementExpression3, value: 1 });
        self.goto.insert(35, GotoNode { token_type: TokenType::StatementExpression3, value: 3 });
        self.goto.insert(36, GotoNode { token_type: TokenType::StatementExpression3, value: 2 });
        self.goto.insert(37, GotoNode { token_type: TokenType::StatementAssignment, value: 3 });
        self.goto.insert(38, GotoNode { token_type: TokenType::StatementConditional, value: 5 });
        self.goto.insert(39, GotoNode { token_type: TokenType::StatementConditional, value: 6 });
        self.goto.insert(40, GotoNode { token_type: TokenType::StatementConditional, value: 6 });
        self.goto.insert(41, GotoNode { token_type: TokenType::StatementConditional2, value: 6 });
        self.goto.insert(42, GotoNode { token_type: TokenType::StatementConditional2, value: 6 });
        self.goto.insert(43, GotoNode { token_type: TokenType::StatementConditional3, value: 4 });
        self.goto.insert(44, GotoNode { token_type: TokenType::ConditionalExpression, value: 3 });
        self.goto.insert(45, GotoNode { token_type: TokenType::ConditionalExpression, value: 2 });
        self.goto.insert(46, GotoNode { token_type: TokenType::ComparisonOperator, value: 1 });
        self.goto.insert(47, GotoNode { token_type: TokenType::ComparisonOperator, value: 1 });
        self.goto.insert(48, GotoNode { token_type: TokenType::ComparisonOperator, value: 1 });
        self.goto.insert(49, GotoNode { token_type: TokenType::ComparisonOperator, value: 1 });
        self.goto.insert(50, GotoNode { token_type: TokenType::ComparisonOperator, value: 1 });
        self.goto.insert(51, GotoNode { token_type: TokenType::ComparisonOperator, value: 1 });
        self.goto.insert(52, GotoNode { token_type: TokenType::ComparisonOperatorUnary, value: 1 });
        self.goto.insert(53, GotoNode { token_type: TokenType::StatementLoop, value: 1 });
        self.goto.insert(54, GotoNode { token_type: TokenType::StatementLoop, value: 1 });
        self.goto.insert(55, GotoNode { token_type: TokenType::StatementLoopFor, value: 7 });
        self.goto.insert(56, GotoNode { token_type: TokenType::StatementLoopFor, value: 9 });
        self.goto.insert(57, GotoNode { token_type: TokenType::StatementLoopWhile, value: 5 });
        self.action.insert((0, TokenType::StatementSuite), ActionNode { action: ParserAction::Goto, value: 1 });
        self.action.insert((0, TokenType::StatementList), ActionNode { action: ParserAction::Goto, value: 2 });
        self.action.insert((0, TokenType::Statement), ActionNode { action: ParserAction::Goto, value: 3 });
        self.action.insert((0, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 4 });
        self.action.insert((0, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 5 });
        self.action.insert((0, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 6 });
        self.action.insert((0, TokenType::StatementClass), ActionNode { action: ParserAction::Goto, value: 7 });
        self.action.insert((0, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 8 });
        self.action.insert((0, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 9 });
        self.action.insert((0, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 10 });
        self.action.insert((0, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 11 });
        self.action.insert((0, TokenType::At), ActionNode { action: ParserAction::Shift, value: 12 });
        self.action.insert((0, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((0, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 13 });
        self.action.insert((0, TokenType::If), ActionNode { action: ParserAction::Shift, value: 19 });
        self.action.insert((0, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 20 });
        self.action.insert((0, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 21 });
        self.action.insert((0, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((0, TokenType::For), ActionNode { action: ParserAction::Shift, value: 22 });
        self.action.insert((0, TokenType::While), ActionNode { action: ParserAction::Shift, value: 23 });
        self.action.insert((0, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((0, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((0, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((1, TokenType::EndOfFile), ActionNode { action: ParserAction::Accept, value: -1 });
        self.action.insert((2, TokenType::Newline), ActionNode { action: ParserAction::Shift, value: 24 });
        self.action.insert((3, TokenType::StatementList), ActionNode { action: ParserAction::Goto, value: 25 });
        self.action.insert((3, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 5 });
        self.action.insert((3, TokenType::Statement), ActionNode { action: ParserAction::Goto, value: 3 });
        self.action.insert((3, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 4 });
        self.action.insert((3, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 5 });
        self.action.insert((3, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 6 });
        self.action.insert((3, TokenType::StatementClass), ActionNode { action: ParserAction::Goto, value: 7 });
        self.action.insert((3, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 8 });
        self.action.insert((3, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 9 });
        self.action.insert((3, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 10 });
        self.action.insert((3, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 11 });
        self.action.insert((3, TokenType::At), ActionNode { action: ParserAction::Shift, value: 12 });
        self.action.insert((3, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((3, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 13 });
        self.action.insert((3, TokenType::If), ActionNode { action: ParserAction::Shift, value: 19 });
        self.action.insert((3, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 20 });
        self.action.insert((3, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 21 });
        self.action.insert((3, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((3, TokenType::For), ActionNode { action: ParserAction::Shift, value: 22 });
        self.action.insert((3, TokenType::While), ActionNode { action: ParserAction::Shift, value: 23 });
        self.action.insert((3, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((3, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((3, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((4, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((4, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((5, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((5, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((6, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((6, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((7, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((7, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((8, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((8, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((9, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((9, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((10, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((10, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((11, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((11, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((12, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 26 });
        self.action.insert((13, TokenType::Equals), ActionNode { action: ParserAction::Shift, value: 27 });
        self.action.insert((14, TokenType::StatementExpressionP), ActionNode { action: ParserAction::Goto, value: 28 });
        self.action.insert((14, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 30 });
        self.action.insert((14, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::Add), ActionNode { action: ParserAction::Shift, value: 29 });
        self.action.insert((15, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 31 });
        self.action.insert((15, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((15, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((15, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((16, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 32 });
        self.action.insert((16, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 33 });
        self.action.insert((16, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 34 });
        self.action.insert((16, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 35 });
        self.action.insert((17, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((18, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 36 });
        self.action.insert((18, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((18, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((18, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((18, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((18, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((19, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 43 });
        self.action.insert((19, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((19, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((19, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((20, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((20, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((21, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((21, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((22, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 46 });
        self.action.insert((23, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 47 });
        self.action.insert((23, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((23, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((23, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((24, TokenType::EndOfFile), ActionNode { action: ParserAction::Reduce, value: 1 });
        self.action.insert((25, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 4 });
        self.action.insert((26, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 48 });
        self.action.insert((26, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 49 });
        self.action.insert((27, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 50 });
        self.action.insert((27, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((27, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((27, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((27, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((27, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((28, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((28, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((29, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 51 });
        self.action.insert((29, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((29, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((29, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((29, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((29, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((30, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 52 });
        self.action.insert((30, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((30, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((30, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((30, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((30, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((31, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((31, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((32, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((33, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 53 });
        self.action.insert((33, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 54 });
        self.action.insert((33, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((33, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((33, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((34, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 55 });
        self.action.insert((34, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 54 });
        self.action.insert((34, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((34, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((34, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((35, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 56 });
        self.action.insert((35, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 54 });
        self.action.insert((35, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((35, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((35, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((36, TokenType::RightParenthesis), ActionNode { action: ParserAction::Shift, value: 57 });
        self.action.insert((37, TokenType::StatementExpressionP), ActionNode { action: ParserAction::Goto, value: 58 });
        self.action.insert((37, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((37, TokenType::Add), ActionNode { action: ParserAction::Shift, value: 59 });
        self.action.insert((37, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 60 });
        self.action.insert((38, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 61 });
        self.action.insert((38, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((38, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((38, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((39, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 62 });
        self.action.insert((39, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((39, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((39, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((39, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 63 });
        self.action.insert((39, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 64 });
        self.action.insert((39, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 65 });
        self.action.insert((40, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((41, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 66 });
        self.action.insert((41, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((41, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((41, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((41, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((41, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((42, TokenType::ComparisonOperator), ActionNode { action: ParserAction::Goto, value: 67 });
        self.action.insert((42, TokenType::LessThan), ActionNode { action: ParserAction::Shift, value: 68 });
        self.action.insert((42, TokenType::LessThanEquals), ActionNode { action: ParserAction::Shift, value: 69 });
        self.action.insert((42, TokenType::GreaterThan), ActionNode { action: ParserAction::Shift, value: 70 });
        self.action.insert((42, TokenType::GreaterThanEquals), ActionNode { action: ParserAction::Shift, value: 71 });
        self.action.insert((42, TokenType::DoubleEquals), ActionNode { action: ParserAction::Shift, value: 72 });
        self.action.insert((42, TokenType::TripleEquals), ActionNode { action: ParserAction::Shift, value: 73 });
        self.action.insert((43, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 74 });
        self.action.insert((44, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 75 });
        self.action.insert((45, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 52 });
        self.action.insert((46, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 76 });
        self.action.insert((47, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 77 });
        self.action.insert((48, TokenType::FunctionParams), ActionNode { action: ParserAction::Goto, value: 79 });
        self.action.insert((48, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 78 });
        self.action.insert((49, TokenType::StatementSuiteClass), ActionNode { action: ParserAction::Goto, value: 80 });
        self.action.insert((49, TokenType::StatementListClass), ActionNode { action: ParserAction::Goto, value: 81 });
        self.action.insert((49, TokenType::StatementRestricted), ActionNode { action: ParserAction::Goto, value: 82 });
        self.action.insert((49, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 83 });
        self.action.insert((49, TokenType::At), ActionNode { action: ParserAction::Shift, value: 84 });
        self.action.insert((50, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((50, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((51, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((51, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((52, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((52, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((53, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((53, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((54, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 32 });
        self.action.insert((54, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((54, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 33 });
        self.action.insert((54, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 34 });
        self.action.insert((54, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 35 });
        self.action.insert((55, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((55, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((56, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((56, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((57, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((57, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((58, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((59, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 85 });
        self.action.insert((59, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((59, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((59, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((59, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((59, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((60, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 86 });
        self.action.insert((60, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((60, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((60, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((60, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((60, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((61, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((61, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((61, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((61, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((61, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((61, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((62, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((62, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((62, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((63, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 87 });
        self.action.insert((63, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 88 });
        self.action.insert((63, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((63, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((63, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((64, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 89 });
        self.action.insert((64, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 88 });
        self.action.insert((64, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((64, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((64, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((65, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 90 });
        self.action.insert((65, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 88 });
        self.action.insert((65, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((65, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((65, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((66, TokenType::RightParenthesis), ActionNode { action: ParserAction::Shift, value: 91 });
        self.action.insert((67, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 92 });
        self.action.insert((68, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 46 });
        self.action.insert((69, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 47 });
        self.action.insert((70, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 48 });
        self.action.insert((71, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 49 });
        self.action.insert((72, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 50 });
        self.action.insert((73, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 51 });
        self.action.insert((74, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 93 });
        self.action.insert((74, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((74, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((74, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((74, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((74, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((74, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((74, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((74, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((74, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((74, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((74, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((74, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((74, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((74, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((74, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((74, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((74, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((74, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((74, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((75, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 45 });
        self.action.insert((76, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((77, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 114 });
        self.action.insert((77, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((77, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((77, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((77, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((77, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((77, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((77, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((77, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((77, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((77, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((77, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((77, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((77, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((77, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((77, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((77, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((77, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((77, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((77, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((78, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 23 });
        self.action.insert((78, TokenType::Comma), ActionNode { action: ParserAction::Reduce, value: 23 });
        self.action.insert((79, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 115 });
        self.action.insert((79, TokenType::Comma), ActionNode { action: ParserAction::Shift, value: 116 });
        self.action.insert((80, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 117 });
        self.action.insert((81, TokenType::Newline), ActionNode { action: ParserAction::Shift, value: 118 });
        self.action.insert((82, TokenType::StatementListClass), ActionNode { action: ParserAction::Goto, value: 119 });
        self.action.insert((82, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 9 });
        self.action.insert((82, TokenType::StatementRestricted), ActionNode { action: ParserAction::Goto, value: 82 });
        self.action.insert((82, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 83 });
        self.action.insert((82, TokenType::At), ActionNode { action: ParserAction::Shift, value: 84 });
        self.action.insert((83, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 16 });
        self.action.insert((83, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 16 });
        self.action.insert((84, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 120 });
        self.action.insert((85, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((86, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((87, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((87, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((87, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((88, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 62 });
        self.action.insert((88, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((88, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((88, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((88, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 63 });
        self.action.insert((88, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 64 });
        self.action.insert((88, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 65 });
        self.action.insert((89, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((89, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((89, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((90, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((90, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((90, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((91, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((91, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((91, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((91, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((91, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((91, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((92, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 44 });
        self.action.insert((93, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 121 });
        self.action.insert((94, TokenType::Newline), ActionNode { action: ParserAction::Shift, value: 122 });
        self.action.insert((95, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 123 });
        self.action.insert((95, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 7 });
        self.action.insert((95, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((95, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((95, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((95, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((95, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((95, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((95, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((95, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((95, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((95, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((95, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((95, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((95, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((95, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((95, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((95, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((95, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((95, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((96, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((96, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((97, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((98, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((99, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((100, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((101, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::Equals), ActionNode { action: ParserAction::Shift, value: 124 });
        self.action.insert((103, TokenType::StatementExpressionP), ActionNode { action: ParserAction::Goto, value: 125 });
        self.action.insert((103, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 127 });
        self.action.insert((103, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((103, TokenType::Add), ActionNode { action: ParserAction::Shift, value: 126 });
        self.action.insert((104, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 128 });
        self.action.insert((104, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((104, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((104, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((105, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 129 });
        self.action.insert((105, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((105, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 130 });
        self.action.insert((105, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 131 });
        self.action.insert((105, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 132 });
        self.action.insert((106, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((106, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 133 });
        self.action.insert((107, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((107, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((107, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((107, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((107, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((108, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 134 });
        self.action.insert((108, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((108, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((108, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((109, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((109, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((110, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 135 });
        self.action.insert((112, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 136 });
        self.action.insert((112, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((112, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((112, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((113, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 137 });
        self.action.insert((113, TokenType::Ellipsis), ActionNode { action: ParserAction::Shift, value: 138 });
        self.action.insert((114, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 139 });
        self.action.insert((115, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 140 });
        self.action.insert((115, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((115, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((115, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((115, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((115, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((115, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((115, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((115, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((115, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((115, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((115, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((115, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((115, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((115, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((115, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((115, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((115, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((115, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((115, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((116, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 141 });
        self.action.insert((117, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((117, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((118, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 3 });
        self.action.insert((119, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 8 });
        self.action.insert((120, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 142 });
        self.action.insert((121, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::StatementConditional2), ActionNode { action: ParserAction::Goto, value: 143 });
        self.action.insert((121, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 144 });
        self.action.insert((121, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((121, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 145 });
        self.action.insert((121, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 146 });
        self.action.insert((122, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 2 });
        self.action.insert((123, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 6 });
        self.action.insert((124, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 147 });
        self.action.insert((124, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((124, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((124, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((124, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((124, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((125, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((125, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 148 });
        self.action.insert((126, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((126, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((126, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((126, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((126, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((127, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 149 });
        self.action.insert((127, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((127, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((127, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((127, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((127, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((128, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((128, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((129, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 150 });
        self.action.insert((130, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 151 });
        self.action.insert((130, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((130, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((130, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((131, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 152 });
        self.action.insert((131, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 151 });
        self.action.insert((131, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((131, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((131, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((132, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 153 });
        self.action.insert((132, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 151 });
        self.action.insert((132, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((132, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((132, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((133, TokenType::RightParenthesis), ActionNode { action: ParserAction::Shift, value: 154 });
        self.action.insert((134, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 155 });
        self.action.insert((135, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 156 });
        self.action.insert((136, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 157 });
        self.action.insert((137, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 158 });
        self.action.insert((137, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((137, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((137, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((137, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((137, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((137, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((137, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((137, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((137, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((137, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((137, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((137, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((137, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((137, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((137, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((137, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((137, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((137, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((137, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((138, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 159 });
        self.action.insert((139, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((139, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((140, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 160 });
        self.action.insert((141, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 22 });
        self.action.insert((141, TokenType::Comma), ActionNode { action: ParserAction::Reduce, value: 22 });
        self.action.insert((142, TokenType::FunctionParams), ActionNode { action: ParserAction::Goto, value: 161 });
        self.action.insert((142, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 78 });
        self.action.insert((143, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((143, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 162 });
        self.action.insert((144, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((144, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((145, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 163 });
        self.action.insert((145, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((145, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((145, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((146, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 164 });
        self.action.insert((147, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((147, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((148, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((149, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((150, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 129 });
        self.action.insert((151, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((151, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 130 });
        self.action.insert((151, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 131 });
        self.action.insert((151, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 132 });
        self.action.insert((152, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((152, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((153, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((154, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 165 });
        self.action.insert((155, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((155, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((155, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((155, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((155, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((155, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((155, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((155, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((155, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((155, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((155, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((155, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((155, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((155, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((155, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((155, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((155, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((155, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((155, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((156, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 166 });
        self.action.insert((157, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 167 });
        self.action.insert((157, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((157, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((157, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((157, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((157, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((157, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((157, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((157, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((157, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((157, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((157, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((157, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((157, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((157, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((157, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((157, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((157, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((157, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((157, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((158, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 168 });
        self.action.insert((159, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 169 });
        self.action.insert((160, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((160, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((161, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 170 });
        self.action.insert((161, TokenType::Comma), ActionNode { action: ParserAction::Shift, value: 116 });
        self.action.insert((162, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 171 });
        self.action.insert((162, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((162, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((162, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((163, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 172 });
        self.action.insert((164, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 173 });
        self.action.insert((164, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((164, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((164, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((164, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((164, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((164, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((164, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((164, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((164, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((164, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((164, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((164, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((164, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((164, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((164, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((164, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((164, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((164, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((164, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((165, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 174 });
        self.action.insert((166, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 175 });
        self.action.insert((166, TokenType::Ellipsis), ActionNode { action: ParserAction::Shift, value: 176 });
        self.action.insert((167, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 177 });
        self.action.insert((168, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((168, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((169, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 178 });
        self.action.insert((169, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((169, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((169, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((169, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((169, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((169, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((169, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((169, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((169, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((169, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((169, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((169, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((169, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((169, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((169, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((169, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((169, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((169, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((169, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((170, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 179 });
        self.action.insert((170, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((170, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((170, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((170, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((170, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((170, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((170, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((170, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((170, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((170, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((170, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((170, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((170, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((170, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((170, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((170, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((170, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((170, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((170, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((171, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 180 });
        self.action.insert((172, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 181 });
        self.action.insert((172, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((172, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((172, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((172, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((172, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((172, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((172, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((172, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((172, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((172, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((172, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((172, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((172, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((172, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((172, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((172, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((172, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((172, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((172, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((173, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 182 });
        self.action.insert((174, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::StatementConditional2), ActionNode { action: ParserAction::Goto, value: 183 });
        self.action.insert((174, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 184 });
        self.action.insert((174, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((174, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 185 });
        self.action.insert((174, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 186 });
        self.action.insert((175, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 187 });
        self.action.insert((175, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((175, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((175, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((175, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((175, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((175, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((175, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((175, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((175, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((175, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((175, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((175, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((175, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((175, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((175, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((175, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((175, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((175, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((175, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((176, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 188 });
        self.action.insert((177, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((177, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 189 });
        self.action.insert((179, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 190 });
        self.action.insert((180, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 191 });
        self.action.insert((180, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((180, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((180, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((180, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((180, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((180, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((180, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((180, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((180, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((180, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((180, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((180, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((180, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((180, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((180, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((180, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((180, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((180, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((180, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((181, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 192 });
        self.action.insert((182, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((182, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((183, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((183, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 193 });
        self.action.insert((184, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((184, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 194 });
        self.action.insert((185, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((185, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((185, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((186, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 195 });
        self.action.insert((187, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 196 });
        self.action.insert((188, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 197 });
        self.action.insert((189, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((189, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((190, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((190, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((191, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 198 });
        self.action.insert((192, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 199 });
        self.action.insert((192, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 200 });
        self.action.insert((193, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 201 });
        self.action.insert((193, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((193, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((193, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((194, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 202 });
        self.action.insert((195, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 203 });
        self.action.insert((195, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((195, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((195, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((195, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((195, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((195, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((195, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((195, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((195, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((195, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((195, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((195, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((195, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((195, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((195, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((195, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((195, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((195, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((195, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((196, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((196, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 204 });
        self.action.insert((197, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((197, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((197, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((197, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((197, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((197, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((197, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((197, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((197, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((197, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((197, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((197, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((197, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((197, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((197, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((197, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((197, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((197, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((197, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((198, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((198, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((199, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((199, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((200, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 205 });
        self.action.insert((201, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 206 });
        self.action.insert((202, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 207 });
        self.action.insert((202, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((202, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((202, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((202, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((202, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((202, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((202, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((202, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((202, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((202, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((202, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((202, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((202, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((202, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((202, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((202, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((202, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((202, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((202, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((203, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 208 });
        self.action.insert((204, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 209 });
        self.action.insert((205, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 210 });
        self.action.insert((205, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((205, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((205, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((205, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((205, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((205, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((205, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((205, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((205, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((205, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((205, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((205, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((205, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((205, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((205, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((205, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((205, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((205, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((205, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((206, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 211 });
        self.action.insert((206, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((206, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((206, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((206, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((206, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((206, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((206, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((206, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((206, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((206, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((206, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((206, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((206, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((206, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((206, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((206, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((206, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((206, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((206, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((207, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 212 });
        self.action.insert((208, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((208, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((209, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 213 });
        self.action.insert((211, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 214 });
        self.action.insert((212, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 215 });
        self.action.insert((212, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 216 });
        self.action.insert((213, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((213, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((214, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((214, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((215, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 217 });
        self.action.insert((217, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 218 });
        self.action.insert((217, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((217, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((217, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((217, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((217, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((217, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((217, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((217, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((217, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 103 });
        self.action.insert((217, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 102 });
        self.action.insert((217, TokenType::If), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((217, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 109 });
        self.action.insert((217, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((217, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 105 });
        self.action.insert((217, TokenType::For), ActionNode { action: ParserAction::Shift, value: 111 });
        self.action.insert((217, TokenType::While), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((217, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 106 });
        self.action.insert((217, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((217, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 104 });
        self.action.insert((218, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 219 });
        self.action.insert((219, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((219, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 43 });
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
                                let statement_list = node_stack.pop().unwrap();
                                let node = NodeStatementSuite { statement_list: statement_list };
                                node_stack.push(Box::new(node));
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
                                //NTS_STATEMENT_FUNCTION -> TS_AT TS_IDENTIFIER TS_COLON NTS_FUNCTION_PARAMS TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                            }
                            22 => {
                                //NTS_FUNCTION_PARAMS -> NTS_FUNCTION_PARAMS TS_COMMA TS_IDENTIFIER
                            }
                            23 => {
                                //NTS_FUNCTION_PARAMS -> TS_IDENTIFIER
                            }
                            24 => {
                                //NTS_STATEMENT_CLASS -> TS_AT TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_CLASS TS_RIGHT_CURLY_BRACE
                            }
                            25 => {
                                //NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2 NTS_STATEMENT_EXPRESSION_P
                            }
                            26 => {
                                //NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2
                            }
                            27 => {
                                //NTS_STATEMENT_EXPRESSION_P -> TS_ADD NTS_STATEMENT_EXPRESSION
                            }
                            28 => {
                                //NTS_STATEMENT_EXPRESSION_P -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION
                            }
                            29 => {
                                //NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3 NTS_STATEMENT_EXPRESSION_2P
                            }
                            30 => {
                                //NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3
                            }
                            31 => {
                                //NTS_STATEMENT_EXPRESSION_2P -> TS_MULTIPLY NTS_STATEMENT_EXPRESSION_2
                            }
                            32 => {
                                //NTS_STATEMENT_EXPRESSION_2P -> TS_DIVIDE NTS_STATEMENT_EXPRESSION_2
                            }
                            33 => {
                                //NTS_STATEMENT_EXPRESSION_2P -> TS_MODULO NTS_STATEMENT_EXPRESSION_2
                            }
                            34 => {
                                //NTS_STATEMENT_EXPRESSION_3 -> TS_TERM
                            }
                            35 => {
                                //NTS_STATEMENT_EXPRESSION_3 -> TS_LEFT_PARENTHESIS NTS_STATEMENT_EXPRESSION TS_RIGHT_PARENTHESIS
                            }
                            36 => {
                                //NTS_STATEMENT_EXPRESSION_3 -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION_3
                            }
                            37 => {
                                //NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS NTS_STATEMENT_EXPRESSION
                            }
                            38 => {
                                //NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                            }
                            39 => {
                                //NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_2
                            }
                            40 => {
                                //NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
                            }
                            41 => {
                                //NTS_STATEMENT_CONDITIONAL_2 -> NTS_STATEMENT_CONDITIONAL_2 TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                            }
                            42 => {
                                //NTS_STATEMENT_CONDITIONAL_2 -> TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
                            }
                            43 => {
                                //NTS_STATEMENT_CONDITIONAL_3 -> TS_ELSE TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                            }
                            44 => {
                                //NTS_CONDITIONAL_EXPRESSION -> TS_TERM NTS_COMPARISON_OPERATOR TS_TERM
                            }
                            45 => {
                                //NTS_CONDITIONAL_EXPRESSION -> NTS_COMPARISON_OPERATOR_UNARY TS_TERM
                            }
                            46 => {
                                //NTS_COMPARISON_OPERATOR -> TS_LESS_THAN
                            }
                            47 => {
                                //NTS_COMPARISON_OPERATOR -> TS_LESS_THAN_EQUALS
                            }
                            48 => {
                                //NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN
                            }
                            49 => {
                                //NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN_EQUALS
                            }
                            50 => {
                                //NTS_COMPARISON_OPERATOR -> TS_DOUBLE_EQUALS
                            }
                            51 => {
                                //NTS_COMPARISON_OPERATOR -> TS_TRIPLE_EQUALS
                            }
                            52 => {
                                //NTS_COMPARISON_OPERATOR_UNARY -> TS_NOT
                            }
                            53 => {
                                //NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_FOR
                            }
                            54 => {
                                //NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_WHILE
                            }
                            55 => {
                                //NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON TS_TERM TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                            }
                            56 => {
                                //NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON TS_TERM TS_ELLIPSIS TS_TERM TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                            }
                            57 => {
                                //NTS_STATEMENT_LOOP_WHILE -> TS_WHILE NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
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
