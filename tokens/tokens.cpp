//
// Created by Matt on 4/25/2019.
//

#include "tokens.h"

tokens::tokens() {
    TOKENS = {
            {"if", TS_IF},
            {"elif", TS_ELIF},
            {"else", TS_ELSE},

            {"{", TS_LCB},
            {"}", TS_RCB},
            {"(", TS_LPAREN},
            {")", TS_RPAREN},

            {"=", TS_OPERATOR_EQUALS},
            {"+", TS_OPERATOR_ADD},
            {"-", TS_OPERATOR_SUB},
            {"*", TS_OPERATOR_MUL},
            {"/", TS_OPERATOR_DIV},
            {"^", TS_OPERATOR_EXP},

            {"<", TS_OPERATOR_LT},
            {">", TS_OPERATOR_GT},
            {">=", TS_OPERATOR_GTE},
            {"<=", TS_OPERATOR_LTE},
            {"==", TS_OPERATOR_DOUBLE_EQUALS},
            {"===", TS_OPERATOR_TRIPLE_EQUALS},

            {"@", TS_DEFINE},

            {"", TS_EMPTY},
            {":", TS_COLON},
            {",", TS_COMMA},
            {" ", TS_SPACE},
            {"\n", TS_NEWLINE},
            {"$", TS_EOF},
    };

    TYPES = {
            {0, TS_IDENTIFIER},
            {1, TS_TERM},
            {2, TS_TERM},
            {3, TS_TERM},
            {4, NTS_OPERATOR}
    };

}