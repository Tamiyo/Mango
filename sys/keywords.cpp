//
// Created by Matt on 4/15/2019.
//

#include "keywords.h"

keywords::keywords() {
	KEYWORDS = {
		{"if", TS_IF},
		{"elif", TS_ELSEIF},
		{"else", TS_ELSE},

		{"{", TS_LCB},
		{"}", TS_RCB},
		{"(", TS_RPAREN},
		{")", TS_LPAREN},

		{"=", TS_EQUALS},
		{"+", TS_PLUS},
		{"-", TS_MINUS},
		{"*", TS_MUL},
		{"/", TS_DIV},
		{"^", TS_EXP},

		{"<", TS_LT},
		{">", TS_GT},
		{">=", TS_GTE},
		{"<=", TS_LTE},
		{"==", TS_EQUIV},
		{"===", TS_TEQUIV},

		{" ", TS_SPACE},
		{"\n", TS_NEWLINE},
		{"\\eof", TS_EOF},
	};

	TYPES = {
		{0, TS_IDENT},
		{1, TS_VARIABLE},
		{2, TS_VARIABLE},
		{3, TS_VARIABLE},
		{4, NTS_OPERATOR}
	};

}
