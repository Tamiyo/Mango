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
        TS_STRING,
        TS_IDENT,
        TS_FLOAT,
        TS_INT,

        TS_PLUS,
        TS_MINUS,
        TS_MUL,
        TS_DIV,
        TS_EQU,
        TS_GT,
        TS_GTE,
        TS_LT,
        TS_LTE,
        TS_ENDL,
        TS_EOF,
        TS_SPACE,

        // Non-Terminal Symbols
        NTS_MANGO,
        NTS_STMTS,
        NTS_STMT,
        NTS_ASSIGN,
        NTS_EXPR
    };

    map<string, Symbols> KEYWORDS;
    map<int, Symbols> TYPES;

};


#endif //CLIONLANG_KEYWORDS_H
