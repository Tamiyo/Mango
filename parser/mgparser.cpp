//
// Created by Matt on 4/16/2019.
//

#include "mgparser.h"

/// TODO Convert Grammar to LL Grammar by removing left-recursion
/// http://www.montefiore.ulg.ac.be/~geurts/Cours/compil/2012/03-syntaxanalysis-2-2012-2013.pdf

/// TODO Improve grammar and convert to LR parsing table (LALR soon)
/// http://jsmachines.sourceforge.net/machines/lr1.html

/// TODO Remove left-recursion
/// https://lab.brainonfire.net/CFG/remove-left-recursion.html

mgparser::mgparser(const char *body) {
	lexer = new mglex(body);

	// Initialize the stack
	ss.push(keywords::TS_EOF);
	ss.push(keywords::NTS_MANGO);

	// Create the LL parsing table
	table[keywords::NTS_MANGO][keywords::TS_NEWLINE] = 2;
	table[keywords::NTS_MANGO][keywords::NTS_CTRL] = 2;
	table[keywords::NTS_MANGO][keywords::TS_IDENT] = 29;
	table[keywords::NTS_MANGO][keywords::TS_EQUALS] = 2;
	table[keywords::NTS_MANGO][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_MANGO][keywords::TS_PLUS] = 29;
	table[keywords::NTS_MANGO][keywords::TS_MINUS] = 29;
	table[keywords::NTS_MANGO][keywords::TS_MUL] = 29;
	table[keywords::NTS_MANGO][keywords::TS_DIV] = 29;
	table[keywords::NTS_MANGO][keywords::TS_EXP] = 29;
	table[keywords::NTS_MANGO][keywords::TS_LT] = 29;
	table[keywords::NTS_MANGO][keywords::TS_LTE] = 29;
	table[keywords::NTS_MANGO][keywords::TS_GT] = 29;
	table[keywords::NTS_MANGO][keywords::TS_GTE] = 29;
	table[keywords::NTS_MANGO][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_MANGO][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_MANGO][keywords::TS_NEG] = 29;
	table[keywords::NTS_MANGO][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_MANGO][keywords::TS_STRING] = 29;
	table[keywords::NTS_MANGO][keywords::TS_INT] = 29;
	table[keywords::NTS_MANGO][keywords::TS_FLOAT] = 28;
	table[keywords::NTS_STMTS][keywords::TS_NEWLINE] = 5;
	table[keywords::NTS_STMTS][keywords::NTS_CTRL] = 3;
	table[keywords::NTS_STMTS][keywords::TS_IDENT] = 29;
	table[keywords::NTS_STMTS][keywords::TS_EQUALS] = 4;
	table[keywords::NTS_STMTS][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_STMTS][keywords::TS_PLUS] = 29;
	table[keywords::NTS_STMTS][keywords::TS_MINUS] = 29;
	table[keywords::NTS_STMTS][keywords::TS_MUL] = 29;
	table[keywords::NTS_STMTS][keywords::TS_DIV] = 29;
	table[keywords::NTS_STMTS][keywords::TS_EXP] = 29;
	table[keywords::NTS_STMTS][keywords::TS_LT] = 29;
	table[keywords::NTS_STMTS][keywords::TS_LTE] = 29;
	table[keywords::NTS_STMTS][keywords::TS_GT] = 29;
	table[keywords::NTS_STMTS][keywords::TS_GTE] = 29;
	table[keywords::NTS_STMTS][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_STMTS][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_STMTS][keywords::TS_NEG] = 29;
	table[keywords::NTS_STMTS][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_STMTS][keywords::TS_STRING] = 29;
	table[keywords::NTS_STMTS][keywords::TS_INT] = 29;
	table[keywords::NTS_STMTS][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_STMT][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_STMT][keywords::NTS_CTRL] = 6;
	table[keywords::NTS_STMT][keywords::TS_IDENT] = 29;
	table[keywords::NTS_STMT][keywords::TS_EQUALS] = 29;
	table[keywords::NTS_STMT][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_STMT][keywords::TS_PLUS] = 29;
	table[keywords::NTS_STMT][keywords::TS_MINUS] = 29;
	table[keywords::NTS_STMT][keywords::TS_MUL] = 29;
	table[keywords::NTS_STMT][keywords::TS_DIV] = 29;
	table[keywords::NTS_STMT][keywords::TS_EXP] = 29;
	table[keywords::NTS_STMT][keywords::TS_LT] = 29;
	table[keywords::NTS_STMT][keywords::TS_LTE] = 29;
	table[keywords::NTS_STMT][keywords::TS_GT] = 29;
	table[keywords::NTS_STMT][keywords::TS_GTE] = 29;
	table[keywords::NTS_STMT][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_STMT][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_STMT][keywords::TS_NEG] = 29;
	table[keywords::NTS_STMT][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_STMT][keywords::TS_STRING] = 29;
	table[keywords::NTS_STMT][keywords::TS_INT] = 29;
	table[keywords::NTS_STMT][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_ASSIGN][keywords::NTS_CTRL] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_IDENT] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_EQUALS] = 7;
	table[keywords::NTS_ASSIGN][keywords::NTS_VARIABLE] = 28;
	table[keywords::NTS_ASSIGN][keywords::TS_PLUS] = 28;
	table[keywords::NTS_ASSIGN][keywords::TS_MINUS] = 28;
	table[keywords::NTS_ASSIGN][keywords::TS_MUL] = 28;
	table[keywords::NTS_ASSIGN][keywords::TS_DIV] = 28;
	table[keywords::NTS_ASSIGN][keywords::TS_EXP] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_LT] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_LTE] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_GT] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_GTE] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_NEG] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_STRING] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_INT] = 29;
	table[keywords::NTS_ASSIGN][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_EXPR][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_EXPR][keywords::NTS_CTRL] = 29;
	table[keywords::NTS_EXPR][keywords::TS_IDENT] = 29;
	table[keywords::NTS_EXPR][keywords::TS_EQUALS] = 29;
	table[keywords::NTS_EXPR][keywords::NTS_VARIABLE] = 8;
	table[keywords::NTS_EXPR][keywords::TS_PLUS] = 8;
	table[keywords::NTS_EXPR][keywords::TS_MINUS] = 8;
	table[keywords::NTS_EXPR][keywords::TS_MUL] = 8;
	table[keywords::NTS_EXPR][keywords::TS_DIV] = 8;
	table[keywords::NTS_EXPR][keywords::TS_EXP] = 29;
	table[keywords::NTS_EXPR][keywords::TS_LT] = 29;
	table[keywords::NTS_EXPR][keywords::TS_LTE] = 29;
	table[keywords::NTS_EXPR][keywords::TS_GT] = 29;
	table[keywords::NTS_EXPR][keywords::TS_GTE] = 29;
	table[keywords::NTS_EXPR][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_EXPR][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_EXPR][keywords::TS_NEG] = 29;
	table[keywords::NTS_EXPR][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_EXPR][keywords::TS_STRING] = 29;
	table[keywords::NTS_EXPR][keywords::TS_INT] = 29;
	table[keywords::NTS_EXPR][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_EXPR_A][keywords::NTS_CTRL] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_IDENT] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_EQUALS] = 9;
	table[keywords::NTS_EXPR_A][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_PLUS] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_MINUS] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_MUL] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_DIV] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_EXP] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_LT] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_LTE] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_GT] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_GTE] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_TEQUIV] = 10;
	table[keywords::NTS_EXPR_A][keywords::TS_NEG] = 10;
	table[keywords::NTS_EXPR_A][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_STRING] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_INT] = 29;
	table[keywords::NTS_EXPR_A][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_CTRL][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_CTRL][keywords::NTS_CTRL] = 29;
	table[keywords::NTS_CTRL][keywords::TS_IDENT] = 29;
	table[keywords::NTS_CTRL][keywords::TS_EQUALS] = 28;
	table[keywords::NTS_CTRL][keywords::NTS_VARIABLE] = 11;
	table[keywords::NTS_CTRL][keywords::TS_PLUS] = 12;
	table[keywords::NTS_CTRL][keywords::TS_MINUS] = 13;
	table[keywords::NTS_CTRL][keywords::TS_MUL] = 14;
	table[keywords::NTS_CTRL][keywords::TS_DIV] = 15;
	table[keywords::NTS_CTRL][keywords::TS_EXP] = 29;
	table[keywords::NTS_CTRL][keywords::TS_LT] = 29;
	table[keywords::NTS_CTRL][keywords::TS_LTE] = 29;
	table[keywords::NTS_CTRL][keywords::TS_GT] = 29;
	table[keywords::NTS_CTRL][keywords::TS_GTE] = 29;
	table[keywords::NTS_CTRL][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_CTRL][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_CTRL][keywords::TS_NEG] = 29;
	table[keywords::NTS_CTRL][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_CTRL][keywords::TS_STRING] = 29;
	table[keywords::NTS_CTRL][keywords::TS_INT] = 29;
	table[keywords::NTS_CTRL][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_CTRL_IF][keywords::NTS_CTRL] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_IDENT] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_EQUALS] = 28;
	table[keywords::NTS_CTRL_IF][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_PLUS] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_MINUS] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_MUL] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_DIV] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_EXP] = 16;
	table[keywords::NTS_CTRL_IF][keywords::TS_LT] = 17;
	table[keywords::NTS_CTRL_IF][keywords::TS_LTE] = 18;
	table[keywords::NTS_CTRL_IF][keywords::TS_GT] = 19;
	table[keywords::NTS_CTRL_IF][keywords::TS_GTE] = 20;
	table[keywords::NTS_CTRL_IF][keywords::TS_EQUIV] = 21;
	table[keywords::NTS_CTRL_IF][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_NEG] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_STRING] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_INT] = 29;
	table[keywords::NTS_CTRL_IF][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::NTS_CTRL] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_IDENT] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_EQUALS] = 28;
	table[keywords::NTS_CTRL_ELSEIF][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_PLUS] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_MINUS] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_MUL] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_DIV] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_EXP] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_LT] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_LTE] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_GT] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_GTE] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_TEQUIV] = 22;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_NEG] = 23;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_NOTNULL] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_STRING] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_INT] = 29;
	table[keywords::NTS_CTRL_ELSEIF][keywords::TS_FLOAT] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_NEWLINE] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::NTS_CTRL] = 27;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_IDENT] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_EQUALS] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::NTS_VARIABLE] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_PLUS] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_MINUS] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_MUL] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_DIV] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_EXP] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_LT] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_LTE] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_GT] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_GTE] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_EQUIV] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_TEQUIV] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_NEG] = 29;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_NOTNULL] = 24;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_STRING] = 25;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_INT] = 26;
	table[keywords::NTS_CTRL_ELSE][keywords::TS_FLOAT] = 29;
}

void mgparser::ppeval() {

}
