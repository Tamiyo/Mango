//
// Created by Matt on 4/16/2019.
//

#ifndef CLIONLANG_PARSER_H
#define CLIONLANG_PARSER_H

#include "../lex/mglex.h"
#include <stack>
#include <map>
#include <string>

using std::stack;
using std::string;
using std::map;

class mgparser {
public:
    explicit mgparser(const char *);

    void ppeval();

private:
    // The Lexer
    mglex *lexer;

    // The Stack for our LL parser
    stack<keywords::Symbols> ss;

    // The LL parsing table
    map<keywords::Symbols, map<keywords::Symbols, int>> table;
};


#endif //CLIONLANG_PARSER_H
