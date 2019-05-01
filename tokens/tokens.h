//
// Created by Matt on 4/25/2019.
//

#ifndef MANGO_CL_TOKENS_H
#define MANGO_CL_TOKENS_H

#include<map>
#include<string>

class tokens {
public:
    tokens();

    enum Symbols {
        // Terminal Symbols

        // "Types", denoted as TS_VARIABLE for now in grammer.mg
                TS_STRING,
        TS_IDENT,
        TS_FLOAT,
        TS_INT,
        TS_VARIABLE,


        // Control Symbols
                TS_IF,
        TS_ELSEIF,
        TS_ELSE,

        // Grouping Symbols
                TS_LCB,
        TS_RCB,
        TS_LPAREN,
        TS_RPAREN,

        // Operator Symbols
                TS_EQUALS,
        TS_PLUS,
        TS_MINUS,
        TS_MUL,
        TS_DIV,
        TS_EXP,

        // Comparison Symbols
                TS_LT,
        TS_LTE,
        TS_GT,
        TS_GTE,
        TS_EQUIV,
        TS_TEQUIV,

        // Single Comparison Symbols
                TS_NEG,
        TS_NOTNULL,

        // Other Symbols
                TS_SPACE,
        TS_NEWLINE,
        TS_EMPTY,
        TS_EOF,

        // Non-Terminal Symbols
                NTS_MANGO,
        NTS_STMTS,
        NTS_STMT,
        NTS_ASSIGN,

        NTS_EXPR,
        NTS_EXPR_P,
        NTS_EXPR2,
        NTS_EXPR2_P,
        NTS_EXPR3,

        NTS_CTRL,
        NTS_CTRL_IF,
        NTS_CTRL_ELSEIF,
        NTS_CTRL_ELSE,
        NTS_COND,
        NTS_OPERATOR,
        NTS_COND_OPERATOR,
        NTS_COND_S_OPERATOR,

        // Personal Iterative Symbol
                TS_$,
    };

    std::map<std::string, Symbols> TOKENS;
    std::map<int, Symbols> TYPES;
};


#endif //MANGO_CL_TOKENS_H
