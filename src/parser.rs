use crate::core::{TokenType, ActionNode, GotoNode, ParserAction, LexerResult};
use std::vec::Vec;
use std::collections::HashMap;
use crate::core::TokenType::Term;
use core::borrow::Borrow;
use std::slice::Iter;


pub struct Parser {
    pub stack: Vec<i32>,
    pub token_stack: Vec<LexerResult>,
    pub node_stack: Vec<ActionNode>,
    pub action: HashMap<(i32, TokenType), ActionNode>,
    pub goto: HashMap<i32, GotoNode>,
}

impl Default for Parser {
    fn default() -> Parser {
        Parser {
            stack: Vec::new(),
            token_stack: Vec::new(),
            node_stack: Vec::new(),
            action: HashMap::new(),
            goto: HashMap::new(),
        }
    }
}

impl Parser {
    fn initialize(&mut self) {
        self.stack.push(0);

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

    pub fn parse(&mut self) {
        self.initialize();
        let mut token = self.token_stack.first().unwrap();

        loop {
            let mut state = self.stack.pop().unwrap();
            println!("Token: {}, State: {}, TokenType: {:?}", token.token, state, token.token_type);
            let action_node = self.action.get(&(state, token.token_type)).unwrap();
            println!("Action: {:?}", action_node.action);

            match action_node.action {
                ParserAction::Shift => {
//                    if token.token_type == TokenType::Term { self.token_stack.push(token) }
                    self.stack.push(action_node.value);
                    token = &self.token_stack.pop().unwrap();
                }
                ParserAction::Reduce => {
                    let goto_node = self.goto.get(&action_node.value).unwrap();
                    for _ in 0..goto_node.value { self.stack.pop(); }
                    state = *self.stack.get(0).unwrap();
                    let goto_action = self.action.get(&(state, goto_node.token_type)).unwrap();
                    self.stack.push(goto_action.value);
                }
                ParserAction::Accept => { println!("Parse Accepted!"); }
                _ => {
                    println!("Parse Error!");
                    break;
                }
            }
        }
    }
}