#include "mgparser.h"
#include <string>
#include <iostream>

mgparser::mgparser(const char* file) {
	lexer = new mglexer(file);
	ss.push(stack_symbol{0});
	GOTO[1] = {tokens::NTS_SP, 2};
	GOTO[2] = {tokens::NTS_S, 4};
	GOTO[3] = {tokens::NTS_C, 4};
	GOTO[4] = {tokens::NTS_C, 2};
	ACTION[0][tokens::NTS_S] = "1";
	ACTION[0][tokens::NTS_C] = "2";
	ACTION[0][tokens::TS_C] = "S3";
	ACTION[0][tokens::TS_D] = "S4";
	ACTION[1][tokens::TS_EOF] = "ACCEPT";
	ACTION[2][tokens::NTS_C] = "5";
	ACTION[2][tokens::TS_C] = "S6";
	ACTION[2][tokens::TS_D] = "S7";
	ACTION[3][tokens::NTS_C] = "8";
	ACTION[3][tokens::TS_C] = "S3";
	ACTION[3][tokens::TS_D] = "S4";
	ACTION[4][tokens::TS_D] = "R3";
	ACTION[4][tokens::TS_C] = "R3";
	ACTION[5][tokens::TS_EOF] = "R1";
	ACTION[6][tokens::NTS_C] = "9";
	ACTION[6][tokens::TS_C] = "S6";
	ACTION[6][tokens::TS_D] = "S7";
	ACTION[7][tokens::TS_EOF] = "R3";
	ACTION[8][tokens::TS_D] = "R2";
	ACTION[8][tokens::TS_C] = "R2";
	ACTION[9][tokens::TS_EOF] = "R2";
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
		}
	}
}
