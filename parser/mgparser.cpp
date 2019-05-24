#include "mgparser.h"
#include <string>
#include <iostream>

mgparser::mgparser(const char* file) {
	lexer = new mglexer(file);
	ss.push(stack_symbol{0});
	GOTO[1] = {tokens::NTS_MANGO, 2};
	GOTO[1] = {tokens::NTS_STMTS, 4};
	GOTO[2] = {tokens::NTS_STMT, 2};
	GOTO[3] = {tokens::NTS_STMT, 2};
	GOTO[4] = {tokens::NTS_ASSIGN, 6};
	GOTO[5] = {tokens::NTS_EXPR, 6};
	GOTO[6] = {tokens::NTS_OPERATOR, 2};
	GOTO[7] = {tokens::NTS_OPERATOR, 2};
	GOTO[8] = {tokens::NTS_OPERATOR, 2};
	GOTO[9] = {tokens::NTS_OPERATOR, 2};
	GOTO[10] = {tokens::NTS_OPERATOR, 2};
	ACTION[0][tokens::NTS_STMTS] = "1";
	ACTION[0][tokens::NTS_STMT] = "2";
	ACTION[0][tokens::NTS_ASSIGN] = "3";
	ACTION[0][tokens::NTS_EXPR] = "4";
	ACTION[0][tokens::TS_IDENT] = "S5";
	ACTION[0][tokens::TS_VARIABLE] = "S6";
	ACTION[1][tokens::TS_EOF] = "ACCEPT";
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
	auto token = lexer->lltoken();
	while (true) {
		auto s = ss.top();
		if (token.second >= tokens::TS_STRING && token.second <= tokens::MYSBL_END) {
			if (ACTION[s.state][token.second].substr(0, 1) == "S") {
				ss.push({token.second});
				ss.push({atoi(ACTION[s.state][token.second].substr(1).c_str())});
				token = lexer->lltoken();
			} else if (ACTION[s.state][token.second] == "ACCEPT") {
				cout << "Parse Accepted" << endl;
				break;
			} else if (ACTION[s.state][token.second].substr(0, 1) == "R") {
				auto g = GOTO[atoi(ACTION[s.state][token.second].substr(1).c_str())];
				for (int i = 0; i < g.second; i++) { ss.pop(); }
				s = ss.top();
				ss.push(stack_symbol{g.first});
				ss.push(stack_symbol{atoi(ACTION[s.state][g.first].c_str())});
			} else {
				cout << "Parse Error" << endl;
				break;
			}
		} else {
			token = lexer->lltoken();
		}
	}
}
