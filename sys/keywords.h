//
// Created by Matt on 4/15/2019.
//

#ifndef CLIONLANG_KEYWORDS_H
#define CLIONLANG_KEYWORDS_H

#include<map>
#include<string>

using std::map;
using std::string;

class keywords {
public:
	keywords();

	enum Symbols {
		// Terminal Symbols

		// "Types", denoted as TS_VARIABLE for now in grammer.mg
		TS_STRING,
		TS_IDENT,
		TS_FLOAT,
		TS_INT,

		// Control Symbols
		TS_IF,
		TS_ELSEIF,
		TS_ELSE,

		// Grouping Symbols
		TS_LCB,
		TS_RCB,
		TS_LPAREN,
		TS_RPAREN,

		// Operator Symbols
		TS_EQUALS,
		TS_PLUS,
		TS_MINUS,
		TS_MUL,
		TS_DIV,
		TS_EXP,

		// Comparison Symbols
		TS_LT,
		TS_LTE,
		TS_GT,
		TS_GTE,
		TS_EQUIV,
		TS_TEQUIV,

		// Single Comparison Symbols
		TS_NEG,
		TS_NOTNULL,

		// Other Symbols
		TS_SPACE,
		TS_NEWLINE,
		TS_EMPTY,
		TS_EOF,

		// Non-Terminal Symbols
		NTS_MANGO,
		NTS_STMTS,
		NTS_STMT,
		NTS_ASSIGN,
		NTS_EXPR,
		NTS_EXPR_A,
		NTS_CTRL,
		NTS_CTRL_IF,
		NTS_CTRL_ELSEIF,
		NTS_CTRL_ELSE,
		NTS_COND,
		NTS_OPERATOR,
		NTS_COND_OPERATOR,
		NTS_COND_S_OPERATOR,
		NTS_VARIABLE,

		// Personal Iterative Symbol
		TS_$,
	};

	std::map<std::string, Symbols> conversion_table = {
		{"TS_STRING",TS_STRING},
		{"TS_IDENT",TS_IDENT},
		{"TS_FLOAT",TS_FLOAT},
		{"TS_INT",TS_INT},
		{"TS_IF",TS_IF},
		{"TS_ELSEIF",TS_ELSEIF},
		{"TS_ELSE",TS_ELSE},
		{"TS_LCB",TS_LCB},
		{"TS_RCB",TS_RCB},
		{"TS_LPAREN",TS_LPAREN},
		{"TS_RPAREN",TS_RPAREN},
		{"TS_EQUALS",TS_EQUALS},
		{"TS_PLUS",TS_PLUS},
		{"TS_MINUS",TS_MINUS},
		{"TS_MUL",TS_MUL},
		{"TS_DIV",TS_DIV},
		{"TS_EXP",TS_EXP},
		{"TS_LT",TS_LT},
		{"TS_LTE",TS_LTE},
		{"TS_GT",TS_GT},
		{"TS_GTE",TS_GTE},
		{"TS_EQUIV",TS_EQUIV},
		{"TS_TEQUIV",TS_TEQUIV},
		{"TS_NEG",TS_NEG},
		{"TS_NOTNULL",TS_NOTNULL},
		{"TS_SPACE",TS_SPACE},
		{"TS_NEWLINE",TS_NEWLINE},
		{"TS_EMPTY",TS_EMPTY},
		{"TS_EOF",TS_EOF},
		{"NTS_MANGO",NTS_MANGO},
		{"NTS_STMTS",NTS_STMTS},
		{"NTS_STMT",NTS_STMT},
		{"NTS_ASSIGN",NTS_ASSIGN},
		{"NTS_EXPR",NTS_EXPR},
		{"NTS_EXPR_A",NTS_EXPR_A},
		{"NTS_CTRL",NTS_CTRL},
		{"NTS_CTRL_IF",NTS_CTRL_IF},
		{"NTS_CTRL_ELSEIF",NTS_CTRL_ELSEIF},
		{"NTS_CTRL_ELSE",NTS_CTRL_ELSE},
		{"NTS_COND",NTS_COND},
		{"NTS_OPERATOR",NTS_OPERATOR},
		{"NTS_COND_OPERATOR",NTS_COND_OPERATOR},
		{"NTS_COND_S_OPERATOR",NTS_COND_S_OPERATOR},
		{"NTS_VARIABLE",NTS_VARIABLE},
		{"ENUM_TYPE_END",TS_$},
	};

	map<string, Symbols> KEYWORDS;
	map<int, Symbols> TYPES;

};


#endif //CLIONLANG_KEYWORDS_H
