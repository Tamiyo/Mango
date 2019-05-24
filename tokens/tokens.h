#ifndef MANGO_CL_TOKENS_H
#define MANGO_CL_TOKENS_H
#include<map>
#include<string>

class tokens {
public:
		tokens();

		enum Symbols {
		// Terminal Symbols
		TS_STRING,
		TS_IDENT,
		TS_FLOAT,
		TS_INT,
		TS_VARIABLE,

		// Control Symbols
		TS_IF,
		TS_ELSEIF,
		TS_ELSE,

		// Grouping Symbols
		TS_LCB,
		TS_RCB,
		TS_LPAREN,
		TS_RPAREN,

		// Operator Symbols,
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

		NTS_OPERATOR,

		// Non-Terminal Symbols (Generated)
		NTS_MANGO,
		NTS_STMTS,
		NTS_STMT,
		NTS_ASSIGN,
		NTS_EXPR,

MYSBL_END
	};

	std::map<std::string, Symbols> TOKENS;
	std::map<int, Symbols> TYPES;
};

#endif //MANGO_CL_TOKENS_H