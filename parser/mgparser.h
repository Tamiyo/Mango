//
// Created by Matt on 4/25/2019.
//

#ifndef MANGO_CL_MGPARSER_H
#define MANGO_CL_MGPARSER_H


#include <stack>
#include <map>
#include <string.h>
#include <regex>
#include "../lexer/mglexer.h"

using std::stack;
using std::string;
using std::map;
using std::regex;
using std::regex_match;

class mgparser {
public:
    explicit mgparser(const char *);

    void ppeval();

private:
    // The Lexer
    mglexer *lexer;

    // The Stack for our LL parser
    stack<tokens::Symbols> ss;

    // The LL parsing table
    map<tokens::Symbols, map<tokens::Symbols, int>> table;

};


#endif //MANGO_CL_MGPARSER_H
