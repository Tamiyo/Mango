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
    table[keywords::NTS_MANGO][keywords::TS_IDENT] = 1;
    table[keywords::NTS_MANGO][keywords::TS_PLUS] = 1;
    table[keywords::NTS_MANGO][keywords::TS_MINUS] = 1;
    table[keywords::NTS_MANGO][keywords::TS_MUL] = 1;
    table[keywords::NTS_MANGO][keywords::TS_DIV] = 1;

    table[keywords::NTS_STMTS][keywords::TS_IDENT] = 2;
    table[keywords::NTS_STMTS][keywords::TS_PLUS] = 2;
    table[keywords::NTS_STMTS][keywords::TS_MINUS] = 2;
    table[keywords::NTS_STMTS][keywords::TS_MUL] = 2;
    table[keywords::NTS_STMTS][keywords::TS_DIV] = 2;

    table[keywords::NTS_STMT][keywords::TS_IDENT] = 3;
    table[keywords::NTS_STMT][keywords::TS_PLUS] = 4;
    table[keywords::NTS_STMT][keywords::TS_MINUS] = 4;
    table[keywords::NTS_STMT][keywords::TS_MUL] = 4;
    table[keywords::NTS_STMT][keywords::TS_DIV] = 4;

    table[keywords::NTS_ASSIGN][keywords::TS_IDENT] = 5;

    table[keywords::NTS_EXPR][keywords::TS_IDENT] = 6;
    table[keywords::NTS_EXPR][keywords::TS_PLUS] = 7;
    table[keywords::NTS_EXPR][keywords::TS_MINUS] = 8;
    table[keywords::NTS_EXPR][keywords::TS_MUL] = 9;
    table[keywords::NTS_EXPR][keywords::TS_DIV] = 10;

}

void mgparser::ppeval() {
    while (!ss.empty()) {
        pair<const char *, keywords::Symbols> token = lexer->lltoken();
        if (token.second != keywords::TS_SPACE) {
            cout << token.first << endl;
            if (token.second == ss.top()) {
                cout << "Matched Symbols: " << token.first << endl;
                ss.pop();
            } else {
                cout << "Rule " << table[ss.top()][token.second] << endl;
                switch (table[ss.top()][token.second]) {
                    case 1:
                        ss.pop();
                        ss.push(keywords::NTS_STMTS);
                        break;
                    case 2:
                        ss.pop();
                        ss.push(keywords::NTS_STMT);
                        ss.push(keywords::TS_ENDL);
                        break;
                    case 3:
                        ss.pop();
                        ss.push(keywords::NTS_ASSIGN);
                        break;
                    case 4:
                        ss.pop();
                        ss.push(keywords::NTS_EXPR);
                        break;
                    case 5:
                        ss.pop();
                        ss.push(keywords::TS_IDENT);
                        ss.push(keywords::TS_EQU);
                        ss.push(keywords::NTS_EXPR);
                        break;
                    case 6:
                        ss.pop();
                        ss.push(keywords::TS_INT);
                        break;
                    case 7:
                        ss.pop();
                        ss.push(keywords::TS_PLUS);
                        ss.push(keywords::NTS_EXPR);
                        ss.push(keywords::NTS_EXPR);
                        break;
                    case 8:
                        ss.pop();
                        ss.push(keywords::TS_MINUS);
                        ss.push(keywords::NTS_EXPR);
                        ss.push(keywords::NTS_EXPR);
                        break;
                    case 9:
                        ss.pop();
                        ss.push(keywords::TS_MUL);
                        ss.push(keywords::NTS_EXPR);
                        ss.push(keywords::NTS_EXPR);
                        break;
                    case 10:
                        ss.pop();
                        ss.push(keywords::TS_DIV);
                        ss.push(keywords::NTS_EXPR);
                        ss.push(keywords::NTS_EXPR);
                        break;
                    default:
                        cout << "parsing table defaulted" << endl;
                        return;

                }
            }
        }
    }
    cout << "finished parsing" << endl;
}
