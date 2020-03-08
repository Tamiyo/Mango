#ifndef MANGOREVISITEDCPPCLION_GRAMMAR_H
#define MANGOREVISITEDCPPCLION_GRAMMAR_H

#include "map"
#include "string"
#include "vector"

using std::map;
using std::string;
using std::vector;

namespace grammar {
    enum token {
        Mango,
        StatementSuite,
        StatementList,
        Statement,
        Print,
        Expression,
        MultiplicativeExpression,
        BaseExpression,
        Literal,

        op_plus,
        op_minus,
        op_mult,
        op_div,
        op_idiv,
        op_mod,
        op_pow,

        op_equals,

        cop_lt,
        cop_lte,
        cop_gt,
        cop_gte,
        cop_equals,

        logical_not,
        logical_and,
        logical_or,

        kw_print,
        kw_if,
        kw_elif,
        kw_else,
        kw_for,
        kw_while,
        kw_break,
        kw_return,
        kw_func,

        open_paren,
        close_paren,
        open_curly,
        close_curly,
        open_square,
        close_square,

        colon,
        semicolon,
        period,
        comma,
        squote,
        dquote,
        underscore,
        hashtag,
        epsilon,
        newline,

        identifier,

        type_string,
        type_double,
        type_int,

        eof
    };

    static vector<token> nonterminals = {
            Mango,
            StatementSuite,
            StatementList,
            Statement,
            Print,
            Expression,
            MultiplicativeExpression,
            BaseExpression,
            Literal,
    };

    static vector<token> terminals = {
            op_plus,
            op_minus,
            op_mult,
            op_div,
            op_idiv,
            op_mod,
            op_pow,

            op_equals,

            cop_lt,
            cop_lte,
            cop_gt,
            cop_gte,
            cop_equals,

            logical_not,
            logical_and,
            logical_or,

            kw_print,
            kw_if,
            kw_elif,
            kw_else,
            kw_for,
            kw_while,
            kw_break,
            kw_return,
            kw_func,

            open_paren,
            close_paren,
            open_curly,
            close_curly,
            open_square,
            close_square,

            colon,
            semicolon,
            period,
            comma,
            squote,
            dquote,
            underscore,
            hashtag,
            epsilon,
            newline,

            identifier,
            type_string,
            type_double,
            type_int,

            eof
    };

    static map<token, string> token_map = {
            {Mango,                    "Mango"},
            {StatementSuite,           "StatementSuite"},
            {StatementList,            "StatementList"},
            {Statement,                "Statement"},
            {Print,                    "Print"},
            {Expression,               "Expression"},
            {MultiplicativeExpression, "MultiplicativeExpression"},
            {BaseExpression,           "BaseExpression"},
            {Literal,                  "Literal"},
            {op_plus,                  "op_plus"},
            {op_minus,                 "op_minus"},
            {op_mult,                  "op_mult"},
            {op_div,                   "op_div"},
            {op_idiv,                  "op_idiv"},
            {op_mod,                   "op_mod"},
            {op_pow,                   "op_pow"},
            {op_equals,                "op_equals"},
            {cop_lt,                   "cop_lt"},
            {cop_lte,                  "cop_lte"},
            {cop_gt,                   "cop_gt"},
            {cop_gte,                  "cop_gte"},
            {cop_equals,               "cop_equals"},
            {logical_not,              "logical_not"},
            {logical_and,              "logical_and"},
            {logical_or,               "logical_or"},
            {kw_print,                 "kw_print"},
            {kw_if,                    "kw_if"},
            {kw_elif,                  "kw_elif"},
            {kw_else,                  "kw_else"},
            {kw_for,                   "kw_for"},
            {kw_while,                 "kw_while"},
            {kw_break,                 "kw_break"},
            {kw_return,                "kw_return"},
            {kw_func,                  "kw_func"},
            {open_paren,               "open_paren"},
            {close_paren,              "close_paren"},
            {open_curly,               "open_curly"},
            {close_curly,              "close_curly"},
            {open_square,              "open_square"},
            {close_square,             "close_square"},
            {colon,                    "colon"},
            {semicolon,                "semicolon"},
            {period,                   "period"},
            {comma,                    "comma"},
            {squote,                   "squote"},
            {dquote,                   "dquote"},
            {underscore,               "underscore"},
            {hashtag,                  "hashtag"},
            {epsilon,                  "epsilon"},
            {newline,                  "newline"},
            {identifier,               "identifier"},
            {type_string,              "type_string"},
            {type_int,                 "type_int"},
            {type_double,              "type_double"},
            {eof,                      "eof"},
    };

    static map<const string, token> keyword_token_map = {
            {"print",  kw_print},
            {"if",     kw_if},
            {"elif",   kw_elif},
            {"else",   kw_else},
            {"for",    kw_for},
            {"while",  kw_while},
            {"break",  kw_break},
            {"return", kw_return},
            {"@",      kw_func},
    };

    static map<const string, token> operator_token_map = {
            {"+",  op_plus},
            {"-",  op_minus},
            {"*",  op_mult},
            {"/",  op_div},
            {"//", op_idiv},
            {"%",  op_mod},
            {"^",  op_pow},
            {"<",  cop_lt},
            {"<=", cop_lte},
            {">",  cop_gt},
            {">=", cop_gte},
            {"==", cop_equals},
            {"=",  op_equals},
            {"!",  logical_not},
            {"&&", logical_and},
            {"||", logical_or},
            {"(",  open_paren},
            {")",  close_paren},
            {"{",  open_curly},
            {"}",  close_curly},
            {"[",  open_square},
            {"]",  close_square},
            {":",  colon},
            {";",  semicolon},
            {".",  period},
            {",",  comma},
            {"'",  squote},
            {"\"", dquote},
            {"_",  underscore},
            {"#",  hashtag},
            {"\n", newline},
            {"$",  eof},
    };

    inline token operator++(token &x) {
        return x = static_cast<token>((static_cast<int>(x) + 1));
    }

    inline token operator*(token x) {
        return x;
    }

    inline bool operator==(token &t1, token &t2) {
        return (int) t1 == (int) t2;
    }

    inline token begin(token x) {
        return Mango;
    }

    inline token end(token x) {
        return token(int(eof) + 1);
    }


    static token start_symbol = Mango;

    static map<token, vector<vector<token>>> grammar = {
            {
                    Mango,                    {
                                                      {StatementSuite,           eof}
                                              }
            },
            {
                    StatementSuite,           {
                                                      {StatementList}
                                              }
            },
            {
                    StatementList,            {
                                                      {Statement,                newline,    StatementList},
                                                      {Statement,                newline}
                                              }
            },
            {
                    Statement,                {
                                                      {Print},
                                                      {Expression}
                                              }
            },
            {
                    Print,                    {
                                                      {kw_print,                 open_paren, Expression, close_paren}
                                              }
            },
            {
                    Expression,               {
                                                      {MultiplicativeExpression, op_plus,    Expression},
                                                      {MultiplicativeExpression, op_minus, Expression},
                                                      {MultiplicativeExpression},
                                              }
            },
            {
                    MultiplicativeExpression, {
                                                      {BaseExpression,           op_mult,    MultiplicativeExpression},
                                                      {BaseExpression,           op_div,   MultiplicativeExpression},
                                                      {BaseExpression, op_idiv, MultiplicativeExpression},
                                                      {BaseExpression, op_mod, MultiplicativeExpression},
                                                      {BaseExpression, op_pow, MultiplicativeExpression},
                                                      {BaseExpression},
                                              }
            },
            {
                    BaseExpression,           {
                                                      {open_paren,               Expression, close_paren},
                                                      {identifier},
                                                      {Literal}
                                              }
            },
            {
                    Literal,                  {
                                                      {type_double},
                                                      {type_int},
                                                      {type_string},
                                              }
            },
    };
}

#endif //MANGOREVISITEDCPPCLION_GRAMMAR_H

