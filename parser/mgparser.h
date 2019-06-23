//
// Created by Matt on 4/25/2019.
//

#ifndef MANGO_CL_MGPARSER_H
#define MANGO_CL_MGPARSER_H


#include <stack>
#include <map>
#include <string.h>
#include <cstdlib>
#include <regex>
#include "../parse_tree/node.h"
#include "../lexer/mglexer.h"

using std::stack;
using std::string;
using std::map;
using std::pair;
using std::regex;
using std::regex_match;

class mgparser {
public:
    explicit mgparser(const char *);

    void ppeval();

private:
    // The Lexer
    mglexer *lexer;

    struct stack_symbol {
        union {
            int state;
            tokens::Symbols symbol;
        };
    };

    // The Stack for our LL parser
    stack<stack_symbol> ss;
    stack<string> strs;
    stack<node> ns;

    // The LL parsing table
    map<int, map<tokens::Symbols, string>> ACTION;
    map<int, pair<tokens::Symbols, int>> GOTO;

};


#endif //MANGO_CL_MGPARSER_H
