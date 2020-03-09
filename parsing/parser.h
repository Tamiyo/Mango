#ifndef MANGOREVISITEDCPPCLION_PARSER_H
#define MANGOREVISITEDCPPCLION_PARSER_H

#include "parse_table.h"

#include "iostream"

#include "lexer.h"
#include "string"
#include "tuple"
#include "stack"
#include "queue"


using std::string;
using std::find;
using std::tuple;
using std::get;
using std::move;

using std::stack;
using std::queue;

using grammar::token;

using std::cout;
using std::endl;

class parser {
public:
    explicit parser(queue<lexer_token> ltokens) {
        this->ltokens = move(ltokens);

        int index = 1;
        for (const auto &p : grammar::grammar) {
            for (const auto &prod : p.second) {
                indexed_grammar[index++] = tuple<int, token, vector<token>>(prod.size(), p.first, prod);
            }
        }
    }

    Mango1 *parse();

private:
    queue<lexer_token> ltokens;
    map<int, tuple<int, token, vector<token>>> indexed_grammar;
};


#endif //MANGOREVISITEDCPPCLION_PARSER_H