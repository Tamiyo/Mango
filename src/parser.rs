use core::borrow::Borrow;
use std::collections::HashMap;
use std::ptr::null;
use std::slice::Iter;
use std::vec::Vec;

use crate::core::{ActionNode, GotoNode, LexerResult, ParserAction, PrimitiveType, TokenType};
use crate::core::TokenType::{StatementList, StatementSuite, Term};
use crate::parse_tree::{Node, NodeConditionalExpression, NodeConditionalExpressionUnary, NodeFunctionParams, NodeFunctionParamsRecursive, NodeIdentifier, NodeMango, NodeStatement, NodeStatementAssignment, NodeStatementClass, NodeStatementComplex, NodeStatementConditional, NodeStatementConditional2, NodeStatementConditional2Recursive, NodeStatementConditional3, NodeStatementConditionalW2, NodeStatementConditionalW3, NodeStatementExpression, NodeStatementExpression2, NodeStatementExpression2p, NodeStatementExpression2Recursive, NodeStatementExpression3, NodeStatementExpression3Negation, NodeStatementExpression3Paren, NodeStatementExpressionP, NodeStatementExpressionRecursive, NodeStatementFunction, NodeStatementLimited, NodeStatementList, NodeStatementListClass, NodeStatementListClassRecursive, NodeStatementListFunction, NodeStatementListFunctionRecursive, NodeStatementListRecursive, NodeStatementRestricted, NodeStatementSimple, NodeStatementSuite, NodeStatementSuiteClass, NodeTerm};

pub struct Parser { pub token_stack: Vec<LexerResult>, pub action: HashMap<(i32, TokenType), ActionNode>, pub goto: HashMap<i32, GotoNode> }

impl Default for Parser { fn default() -> Parser { Parser { token_stack: Vec::new(), action: HashMap::new(), goto: HashMap::new() } } }

impl Parser {
    fn initialize(&mut self) {
        self.goto.insert(1, GotoNode { token_type: TokenType::StatementSuite, value: 2 });
        self.goto.insert(2, GotoNode { token_type: TokenType::StatementSuiteFunction, value: 2 });
        self.goto.insert(3, GotoNode { token_type: TokenType::StatementSuiteClass, value: 2 });
        self.goto.insert(4, GotoNode { token_type: TokenType::StatementList, value: 3 });
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
        self.action.insert((3, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 5 });
        self.action.insert((4, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 10 });
        self.action.insert((5, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 11 });
        self.action.insert((6, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 12 });
        self.action.insert((7, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 13 });
        self.action.insert((8, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((9, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((10, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((11, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((12, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 26 });
        self.action.insert((13, TokenType::Equals), ActionNode { action: ParserAction::Shift, value: 27 });
        self.action.insert((14, TokenType::StatementExpressionP), ActionNode { action: ParserAction::Goto, value: 28 });
        self.action.insert((14, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((14, TokenType::Add), ActionNode { action: ParserAction::Shift, value: 29 });
        self.action.insert((14, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 30 });
        self.action.insert((15, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 31 });
        self.action.insert((15, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((15, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((15, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((16, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 32 });
        self.action.insert((16, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((16, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 33 });
        self.action.insert((16, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 34 });
        self.action.insert((16, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 35 });
        self.action.insert((17, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((17, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 34 });
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
        self.action.insert((20, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((21, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((22, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 46 });
        self.action.insert((23, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 47 });
        self.action.insert((23, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((23, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((23, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((24, TokenType::EndOfFile), ActionNode { action: ParserAction::Reduce, value: 1 });
        self.action.insert((25, TokenType::StatementList), ActionNode { action: ParserAction::Goto, value: 48 });
        self.action.insert((25, TokenType::Statement), ActionNode { action: ParserAction::Goto, value: 3 });
        self.action.insert((25, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 4 });
        self.action.insert((25, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 5 });
        self.action.insert((25, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 6 });
        self.action.insert((25, TokenType::StatementClass), ActionNode { action: ParserAction::Goto, value: 7 });
        self.action.insert((25, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 8 });
        self.action.insert((25, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 9 });
        self.action.insert((25, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 10 });
        self.action.insert((25, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 11 });
        self.action.insert((25, TokenType::At), ActionNode { action: ParserAction::Shift, value: 12 });
        self.action.insert((25, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((25, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 13 });
        self.action.insert((25, TokenType::If), ActionNode { action: ParserAction::Shift, value: 19 });
        self.action.insert((25, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 20 });
        self.action.insert((25, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 21 });
        self.action.insert((25, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((25, TokenType::For), ActionNode { action: ParserAction::Shift, value: 22 });
        self.action.insert((25, TokenType::While), ActionNode { action: ParserAction::Shift, value: 23 });
        self.action.insert((25, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((25, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((25, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((26, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 49 });
        self.action.insert((26, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 50 });
        self.action.insert((27, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 51 });
        self.action.insert((27, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((27, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((27, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((27, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((27, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((28, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((29, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 52 });
        self.action.insert((29, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 14 });
        self.action.insert((29, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 16 });
        self.action.insert((29, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((29, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((29, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((30, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 53 });
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
        self.action.insert((31, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((32, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((32, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((33, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 54 });
        self.action.insert((33, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 55 });
        self.action.insert((33, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((33, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((33, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((34, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 56 });
        self.action.insert((34, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 55 });
        self.action.insert((34, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((34, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((34, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((35, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 57 });
        self.action.insert((35, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 55 });
        self.action.insert((35, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 17 });
        self.action.insert((35, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 18 });
        self.action.insert((35, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 15 });
        self.action.insert((36, TokenType::RightParenthesis), ActionNode { action: ParserAction::Shift, value: 58 });
        self.action.insert((37, TokenType::StatementExpressionP), ActionNode { action: ParserAction::Goto, value: 59 });
        self.action.insert((37, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((37, TokenType::Add), ActionNode { action: ParserAction::Shift, value: 60 });
        self.action.insert((37, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 61 });
        self.action.insert((38, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 62 });
        self.action.insert((38, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((38, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((38, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((39, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 63 });
        self.action.insert((39, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((39, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((39, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((39, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 64 });
        self.action.insert((39, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 65 });
        self.action.insert((39, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 66 });
        self.action.insert((40, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((40, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((41, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 67 });
        self.action.insert((41, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((41, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((41, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((41, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((41, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((42, TokenType::ComparisonOperator), ActionNode { action: ParserAction::Goto, value: 68 });
        self.action.insert((42, TokenType::LessThan), ActionNode { action: ParserAction::Shift, value: 69 });
        self.action.insert((42, TokenType::LessThanEquals), ActionNode { action: ParserAction::Shift, value: 70 });
        self.action.insert((42, TokenType::GreaterThan), ActionNode { action: ParserAction::Shift, value: 71 });
        self.action.insert((42, TokenType::GreaterThanEquals), ActionNode { action: ParserAction::Shift, value: 72 });
        self.action.insert((42, TokenType::DoubleEquals), ActionNode { action: ParserAction::Shift, value: 73 });
        self.action.insert((42, TokenType::TripleEquals), ActionNode { action: ParserAction::Shift, value: 74 });
        self.action.insert((43, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 75 });
        self.action.insert((44, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 76 });
        self.action.insert((45, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 52 });
        self.action.insert((46, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 77 });
        self.action.insert((47, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 78 });
        self.action.insert((48, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 4 });
        self.action.insert((49, TokenType::FunctionParams), ActionNode { action: ParserAction::Goto, value: 80 });
        self.action.insert((49, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 79 });
        self.action.insert((50, TokenType::StatementSuiteClass), ActionNode { action: ParserAction::Goto, value: 81 });
        self.action.insert((50, TokenType::StatementListClass), ActionNode { action: ParserAction::Goto, value: 82 });
        self.action.insert((50, TokenType::StatementRestricted), ActionNode { action: ParserAction::Goto, value: 83 });
        self.action.insert((50, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 84 });
        self.action.insert((50, TokenType::At), ActionNode { action: ParserAction::Shift, value: 85 });
        self.action.insert((51, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((52, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((53, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((54, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((54, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((54, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((55, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 32 });
        self.action.insert((55, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((55, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((55, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((55, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 33 });
        self.action.insert((55, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 34 });
        self.action.insert((55, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 35 });
        self.action.insert((56, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((56, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((56, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((57, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((57, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((57, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((58, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((58, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((58, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((58, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((58, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((58, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((59, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((60, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 86 });
        self.action.insert((60, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((60, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((60, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((60, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((60, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((61, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 87 });
        self.action.insert((61, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((61, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((61, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((61, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((61, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((62, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((62, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((62, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((62, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((62, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((62, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((63, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((63, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((63, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((64, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 88 });
        self.action.insert((64, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 89 });
        self.action.insert((64, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((64, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((64, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((65, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 90 });
        self.action.insert((65, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 89 });
        self.action.insert((65, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((65, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((65, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((66, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 91 });
        self.action.insert((66, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 89 });
        self.action.insert((66, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((66, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((66, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((67, TokenType::RightParenthesis), ActionNode { action: ParserAction::Shift, value: 92 });
        self.action.insert((68, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 93 });
        self.action.insert((69, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 46 });
        self.action.insert((70, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 47 });
        self.action.insert((71, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 48 });
        self.action.insert((72, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 49 });
        self.action.insert((73, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 50 });
        self.action.insert((74, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 51 });
        self.action.insert((75, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 94 });
        self.action.insert((75, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((75, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((75, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((75, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((75, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((75, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((75, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((75, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((75, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((75, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((75, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((75, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((75, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((75, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((75, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((75, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((75, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((75, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((75, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((76, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 45 });
        self.action.insert((77, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 114 });
        self.action.insert((78, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 115 });
        self.action.insert((78, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((78, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((78, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((78, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((78, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((78, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((78, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((78, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((78, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((78, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((78, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((78, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((78, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((78, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((78, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((78, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((78, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((78, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((78, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((79, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 23 });
        self.action.insert((79, TokenType::Comma), ActionNode { action: ParserAction::Reduce, value: 23 });
        self.action.insert((80, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 116 });
        self.action.insert((80, TokenType::Comma), ActionNode { action: ParserAction::Shift, value: 117 });
        self.action.insert((81, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 118 });
        self.action.insert((82, TokenType::Newline), ActionNode { action: ParserAction::Shift, value: 119 });
        self.action.insert((83, TokenType::StatementListClass), ActionNode { action: ParserAction::Goto, value: 120 });
        self.action.insert((83, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 9 });
        self.action.insert((83, TokenType::StatementRestricted), ActionNode { action: ParserAction::Goto, value: 83 });
        self.action.insert((83, TokenType::StatementFunction), ActionNode { action: ParserAction::Goto, value: 84 });
        self.action.insert((83, TokenType::At), ActionNode { action: ParserAction::Shift, value: 85 });
        self.action.insert((84, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 16 });
        self.action.insert((84, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 16 });
        self.action.insert((85, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 121 });
        self.action.insert((86, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((87, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((88, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((88, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((88, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((89, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 63 });
        self.action.insert((89, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((89, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((89, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((89, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 64 });
        self.action.insert((89, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 65 });
        self.action.insert((89, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 66 });
        self.action.insert((90, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((90, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((90, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((91, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((91, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((91, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((92, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((92, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((92, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((92, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((92, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((92, TokenType::RightParenthesis), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((93, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 44 });
        self.action.insert((94, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 122 });
        self.action.insert((95, TokenType::Newline), ActionNode { action: ParserAction::Shift, value: 123 });
        self.action.insert((96, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 124 });
        self.action.insert((96, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 7 });
        self.action.insert((96, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((96, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((96, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((96, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((96, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((96, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((96, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((96, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((96, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((96, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((96, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((96, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((96, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((96, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((96, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((96, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((96, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((96, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((97, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((97, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 14 });
        self.action.insert((98, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((98, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 15 });
        self.action.insert((99, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((99, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 17 });
        self.action.insert((100, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((100, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 18 });
        self.action.insert((101, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((101, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 19 });
        self.action.insert((102, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((102, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 20 });
        self.action.insert((103, TokenType::Equals), ActionNode { action: ParserAction::Shift, value: 125 });
        self.action.insert((104, TokenType::StatementExpressionP), ActionNode { action: ParserAction::Goto, value: 126 });
        self.action.insert((104, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 128 });
        self.action.insert((104, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 26 });
        self.action.insert((104, TokenType::Add), ActionNode { action: ParserAction::Shift, value: 127 });
        self.action.insert((105, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 129 });
        self.action.insert((105, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((105, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((105, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((106, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 130 });
        self.action.insert((106, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((106, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 131 });
        self.action.insert((106, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 132 });
        self.action.insert((106, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 133 });
        self.action.insert((107, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((107, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 34 });
        self.action.insert((108, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 134 });
        self.action.insert((108, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 37 });
        self.action.insert((108, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 39 });
        self.action.insert((108, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 40 });
        self.action.insert((108, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 41 });
        self.action.insert((108, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 38 });
        self.action.insert((109, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 135 });
        self.action.insert((109, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((109, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((109, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((110, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((110, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 53 });
        self.action.insert((111, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((111, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 54 });
        self.action.insert((112, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 136 });
        self.action.insert((113, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 137 });
        self.action.insert((113, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((113, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((113, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((114, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 138 });
        self.action.insert((114, TokenType::Ellipsis), ActionNode { action: ParserAction::Shift, value: 139 });
        self.action.insert((115, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 140 });
        self.action.insert((116, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 141 });
        self.action.insert((116, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((116, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((116, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((116, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((116, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((116, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((116, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((116, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((116, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((116, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((116, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((116, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((116, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((116, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((116, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((116, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((116, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((116, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((116, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((117, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 142 });
        self.action.insert((118, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 24 });
        self.action.insert((119, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 3 });
        self.action.insert((120, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 8 });
        self.action.insert((121, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 143 });
        self.action.insert((122, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((122, TokenType::StatementConditional2), ActionNode { action: ParserAction::Goto, value: 144 });
        self.action.insert((122, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 145 });
        self.action.insert((122, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 146 });
        self.action.insert((122, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 147 });
        self.action.insert((123, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 2 });
        self.action.insert((124, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 6 });
        self.action.insert((125, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 148 });
        self.action.insert((125, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((125, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((125, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((125, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((125, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((126, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((126, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 25 });
        self.action.insert((127, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 149 });
        self.action.insert((127, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((127, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((127, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((127, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((127, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((128, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 150 });
        self.action.insert((128, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((128, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((128, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((128, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((128, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((129, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((129, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 36 });
        self.action.insert((130, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((130, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 29 });
        self.action.insert((131, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 151 });
        self.action.insert((131, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 152 });
        self.action.insert((131, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((131, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((131, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((132, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 153 });
        self.action.insert((132, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 152 });
        self.action.insert((132, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((132, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((132, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((133, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 154 });
        self.action.insert((133, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 152 });
        self.action.insert((133, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((133, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((133, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((134, TokenType::RightParenthesis), ActionNode { action: ParserAction::Shift, value: 155 });
        self.action.insert((135, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 156 });
        self.action.insert((136, TokenType::Colon), ActionNode { action: ParserAction::Shift, value: 157 });
        self.action.insert((137, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 158 });
        self.action.insert((138, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 159 });
        self.action.insert((138, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((138, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((138, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((138, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((138, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((138, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((138, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((138, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((138, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((138, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((138, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((138, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((138, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((138, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((138, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((138, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((138, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((138, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((138, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((139, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 160 });
        self.action.insert((140, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((141, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 161 });
        self.action.insert((142, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Reduce, value: 22 });
        self.action.insert((142, TokenType::Comma), ActionNode { action: ParserAction::Reduce, value: 22 });
        self.action.insert((143, TokenType::FunctionParams), ActionNode { action: ParserAction::Goto, value: 162 });
        self.action.insert((143, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 79 });
        self.action.insert((144, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((144, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 163 });
        self.action.insert((145, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((146, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 164 });
        self.action.insert((146, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((146, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((146, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((147, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 165 });
        self.action.insert((148, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((148, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 37 });
        self.action.insert((149, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((149, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 27 });
        self.action.insert((150, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((150, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 28 });
        self.action.insert((151, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((151, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 31 });
        self.action.insert((152, TokenType::StatementExpression2p), ActionNode { action: ParserAction::Goto, value: 130 });
        self.action.insert((152, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 30 });
        self.action.insert((152, TokenType::Multiply), ActionNode { action: ParserAction::Shift, value: 131 });
        self.action.insert((152, TokenType::Divide), ActionNode { action: ParserAction::Shift, value: 132 });
        self.action.insert((152, TokenType::Modulo), ActionNode { action: ParserAction::Shift, value: 133 });
        self.action.insert((153, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((153, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 32 });
        self.action.insert((154, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((154, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 33 });
        self.action.insert((155, TokenType::Modulo), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Divide), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Multiply), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Add), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((155, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 35 });
        self.action.insert((156, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 166 });
        self.action.insert((156, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((156, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((156, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((156, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((156, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((156, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((156, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((156, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((156, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((156, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((156, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((156, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((156, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((156, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((156, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((156, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((156, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((156, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((156, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((157, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 167 });
        self.action.insert((158, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 168 });
        self.action.insert((158, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((158, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((158, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((158, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((158, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((158, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((158, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((158, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((158, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((158, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((158, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((158, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((158, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((158, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((158, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((158, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((158, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((158, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((158, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((159, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 169 });
        self.action.insert((160, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 170 });
        self.action.insert((161, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((162, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 171 });
        self.action.insert((162, TokenType::Comma), ActionNode { action: ParserAction::Shift, value: 117 });
        self.action.insert((163, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 172 });
        self.action.insert((163, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((163, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((163, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((164, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 173 });
        self.action.insert((165, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 174 });
        self.action.insert((165, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((165, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((165, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((165, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((165, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((165, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((165, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((165, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((165, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((165, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((165, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((165, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((165, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((165, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((165, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((165, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((165, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((165, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((165, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((166, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 175 });
        self.action.insert((167, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 176 });
        self.action.insert((167, TokenType::Ellipsis), ActionNode { action: ParserAction::Shift, value: 177 });
        self.action.insert((168, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 178 });
        self.action.insert((169, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((170, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 179 });
        self.action.insert((170, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((170, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((170, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((170, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((170, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((170, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((170, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((170, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((170, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((170, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((170, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((170, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((170, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((170, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((170, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((170, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((170, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((170, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((170, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((171, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 180 });
        self.action.insert((171, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((171, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((171, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((171, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((171, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((171, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((171, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((171, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((171, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((171, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((171, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((171, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((171, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((171, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((171, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((171, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((171, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((171, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((171, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((172, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 181 });
        self.action.insert((173, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 182 });
        self.action.insert((173, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((173, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((173, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((173, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((173, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((173, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((173, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((173, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((173, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((173, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((173, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((173, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((173, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((173, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((173, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((173, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((173, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((173, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((173, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((174, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 183 });
        self.action.insert((175, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::StatementConditional2), ActionNode { action: ParserAction::Goto, value: 184 });
        self.action.insert((175, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 185 });
        self.action.insert((175, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 38 });
        self.action.insert((175, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 186 });
        self.action.insert((175, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 187 });
        self.action.insert((176, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 188 });
        self.action.insert((176, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((176, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((176, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((176, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((176, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((176, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((176, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((176, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((176, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((176, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((176, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((176, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((176, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((176, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((176, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((176, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((176, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((176, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((176, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((177, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 189 });
        self.action.insert((178, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((178, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 57 });
        self.action.insert((179, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 190 });
        self.action.insert((180, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 191 });
        self.action.insert((181, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 192 });
        self.action.insert((181, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((181, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((181, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((181, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((181, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((181, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((181, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((181, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((181, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((181, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((181, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((181, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((181, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((181, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((181, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((181, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((181, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((181, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((181, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((182, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 193 });
        self.action.insert((183, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((184, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 39 });
        self.action.insert((184, TokenType::Elif), ActionNode { action: ParserAction::Shift, value: 194 });
        self.action.insert((185, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((185, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 40 });
        self.action.insert((186, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 195 });
        self.action.insert((186, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((186, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((186, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((187, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 196 });
        self.action.insert((188, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 197 });
        self.action.insert((189, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 198 });
        self.action.insert((190, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((191, TokenType::At), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((191, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 21 });
        self.action.insert((192, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 199 });
        self.action.insert((193, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 200 });
        self.action.insert((193, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 201 });
        self.action.insert((194, TokenType::ConditionalExpression), ActionNode { action: ParserAction::Goto, value: 202 });
        self.action.insert((194, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 42 });
        self.action.insert((194, TokenType::ComparisonOperatorUnary), ActionNode { action: ParserAction::Goto, value: 44 });
        self.action.insert((194, TokenType::Not), ActionNode { action: ParserAction::Shift, value: 45 });
        self.action.insert((195, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 203 });
        self.action.insert((196, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 204 });
        self.action.insert((196, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((196, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((196, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((196, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((196, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((196, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((196, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((196, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((196, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((196, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((196, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((196, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((196, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((196, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((196, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((196, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((196, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((196, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((196, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((197, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((197, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 55 });
        self.action.insert((198, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 205 });
        self.action.insert((198, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((198, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((198, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((198, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((198, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((198, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((198, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((198, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((198, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((198, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((198, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((198, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((198, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((198, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((198, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((198, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((198, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((198, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((198, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((199, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((199, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((200, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((200, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((201, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 206 });
        self.action.insert((202, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 207 });
        self.action.insert((203, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 208 });
        self.action.insert((203, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((203, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((203, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((203, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((203, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((203, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((203, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((203, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((203, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((203, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((203, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((203, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((203, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((203, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((203, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((203, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((203, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((203, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((203, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((204, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 209 });
        self.action.insert((205, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 210 });
        self.action.insert((206, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 211 });
        self.action.insert((206, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((206, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((206, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((206, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((206, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((206, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((206, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((206, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((206, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((206, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((206, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((206, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((206, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((206, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((206, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((206, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((206, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((206, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((206, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((207, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 212 });
        self.action.insert((207, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((207, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((207, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((207, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((207, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((207, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((207, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((207, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((207, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((207, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((207, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((207, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((207, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((207, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((207, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((207, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((207, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((207, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((207, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((208, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 213 });
        self.action.insert((209, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((209, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((210, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((210, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 56 });
        self.action.insert((211, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 214 });
        self.action.insert((212, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 215 });
        self.action.insert((213, TokenType::StatementConditional3), ActionNode { action: ParserAction::Goto, value: 216 });
        self.action.insert((213, TokenType::Else), ActionNode { action: ParserAction::Shift, value: 217 });
        self.action.insert((214, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((214, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((215, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((215, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 41 });
        self.action.insert((216, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((216, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 42 });
        self.action.insert((217, TokenType::LeftCurlyBrace), ActionNode { action: ParserAction::Shift, value: 218 });
        self.action.insert((218, TokenType::StatementSuiteFunction), ActionNode { action: ParserAction::Goto, value: 219 });
        self.action.insert((218, TokenType::StatementListFunction), ActionNode { action: ParserAction::Goto, value: 95 });
        self.action.insert((218, TokenType::StatementLimited), ActionNode { action: ParserAction::Goto, value: 96 });
        self.action.insert((218, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 97 });
        self.action.insert((218, TokenType::StatementComplex), ActionNode { action: ParserAction::Goto, value: 98 });
        self.action.insert((218, TokenType::StatementExpression), ActionNode { action: ParserAction::Goto, value: 99 });
        self.action.insert((218, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 100 });
        self.action.insert((218, TokenType::StatementConditional), ActionNode { action: ParserAction::Goto, value: 101 });
        self.action.insert((218, TokenType::StatementLoop), ActionNode { action: ParserAction::Goto, value: 102 });
        self.action.insert((218, TokenType::StatementExpression2), ActionNode { action: ParserAction::Goto, value: 104 });
        self.action.insert((218, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 103 });
        self.action.insert((218, TokenType::If), ActionNode { action: ParserAction::Shift, value: 109 });
        self.action.insert((218, TokenType::StatementLoopFor), ActionNode { action: ParserAction::Goto, value: 110 });
        self.action.insert((218, TokenType::StatementLoopWhile), ActionNode { action: ParserAction::Goto, value: 111 });
        self.action.insert((218, TokenType::StatementExpression3), ActionNode { action: ParserAction::Goto, value: 106 });
        self.action.insert((218, TokenType::For), ActionNode { action: ParserAction::Shift, value: 112 });
        self.action.insert((218, TokenType::While), ActionNode { action: ParserAction::Shift, value: 113 });
        self.action.insert((218, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 107 });
        self.action.insert((218, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Shift, value: 108 });
        self.action.insert((218, TokenType::Subtract), ActionNode { action: ParserAction::Shift, value: 105 });
        self.action.insert((219, TokenType::RightCurlyBrace), ActionNode { action: ParserAction::Shift, value: 220 });
        self.action.insert((220, TokenType::While), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::For), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::If), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::Identifier), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::Subtract), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::LeftParenthesis), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::Term), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 43 });
        self.action.insert((220, TokenType::Elif), ActionNode { action: ParserAction::Reduce, value: 43 });
    }
    pub fn parse(&mut self) -> Box<Node> {
        self.initialize();
        let mut stack: Vec<i32> = Vec::new();
        stack.push(0);
        let mut node_stack: Vec<Box<Node>> = Vec::new();
        let token_stack = self.token_stack.clone();
        let mut iterator = token_stack.iter();
        let mut token = iterator.next().unwrap();
        loop {
            let mut state = *stack.last().unwrap();
            print!("State: {}, TokenType: {:?} -> ", state, token.token_type);
            let action_node = self.action.get(&(state, token.token_type)).unwrap();
            println!("{:?} {}", action_node.action, action_node.value);
            match action_node.action {
                ParserAction::Shift => {
                    if token.token_type == TokenType::Term { self.token_stack.push(token.clone()) }
                    stack.push(action_node.value);

                    match token.token_type {
                        TokenType::Term => {
                            node_stack.push(Box::new(NodeTerm { payload: token.clone() }));
                        }
                        TokenType::Identifier => {
                            node_stack.push(Box::new(NodeIdentifier { payload: token.clone() }));
                        }
                        _ => {}
                    }

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
                                let statement_limited = node_stack.pop().unwrap();
                                let node = NodeStatementListFunction { statement_limited: statement_limited };
                                node_stack.push(Box::new(node));
                            }
                            3 => {
                                //NTS_STATEMENT_SUITE_CLASS -> NTS_STATEMENT_LIST_CLASS TS_NEWLINE
                                let statement_list_class = node_stack.pop().unwrap();
                                let node = NodeStatementSuiteClass { statement_list_class: statement_list_class };
                                node_stack.push(Box::new(node));
                            }
                            4 => {
                                //NTS_STATEMENT_LIST -> NTS_STATEMENT NTS_STATEMENT_LIST
                                let statement_list = node_stack.pop().unwrap();
                                let statement = node_stack.pop().unwrap();
                                let node = NodeStatementListRecursive { statement: statement, statement_list: statement_list };
                                node_stack.push(Box::new(node));
                            }
                            5 => {
                                //NTS_STATEMENT_LIST -> NTS_STATEMENT
                                let statement = node_stack.pop().unwrap();
                                let node = NodeStatementList { statement: statement };
                                node_stack.push(Box::new(node));
                            }
                            6 => {
                                //NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED NTS_STATEMENT_LIST_FUNCTION
                                let statement_list_function = node_stack.pop().unwrap();
                                let statement_limited = node_stack.pop().unwrap();
                                let node = NodeStatementListFunctionRecursive { statement_limited: statement_limited, statement_list_function: statement_list_function };
                                node_stack.push(Box::new(node));
                            }
                            7 => {
                                //NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED
                                let statement_limited = node_stack.pop().unwrap();
                                let node = NodeStatementListFunction { statement_limited: statement_limited };
                                node_stack.push(Box::new(node));
                            }
                            8 => {
                                //NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED NTS_STATEMENT_LIST_CLASS
                                let statement_list_class = node_stack.pop().unwrap();
                                let statement_restricted = node_stack.pop().unwrap();
                                let node = NodeStatementListClassRecursive { statement_restricted: statement_restricted, statement_list_class: statement_list_class };
                                node_stack.push(Box::new(node));
                            }
                            9 => {
                                //NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED
                                let statement_restricted = node_stack.pop().unwrap();
                                let node = NodeStatementListClass { statement_restricted: statement_restricted };
                                node_stack.push(Box::new(node));
                            }
                            10 | 11 | 12 | 13 => {
                                //NTS_STATEMENT -> NTS_STATEMENT_SIMPLE
                                //NTS_STATEMENT -> NTS_STATEMENT_COMPLEX
                                //NTS_STATEMENT -> NTS_STATEMENT_FUNCTION
                                //NTS_STATEMENT -> NTS_STATEMENT_CLASS
                                let statement_x = node_stack.pop().unwrap();
                                let node = NodeStatement { statement_x: statement_x };
                                node_stack.push(Box::new(node));
                            }
                            14 | 15 => {
                                //NTS_STATEMENT_LIMITED -> NTS_STATEMENT_SIMPLE
                                //NTS_STATEMENT_LIMITED -> NTS_STATEMENT_COMPLEX
                                let statement_x = node_stack.pop().unwrap();
                                let node = NodeStatementLimited { statement_x: statement_x };
                                node_stack.push(Box::new(node));
                            }
                            16 => {
                                //NTS_STATEMENT_RESTRICTED -> NTS_STATEMENT_FUNCTION
                                let statement_x = node_stack.pop().unwrap();
                                let node = NodeStatementRestricted { statement_x: statement_x };
                                node_stack.push(Box::new(node));
                            }
                            17 | 18 | 19 => {
                                //NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_EXPRESSION
                                //NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_ASSIGNMENT
                                //NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_CONDITIONAL
                                let statement_x = node_stack.pop().unwrap();
                                let node = NodeStatementSimple { statement_x: statement_x };
                                node_stack.push(Box::new(node));
                            }
                            20 => {
                                //NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_LOOP
                                let statement_x = node_stack.pop().unwrap();
                                let node = NodeStatementComplex { statement_x: statement_x };
                                node_stack.push(Box::new(node));
                            }
                            21 => {
                                //NTS_STATEMENT_FUNCTION -> TS_AT TS_IDENTIFIER TS_COLON NTS_FUNCTION_PARAMS TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                                let statement_suite_function = node_stack.pop().unwrap();
                                let function_params = node_stack.pop().unwrap();
                                let identifier = node_stack.pop().unwrap();
                                let node = NodeStatementFunction { identifier: identifier, function_params: function_params, statement_suite_function: statement_suite_function };
                                node_stack.push(Box::new(node));
                            }
                            22 => {
                                //NTS_FUNCTION_PARAMS -> NTS_FUNCTION_PARAMS TS_COMMA TS_IDENTIFIER
                                let identifier = node_stack.pop().unwrap();
                                let function_params = node_stack.pop().unwrap();
                                let node = NodeFunctionParamsRecursive { function_params: function_params, identifier: identifier };
                                node_stack.push(Box::new(node));
                            }
                            23 => {
                                //NTS_FUNCTION_PARAMS -> TS_IDENTIFIER
                                let identifier = node_stack.pop().unwrap();
                                let node = NodeFunctionParams { identifier: identifier };
                                node_stack.push(Box::new(node));
                            }
                            24 => {
                                //NTS_STATEMENT_CLASS -> TS_AT TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_CLASS TS_RIGHT_CURLY_BRACE
                                let statement_suite_class = node_stack.pop().unwrap();
                                let identifier = node_stack.pop().unwrap();
                                let node = NodeStatementClass { identifier: identifier, statement_suite_class: statement_suite_class };
                                node_stack.push(Box::new(node));
                            }
                            25 => {
                                //NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2 NTS_STATEMENT_EXPRESSION_P
                                let statement_expression_p = node_stack.pop().unwrap();
                                let statement_expression_2 = node_stack.pop().unwrap();
                                let node = NodeStatementExpressionRecursive { statement_expression_2: statement_expression_2, statement_expression_p: statement_expression_p };
                                node_stack.push(Box::new(node));
                            }
                            26 => {
                                //NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2
                                let statement_expression_2 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression { statement_expression_2: statement_expression_2 };
                                node_stack.push(Box::new(node));
                            }
                            27 => {
                                //NTS_STATEMENT_EXPRESSION_P -> TS_ADD NTS_STATEMENT_EXPRESSION
                                let statement_expression = node_stack.pop().unwrap();
                                let node = NodeStatementExpressionP { statement_expression: statement_expression, operator: TokenType::Add };
                                node_stack.push(Box::new(node));
                            }
                            28 => {
                                //NTS_STATEMENT_EXPRESSION_P -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION
                                let statement_expression = node_stack.pop().unwrap();
                                let node = NodeStatementExpressionP { statement_expression: statement_expression, operator: TokenType::Subtract };
                                node_stack.push(Box::new(node));
                            }
                            29 => {
                                //NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3 NTS_STATEMENT_EXPRESSION_2P
                                let statement_expression_2p = node_stack.pop().unwrap();
                                let statement_expression_3 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression2Recursive { statement_expression_3: statement_expression_3, statement_expression_2p: statement_expression_2p };
                                node_stack.push(Box::new(node));
                            }
                            30 => {
                                //NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3
                                let statement_expression_3 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression2 { statement_expression_3: statement_expression_3 };
                                node_stack.push(Box::new(node));
                            }
                            31 => {
                                //NTS_STATEMENT_EXPRESSION_2P -> TS_MULTIPLY NTS_STATEMENT_EXPRESSION_2
                                let statement_expression_2 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression2p { statement_expression_2: statement_expression_2, operator: TokenType::Multiply };
                                node_stack.push(Box::new(node));
                            }
                            32 => {
                                //NTS_STATEMENT_EXPRESSION_2P -> TS_DIVIDE NTS_STATEMENT_EXPRESSION_2
                                let statement_expression_2 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression2p { statement_expression_2: statement_expression_2, operator: TokenType::Divide };
                                node_stack.push(Box::new(node));
                            }
                            33 => {
                                //NTS_STATEMENT_EXPRESSION_2P -> TS_MODULO NTS_STATEMENT_EXPRESSION_2
                                let statement_expression_2 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression2p { statement_expression_2: statement_expression_2, operator: TokenType::Modulo };
                                node_stack.push(Box::new(node));
                            }
                            34 => {
                                //NTS_STATEMENT_EXPRESSION_3 -> TS_TERM
                                let term = node_stack.pop().unwrap();
                                let node = NodeStatementExpression3 { term: term };
                                node_stack.push(Box::new(node));
                            }
                            35 => {
                                //NTS_STATEMENT_EXPRESSION_3 -> TS_LEFT_PARENTHESIS NTS_STATEMENT_EXPRESSION TS_RIGHT_PARENTHESIS
                                let statement_expression = node_stack.pop().unwrap();
                                let node = NodeStatementExpression3Paren { statement_expression: statement_expression };
                                node_stack.push(Box::new(node));
                            }
                            36 => {
                                //NTS_STATEMENT_EXPRESSION_3 -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION_3
                                let statement_expression_3 = node_stack.pop().unwrap();
                                let node = NodeStatementExpression3Negation { statement_expression_3: statement_expression_3 };
                                node_stack.push(Box::new(node));
                            }
                            37 => {
                                //NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS NTS_STATEMENT_EXPRESSION
                                let statement_expression = node_stack.pop().unwrap();
                                let identifier = node_stack.pop().unwrap();
                                let node = NodeStatementAssignment { identifier: identifier, statement_expression: statement_expression };
                                node_stack.push(Box::new(node));
                            }
                            38 => {
                                //NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                                let statement_suite_function = node_stack.pop().unwrap();
                                let conditional_expression = node_stack.pop().unwrap();
                                let node = NodeStatementConditional { conditional_expression: conditional_expression, statement_suite_function: statement_suite_function };
                                node_stack.push(Box::new(node));
                            }
                            39 => {
                                //NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_2
                                let statement_conditional_2 = node_stack.pop().unwrap();
                                let statement_suite_function = node_stack.pop().unwrap();
                                let conditional_expression = node_stack.pop().unwrap();
                                let node = NodeStatementConditionalW2 { conditional_expression: conditional_expression, statement_suite_function: statement_suite_function, statement_conditional_2: statement_conditional_2 };
                                node_stack.push(Box::new(node));
                            }
                            40 => {
                                //NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
                                let statement_conditional_3 = node_stack.pop().unwrap();
                                let statement_suite_function = node_stack.pop().unwrap();
                                let conditional_expression = node_stack.pop().unwrap();
                                let node = NodeStatementConditionalW3 { conditional_expression: conditional_expression, statement_suite_function: statement_suite_function, statement_conditional_3: statement_conditional_3 };
                                node_stack.push(Box::new(node));
                            }
                            41 => {
                                //NTS_STATEMENT_CONDITIONAL_2 -> NTS_STATEMENT_CONDITIONAL_2 TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                                let statement_suite_function = node_stack.pop().unwrap();
                                let conditional_expression = node_stack.pop().unwrap();
                                let statement_conditional_2 = node_stack.pop().unwrap();
                                let node = NodeStatementConditional2Recursive { statement_conditional_2: statement_conditional_2, conditional_expression: conditional_expression, statement_suite_function: statement_suite_function };
                                node_stack.push(Box::new(node));
                            }
                            42 => {
                                //NTS_STATEMENT_CONDITIONAL_2 -> TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
                                let statement_conditional_3 = node_stack.pop().unwrap();
                                let statement_suite_function = node_stack.pop().unwrap();
                                let conditional_expression = node_stack.pop().unwrap();
                                let node = NodeStatementConditional2 { conditional_expression: conditional_expression, statement_suite_function: statement_suite_function, statement_conditional_3: statement_conditional_3 };
                                node_stack.push(Box::new(node));
                            }
                            43 => {
                                //NTS_STATEMENT_CONDITIONAL_3 -> TS_ELSE TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
                                let statement_suite_function = node_stack.pop().unwrap();
                                let node = NodeStatementConditional3 { statement_suite_function: statement_suite_function };
                                node_stack.push(Box::new(node));
                            }
                            44 | 45 => {
                                //NTS_CONDITIONAL_EXPRESSION -> TS_TERM NTS_COMPARISON_OPERATOR TS_TERM
                                let term2 = node_stack.pop().unwrap();
                                let comparison_operator = node_stack.pop().unwrap();
                                let term1 = node_stack.pop().unwrap();
                                let node = NodeConditionalExpression { term1: term1, comparison_operator: comparison_operator, term2: term2 };
                                node_stack.push(Box::new(node));
                            }
                            45 => {
                                //NTS_CONDITIONAL_EXPRESSION -> NTS_COMPARISON_OPERATOR_UNARY TS_TERM
                                let term = node_stack.pop().unwrap();
                                let comparison_operator_unary = node_stack.pop().unwrap();
                                let node = NodeConditionalExpressionUnary { comparison_operator_unary: comparison_operator_unary, term: term };
                                node_stack.push(Box::new(node));
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
