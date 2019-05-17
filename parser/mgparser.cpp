#include "mgparser.h"
#include <string>
#include <iostream>

mgparser::mgparser(const char* file) {
	lexer = new mglexer(file);
	ss.push(stack_symbol{0});
	GOTO[1] = {tokens::NTS_MANGO, 2};
	GOTO[2] = {tokens::NTS_STMTS, 4};
	GOTO[3] = {tokens::NTS_STMT, 2};
	GOTO[4] = {tokens::NTS_STMT, 2};
	GOTO[5] = {tokens::NTS_ASSIGN, 6};
	GOTO[6] = {tokens::NTS_EXPR, 6};
	GOTO[7] = {tokens::NTS_OPERATOR, 2};
	GOTO[8] = {tokens::NTS_OPERATOR, 2};
	GOTO[9] = {tokens::NTS_OPERATOR, 2};
	GOTO[10] = {tokens::NTS_OPERATOR, 2};
	GOTO[11] = {tokens::NTS_OPERATOR, 2};
	ACTION[0][tokens::NTS_STMTS] = "1";
	ACTION[0][tokens::NTS_STMT] = "2";
	ACTION[0][tokens::NTS_ASSIGN] = "3";
	ACTION[0][tokens::NTS_EXPR] = "4";
	ACTION[0][tokens::TS_IDENT] = "S5";
	ACTION[0][tokens::TS_VARIABLE] = "S6";
	ACTION[1][tokens::TS_EOF] = "R";
	ACTION[2][tokens::TS_NEWLINE] = "S7";
	ACTION[3][tokens::TS_NEWLINE] = "R2";
	ACTION[4][tokens::TS_NEWLINE] = "R3";
	ACTION[5][tokens::TS_EQUALS] = "S8";
	ACTION[6][tokens::NTS_OPERATOR] = "9";
	ACTION[6][tokens::TS_PLUS] = "S10";
	ACTION[6][tokens::TS_MINUS] = "S11";
	ACTION[6][tokens::TS_MUL] = "S12";
	ACTION[6][tokens::TS_DIV] = "S13";
	ACTION[6][tokens::TS_EXP] = "S14";
	ACTION[7][tokens::TS_EOF] = "R1";
	ACTION[8][tokens::NTS_EXPR] = "15";
	ACTION[8][tokens::TS_VARIABLE] = "S6";
	ACTION[9][tokens::TS_VARIABLE] = "S16";
	ACTION[10][tokens::TS_VARIABLE] = "R6";
	ACTION[11][tokens::TS_VARIABLE] = "R7";
	ACTION[12][tokens::TS_VARIABLE] = "R8";
	ACTION[13][tokens::TS_VARIABLE] = "R9";
	ACTION[14][tokens::TS_VARIABLE] = "R10";
	ACTION[15][tokens::TS_NEWLINE] = "R4";
	ACTION[16][tokens::TS_NEWLINE] = "R5";
}

void mgparser::ppeval() {
	tokens::Symbols lexeme = tokens::TS_EMPTY;
	bool reduced = false;
	while(lexeme != tokens::TS_EOF) {
		if (!reduced) {
			lexeme = lexer->lltoken().second;
		}
		reduced = false;
		int state = ss.top().state;
		ss.pop();
		if (ACTION[state][lexeme].substr(0,1) == "S") {
			ss.push(stack_symbol{lexeme});
			ss.push(stack_symbol{state});
		}
		else if(ACTION[state][lexeme].substr(0,1) == "R") {
			int gtNum = atoi(ACTION[state][lexeme].substr(1).c_str());
			pair<tokens::Symbols, int> gt = GOTO[gtNum];
			int pop_amnt = gt.second;
			int pop_cur = 0;
			while (pop_cur++ < pop_amnt) {
				ss.pop();
				reduced = true;
			}
			ss.push(stack_symbol{gtNum});
			ss.push(stack_symbol{atoi(ACTION[gtNum][gt.first].c_str())});
		}
		else if (ACTION[state][lexeme] == "ACCEPT") {
			cout << "ACCEPTED BY PARSER" << endl;
			break;
		}
	}
	cout << "parsing completed without error" << endl;
}
