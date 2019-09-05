//
// Created by Matt on 8/11/2019.
//

// This file contains enums and structs that act as wrappers for data.

#ifndef MANGO_V2_CPP_CORE_H
#define MANGO_V2_CPP_CORE_H

#include <string>
#include <map>

using std::string;
using std::map;

///////////////////////
// Token Definitions //
///////////////////////
enum TokenType {
    Term,
    Identifier,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    Equals,
    DoubleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Negation,
    QuestionMark,

    LeftCurlyBrace,
    RightCurlyBrace,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Colon,
    Semicolon,
    Newline,
    EndOfFile,

    For,
    While,
    Return,
    If,
    Elif,
    Else,

    None,
    At,
    Dot,
    Dash,
    Ellipsis,
    SingleQuote,
    Ampersand,
    And,

    Mango,
    StatementSuite,
    StatementSuiteFunction,
    StatementSuiteClass,
    StatementListClass,
    StatementRestricted,
    StatementList,
    Statement,
    StatementSimple,
    StatementComplex,
    StatementLimited,
    StatementFunction,
    StatementClass,
    StatementListFunction,
    StatementReturn,
    StatementExpression,
    StatementExpressionP,
    StatementExpression2,
    StatementExpression2p,
    StatementExpression3,
    StatementAssignment,
    StatementConditional,
    StatementConditional2,
    StatementConditional3,
    FunctionParamsDefine,
    StatementConditionalElif,
    StatementConditionalElse,
    StatementConditionalTest,
    ConditionalExpression,
    ComparisonOperator,
    ComparisonOperatorUnary,
    StatementLoop,
    StatementLoopFor,
    StatementLoopForOptions,
    StatementLoopWhile,
    StatementDefineFunction,
    FunctionParams,
    StatementDefineClass,
};

///////////////////
// Lexer Structs //
///////////////////
enum PrimitiveType {
    Float,
    Integer,
    String,
    Boolean,
    Object,
    Function,
    Null
};

struct LexerResult {
public:
    string token;
    PrimitiveType inferred_type;
    TokenType token_type;

    LexerResult() {
        this->token = "";
        this->inferred_type = PrimitiveType::Null;
        this->token_type = TokenType::None;
    }

    LexerResult(string token, PrimitiveType inferred_type, TokenType token_type) {
        this->token = std::move(token);
        this->inferred_type = inferred_type;
        this->token_type = token_type;
    }
};

////////////////////
// Parser Structs //
////////////////////
enum ParserAction {
    Shift,
    Reduce,
    Accept,
    Goto
};

struct ActionNode {
public:
    ParserAction action;
    int value;
};

struct GotoNode {
    TokenType token_type;
    int value;
};

TokenType identifier_to_enum(const string &symbol) {
    if (symbol == "if") { return TokenType::If; }
    if (symbol == "elif") { return TokenType::Elif; }
    if (symbol == "else") { return TokenType::Else; }
    if (symbol == "for") { return TokenType::For; }
    if (symbol == "while") { return TokenType::While; }
    if (symbol == "return") { return TokenType::Return; }
    return TokenType::None;
}

TokenType symbol_to_enum(const string &symbol) {
    if (symbol == "+") { return TokenType::Add; }
    if (symbol == "-") { return TokenType::Subtract; }
    if (symbol == "*") { return TokenType::Multiply; }
    if (symbol == "/") { return TokenType::Divide; }
    if (symbol == "%") { return TokenType::Modulo; }
    if (symbol == "^") { return TokenType::Exponent; }
    if (symbol == "=") { return TokenType::Equals; }
    if (symbol == "==") { return TokenType::DoubleEquals; }
    if (symbol == "<") { return TokenType::LessThan; }
    if (symbol == "<=") { return TokenType::LessThanEquals; }
    if (symbol == ">") { return TokenType::GreaterThan; }
    if (symbol == ">=") { return TokenType::GreaterThanEquals; }
    if (symbol == "!") { return TokenType::Negation; }
    if (symbol == "{") { return TokenType::LeftCurlyBrace; }
    if (symbol == "}") { return TokenType::RightCurlyBrace; }
    if (symbol == "(") { return TokenType::LeftParenthesis; }
    if (symbol == ")") { return TokenType::RightParenthesis; }
    if (symbol == "-") { return TokenType::Dash; }
    if (symbol == ",") { return TokenType::Comma; }
    if (symbol == ":") { return TokenType::Colon; }
    if (symbol == ";") { return TokenType::Semicolon; }
    if (symbol == "\n") { return TokenType::Newline; }
    if (symbol == "@") { return TokenType::At; }
    if (symbol == ".") { return TokenType::Dot; }
    if (symbol == "'") { return TokenType::SingleQuote; }
    if (symbol == "&") { return TokenType::Ampersand; }
    if (symbol == "&&") { return TokenType::And; }
    if (symbol == "..") { return TokenType::Ellipsis; }
    if (symbol == "$") { return TokenType::EndOfFile; }
    if (symbol == "?") { return TokenType::QuestionMark; }
    return TokenType::None;
}

#endif //MANGO_V2_CPP_CORE_H
