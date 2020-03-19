#ifndef MANGO_LOX_TOKEN_HPP
#define MANGO_LOX_TOKEN_HPP

#include <string>
#include <variant>
#include <map>
#include <utility>

using namespace std;

namespace mango {

    using variable = variant<int, double, string, bool>;

    enum token_type {
        // Operator tokens
        ADD, SUB, MUL, DIV,
        IDIV, MOD, POW,

        // One or two character tokens
        EQUAL, EQUAL_EQUAL, LT, LTE,
        GT, GTE, BANG, BANG_EQUAL,

        // Control tokens
        OPEN_PAREN, CLOSE_PAREN,
        OPEN_SQUARE, CLOSE_SQUARE,
        OPEN_CURLY, CLOSE_CURLY,

        // Single character tokens
        DOT, COMMA, QUOTE,
        DQUOTE, COLON, SEMICOLON,
        UNDERSCORE, HASHTAG, DEFINITION,

        // Literal
        IDENTIFIER, STRING, NUMBER,

        // Keyword tokens
        AND, OR, IF, ELIF, ELSE, FOR, WHILE, PRINT, RETURN, TRUE, FALSE, NONE,

        END_OF_FILE,
    };

    static map<string, token_type> keywords = {
            {"and", AND},
            {"or", OR},
            {"if", IF},
            {"elif", ELIF},
            {"else", ELSE},
            {"true", TRUE},
            {"false", FALSE},
            {"for", FOR},
            {"while", WHILE},
            {"print", PRINT},
            {"return", RETURN},
            {"absent", NONE},
    };

    class token {
    public:
        token_type type;
        string lexeme;
        variable literal;
        int line;

        token(token_type, string, variable, int);
    };
}


#endif //MANGO_LOX_TOKEN_HPP
