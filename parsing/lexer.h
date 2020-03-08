#ifndef MANGOREVISITEDCPPCLION_LEXER_H
#define MANGOREVISITEDCPPCLION_LEXER_H

#include "string"
#include "fstream"
#include "algorithm"
#include "queue"

#include "../core/grammar.h"

using std::queue;

using std::string;
using std::find;

using std::ifstream;
using std::istreambuf_iterator;

using grammar::token;
using grammar::keyword_token_map;
using grammar::operator_token_map;

class lexer_token {
public:
    lexer_token(token t, string s) {
        tok = t;
        val = std::move(s);
    }

    friend std::ostream &operator<<(std::ostream &os, const lexer_token &ltok) {
        os << "[" << grammar::token_map[ltok.tok] << ", " << ltok.val << "]";
        return os;
    }

    token tok;
    string val;
};


class lexer {
public:
    lexer();

    void lex();

    queue<lexer_token> tokens;
private:
    string contents;
};


#endif //MANGOREVISITEDCPPCLION_LEXER_H
