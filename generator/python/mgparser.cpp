#include "mgparser.h"
#include <string>
#include <iostream>

mgparser::mgparser(const char* file) {
	lexer = new mglexer(file);
	ss.push(stack_symbol{0});
	GOTO[1] = {tokens::NTS_MANGO, 2};
	GOTO[1] = {tokens::NTS_STATEMENT_SUITE, 4};
	GOTO[2] = {tokens::NTS_STATEMENT_LIST, 6};
	GOTO[3] = {tokens::NTS_STATEMENT_LIST, 2};
	GOTO[4] = {tokens::NTS_STATEMENT, 2};
	GOTO[5] = {tokens::NTS_STATEMENT_SIMPLE, 2};
	GOTO[6] = {tokens::NTS_STATEMENT_ASSIGNMENT, 6};
	ACTION[0][tokens::NTS_STATEMENT_SUITE] = "1";
	ACTION[0][tokens::NTS_STATEMENT_LIST] = "2";
	ACTION[0][tokens::NTS_STATEMENT] = "3";
	ACTION[0][tokens::NTS_STATEMENT_SIMPLE] = "4";
	ACTION[0][tokens::NTS_STATEMENT_ASSIGNMENT] = "5";
	ACTION[0][tokens::TS_IDENTIFIER] = "S6";
	ACTION[1][tokens::TS_END_OF_FILE] = "ACCEPT";
	ACTION[2][tokens::TS_NEWLINE] = "S7";
	ACTION[3][tokens::TS_NEWLINE] = "R3";
	ACTION[4][tokens::TS_NEWLINE] = "R4";
	ACTION[5][tokens::TS_NEWLINE] = "R5";
	ACTION[6][tokens::TS_EQUALS] = "S9";
	ACTION[7][tokens::TS_END_OF_FILE] = "R1";
	ACTION[8][tokens::NTS_STATEMENT_LIST] = "10";
	ACTION[8][tokens::NTS_STATEMENT] = "3";
	ACTION[8][tokens::NTS_STATEMENT_SIMPLE] = "4";
	ACTION[8][tokens::NTS_STATEMENT_ASSIGNMENT] = "5";
	ACTION[8][tokens::TS_IDENTIFIER] = "S6";
	ACTION[9][tokens::TS_TERM] = "S11";
	ACTION[10][tokens::TS_NEWLINE] = "R2";
	ACTION[11][tokens::TS_NEWLINE] = "R6";
}

void mgparser::ppeval() {
	auto token = lexer->lltoken();
	while (true) {
		auto s = ss.top();
		if (token.second >= tokens::TS_STRING && token.second <= tokens::MYSBL_END) {
			if (ACTION[s.state][token.second].substr(0, 1) == "S") {
				if (token.second == tokens::TS_TERM) { strs.push(token.first); }
				ss.push({token.second});
				ss.push({atoi(ACTION[s.state][token.second].substr(1).c_str())});
				token = lexer->lltoken();
			} else if (ACTION[s.state][token.second] == "ACCEPT") {
				node NTS_STATEMENT_SUITE = ns.top();
				ns.pop();
				ns.push(NTS_MANGO_NTS_STATEMENT_SUITE(NTS_STATEMENT_SUITE));
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
