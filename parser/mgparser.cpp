#include "mgparser.h"

mgparser::mgparser(const char* body) {
	lexer = new mglex(body);
	ss.push(keywords::TS_EOF);
	ss.push(keywords::NTS_MANGO);
	table[keywords::NTS_ASSIGN][keywords::TS_IDENT] = 5;
	table[keywords::NTS_COND_OPERATOR][keywords::TS_LT] = 14;
	table[keywords::NTS_COND_OPERATOR][keywords::TS_LTE] = 15;
	table[keywords::NTS_COND_OPERATOR][keywords::TS_GT] = 16;
	table[keywords::NTS_COND_OPERATOR][keywords::TS_GTE] = 17;
	table[keywords::NTS_COND_OPERATOR][keywords::TS_EQUIV] = 18;
	table[keywords::NTS_COND_OPERATOR][keywords::TS_TEQUIV] = 19;
	table[keywords::NTS_COND_S_OPERATOR][keywords::TS_NEG] = 20;
	table[keywords::NTS_COND_S_OPERATOR][keywords::TS_NOTNULL] = 21;
	table[keywords::NTS_EXPR][keywords::TS_IDENT] = 6;
	table[keywords::NTS_EXPR][keywords::TS_FLOAT] = 6;
	table[keywords::NTS_EXPR][keywords::TS_INT] = 6;
	table[keywords::NTS_EXPR][keywords::TS_STRING] = 6;
	table[keywords::NTS_EXPR_A][keywords::TS_EXP] = 7;
	table[keywords::NTS_EXPR_A][keywords::TS_DIV] = 7;
	table[keywords::NTS_EXPR_A][keywords::TS_MUL] = 7;
	table[keywords::NTS_EXPR_A][keywords::TS_MINUS] = 7;
	table[keywords::NTS_EXPR_A][keywords::TS_PLUS] = 7;
	table[keywords::NTS_EXPR_A][keywords::TS_NEWLINE] = 8;
	table[keywords::NTS_EXPR_A][keywords::TS_EXP] = 8;
	table[keywords::NTS_EXPR_A][keywords::TS_DIV] = 8;
	table[keywords::NTS_EXPR_A][keywords::TS_MUL] = 8;
	table[keywords::NTS_EXPR_A][keywords::TS_MINUS] = 8;
	table[keywords::NTS_EXPR_A][keywords::TS_PLUS] = 8;
	table[keywords::NTS_EXPR_A][keywords::TS_EMPTY] = 8;
	table[keywords::NTS_MANGO][keywords::TS_IDENT] = 1;
	table[keywords::NTS_MANGO][keywords::TS_FLOAT] = 1;
	table[keywords::NTS_MANGO][keywords::TS_INT] = 1;
	table[keywords::NTS_MANGO][keywords::TS_STRING] = 1;
	table[keywords::NTS_MANGO][keywords::TS_IDENT] = 1;
	table[keywords::NTS_OPERATOR][keywords::TS_PLUS] = 9;
	table[keywords::NTS_OPERATOR][keywords::TS_MINUS] = 10;
	table[keywords::NTS_OPERATOR][keywords::TS_MUL] = 11;
	table[keywords::NTS_OPERATOR][keywords::TS_DIV] = 12;
	table[keywords::NTS_OPERATOR][keywords::TS_EXP] = 13;
	table[keywords::NTS_STMT][keywords::TS_IDENT] = 3;
	table[keywords::NTS_STMT][keywords::TS_IDENT] = 4;
	table[keywords::NTS_STMT][keywords::TS_FLOAT] = 4;
	table[keywords::NTS_STMT][keywords::TS_INT] = 4;
	table[keywords::NTS_STMT][keywords::TS_STRING] = 4;
	table[keywords::NTS_STMTS][keywords::TS_IDENT] = 2;
	table[keywords::NTS_STMTS][keywords::TS_FLOAT] = 2;
	table[keywords::NTS_STMTS][keywords::TS_INT] = 2;
	table[keywords::NTS_STMTS][keywords::TS_STRING] = 2;
	table[keywords::NTS_STMTS][keywords::TS_IDENT] = 2;
	table[keywords::NTS_VARIABLE][keywords::TS_STRING] = 22;
	table[keywords::NTS_VARIABLE][keywords::TS_INT] = 23;
	table[keywords::NTS_VARIABLE][keywords::TS_FLOAT] = 24;
	table[keywords::NTS_VARIABLE][keywords::TS_IDENT] = 25;
	ss.push(keywords::TS_EOF);
	ss.push(keywords::NTS_MANGO);
}


void mgparser::ppeval() {
std::pair<const char*, keywords::Symbols> token = lexer->lltoken();
	while(ss.size() > 0) {
		cout << "Found a: " << token.first << endl;
		if(token.second == ss.top()) {
		token = lexer->lltoken();
			cout << "Matched symbols : " << token.first << endl;
			ss.pop();
		}
		else {
			cout << "Rule " << table[ss.top()][token.second] << endl;
			switch(table[ss.top()][token.second]) {
				case 5:
					ss.pop();
					ss.push(keywords::TS_IDENT);
					ss.push(keywords::TS_EQUALS);
					ss.push(keywords::NTS_EXPR);
					break;
				case 14:
					ss.pop();
					ss.push(keywords::TS_LT);
					break;
				case 15:
					ss.pop();
					ss.push(keywords::TS_LTE);
					break;
				case 16:
					ss.pop();
					ss.push(keywords::TS_GT);
					break;
				case 17:
					ss.pop();
					ss.push(keywords::TS_GTE);
					break;
				case 18:
					ss.pop();
					ss.push(keywords::TS_EQUIV);
					break;
				case 19:
					ss.pop();
					ss.push(keywords::TS_TEQUIV);
					break;
				case 20:
					ss.pop();
					ss.push(keywords::TS_NEG);
					break;
				case 21:
					ss.pop();
					ss.push(keywords::TS_NOTNULL);
					break;
				case 6:
					ss.pop();
					ss.push(keywords::NTS_VARIABLE);
					ss.push(keywords::NTS_EXPR_A);
					break;
				case 7:
					ss.pop();
					ss.push(keywords::NTS_OPERATOR);
					ss.push(keywords::NTS_EXPR);
					ss.push(keywords::NTS_EXPR_A);
					break;
				case 8:
					ss.pop();
					ss.push(keywords::TS_EMPTY);
					break;
				case 1:
					ss.pop();
					ss.push(keywords::NTS_STMTS);
					break;
				case 9:
					ss.pop();
					ss.push(keywords::TS_PLUS);
					break;
				case 10:
					ss.pop();
					ss.push(keywords::TS_MINUS);
					break;
				case 11:
					ss.pop();
					ss.push(keywords::TS_MUL);
					break;
				case 12:
					ss.pop();
					ss.push(keywords::TS_DIV);
					break;
				case 13:
					ss.pop();
					ss.push(keywords::TS_EXP);
					break;
				case 3:
					ss.pop();
					ss.push(keywords::NTS_ASSIGN);
					break;
				case 4:
					ss.pop();
					ss.push(keywords::NTS_EXPR);
					break;
				case 2:
					ss.pop();
					ss.push(keywords::NTS_STMT);
					ss.push(keywords::TS_NEWLINE);
					break;
				case 22:
					ss.pop();
					ss.push(keywords::TS_STRING);
					break;
				case 23:
					ss.pop();
					ss.push(keywords::TS_INT);
					break;
				case 24:
					ss.pop();
					ss.push(keywords::TS_FLOAT);
					break;
				case 25:
					ss.pop();
					ss.push(keywords::TS_IDENT);
					break;
				default:
					cout << "parsing table defaulted" << endl;
					return;
			}
		}
	}
cout << "finished parsing" << endl;
return;
}