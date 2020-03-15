#ifndef MANGOREVISITEDCPPCLION_LEXER_H
#define MANGOREVISITEDCPPCLION_LEXER_H

#include "string"
#include "fstream"
#include "algorithm"
#include "queue"

#include "grammar.h"

using namespace std;

namespace mango {
    class lexer_token {
    public:
        lexer_token(token t, string s) {
            tok = t;
            val = move(s);
        }

        friend ostream &operator<<(ostream &os, const lexer_token &ltok) {
            os << "[" << token_map[ltok.tok] << ", " << ((ltok.val == "\n") ? "newline" : ltok.val) << "]";
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
}

#endif //MANGOREVISITEDCPPCLION_LEXER_H
