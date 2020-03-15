#ifndef MANGOREVISITEDCPPCLION_PARSER_H
#define MANGOREVISITEDCPPCLION_PARSER_H


#include <iostream>
#include <string>
#include <tuple>
#include <stack>
#include <queue>
#include <vector>

#include "parse_table.h"
#include "lexer.h"
#include "grammar.h"


using namespace std;
using namespace mango;

namespace mango {
    class parser {
    public:
        explicit parser(queue<lexer_token> ltokens) {
            this->ltokens = move(ltokens);

            int index = 1;
            for (const auto &p : grammar) {
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
}


#endif //MANGOREVISITEDCPPCLION_PARSER_H
