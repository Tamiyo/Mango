use core::borrow::Borrow;
use std::collections::HashMap;
use std::ptr::null;
use std::slice::Iter;
use std::vec::Vec;

use crate::core::{ActionNode, GotoNode, LexerResult, ParserAction, PrimitiveType, TokenType};
use crate::core::TokenType::{StatementSuite, Term};
use crate::parse_tree::{Node, NodeIdentifier, NodeMango, NodeStatement, NodeStatementAssignment, NodeStatementList, NodeStatementSimple, NodeStatementSuite, NodeTerm};

pub struct Parser {
    pub token_stack: Vec<LexerResult>,
    pub action: HashMap<(i32, TokenType), ActionNode>,
    pub goto: HashMap<i32, GotoNode>,
}

impl Default for Parser {
    fn default() -> Parser {
        Parser {
            token_stack: Vec::new(),
            action: HashMap::new(),
            goto: HashMap::new(),
        }
    }
}

impl Parser {
    fn initialize(&mut self) {
        self.goto.insert(1, GotoNode { token_type: TokenType::StatementSuite, value: 2 });
        self.goto.insert(2, GotoNode { token_type: TokenType::StatementList, value: 3 });
        self.goto.insert(3, GotoNode { token_type: TokenType::StatementList, value: 1 });
        self.goto.insert(4, GotoNode { token_type: TokenType::Statement, value: 1 });
        self.goto.insert(5, GotoNode { token_type: TokenType::StatementSimple, value: 1 });
        self.goto.insert(6, GotoNode { token_type: TokenType::StatementAssignment, value: 3 });

        self.action.insert((0, TokenType::StatementSuite), ActionNode { action: ParserAction::Goto, value: 1 });
        self.action.insert((0, TokenType::StatementList), ActionNode { action: ParserAction::Goto, value: 2 });
        self.action.insert((0, TokenType::Statement), ActionNode { action: ParserAction::Goto, value: 3 });
        self.action.insert((0, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 4 });
        self.action.insert((0, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 5 });
        self.action.insert((0, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 6 });
        self.action.insert((1, TokenType::EndOfFile), ActionNode { action: ParserAction::Accept, value: -1 });
        self.action.insert((2, TokenType::Newline), ActionNode { action: ParserAction::Shift, value: 7 });
        self.action.insert((3, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 3 });
        self.action.insert((4, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 4 });
        self.action.insert((5, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 5 });
        self.action.insert((6, TokenType::Equals), ActionNode { action: ParserAction::Shift, value: 9 });
        self.action.insert((7, TokenType::EndOfFile), ActionNode { action: ParserAction::Reduce, value: 1 });
        self.action.insert((8, TokenType::StatementList), ActionNode { action: ParserAction::Goto, value: 10 });
        self.action.insert((8, TokenType::Statement), ActionNode { action: ParserAction::Goto, value: 3 });
        self.action.insert((8, TokenType::StatementSimple), ActionNode { action: ParserAction::Goto, value: 4 });
        self.action.insert((8, TokenType::StatementAssignment), ActionNode { action: ParserAction::Goto, value: 5 });
        self.action.insert((8, TokenType::Identifier), ActionNode { action: ParserAction::Shift, value: 6 });
        self.action.insert((9, TokenType::Term), ActionNode { action: ParserAction::Shift, value: 11 });
        self.action.insert((10, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 2 });
        self.action.insert((11, TokenType::Newline), ActionNode { action: ParserAction::Reduce, value: 6 });
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

                    if token.token_type == TokenType::Term || token.token_type == TokenType::Identifier {
                        item_stack.push(token.clone());
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
                                let node = node_stack.pop().unwrap();
                                node_stack.push(Box::from(NodeStatementSuite { statement_list: node }));
                            }
//            2 => {}
                            3 => {
                                let node = node_stack.pop().unwrap();
                                node_stack.push(Box::from(NodeStatementList { statement: node }));
                            }
                            4 => {
                                let node = node_stack.pop().unwrap();
                                node_stack.push(Box::from(NodeStatement { statement_simple: node }));
                            }
                            5 => {
                                let node = node_stack.pop().unwrap();
                                node_stack.push(Box::from(NodeStatementSimple { statement_assignment: node }));
                            }
                            6 => {
                                let term = item_stack.pop().unwrap();
                                let identifier = item_stack.pop().unwrap();
                                node_stack.push(Box::from(NodeStatementAssignment { identifier: Box::new((NodeIdentifier { value: Box::from(identifier) })), term: Box::new((NodeTerm { value: Box::from(term) })) }));
                            }
                            _ => {}
                        }
                    }
                }
                ParserAction::Accept => {
                    println!("Parse Accepted!");
                    return Box::from(NodeMango {
                        statement_suite: node_stack.pop().unwrap()
                    });
                }
                _ => {
                    println!("Parse Error!");
                    return Box::from(NodeMango {
                        statement_suite: node_stack.pop().unwrap()
                    });
                }
            }
        }
    }
}