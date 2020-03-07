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
        literal,

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
            {literal,                  "literal"},
            {eof,                      "eof"},
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
        return token::Mango;
    }

    inline token end(token x) {
        return token(int(token::eof) + 1);
    }


    static token start_symbol = token::Mango;

    static map<const string, token> keyword_token_map = {
            {"print",  token::kw_print},
            {"if",     token::kw_if},
            {"elif",   token::kw_elif},
            {"else",   token::kw_else},
            {"for",    token::kw_for},
            {"while",  token::kw_while},
            {"break",  token::kw_break},
            {"return", token::kw_return},
            {"@",      token::kw_func},
    };

    static map<const string, token> operator_token_map = {
            {"+",  token::op_plus},
            {"-",  token::op_minus},
            {"*",  token::op_mult},
            {"/",  token::op_div},
            {"//", token::op_idiv},
            {"%",  token::op_mod},
            {"^",  token::op_pow},
            {"<",  token::cop_lt},
            {"<=", token::cop_lte},
            {">",  token::cop_gt},
            {">=", token::cop_gte},
            {"==", token::cop_equals},
            {"=",  token::op_equals},
            {"!",  token::logical_not},
            {"&&", token::logical_and},
            {"||", token::logical_or},
            {"(",  token::open_paren},
            {")",  token::close_paren},
            {"{",  token::open_curly},
            {"}",  token::close_curly},
            {"[",  token::open_square},
            {"]",  token::close_square},
            {":",  token::colon},
            {";",  token::semicolon},
            {".",  token::period},
            {",",  token::comma},
            {"'",  token::squote},
            {"\"", token::dquote},
            {"_",  token::underscore},
            {"#",  token::hashtag},
            {"\n", token::newline},
            {"$",  token::eof},
    };

    static map<token, vector<vector<token>>> grammar = {
            {
                    token::Mango,                    {
                                                             {token::StatementSuite,           token::eof}
                                                     }
            },
            {
                    token::StatementSuite,           {
                                                             {token::StatementList}
                                                     }
            },
            {
                    token::StatementList,            {
                                                             {token::Statement,                token::newline,    token::StatementList},
                                                             {token::Statement,                token::newline}
                                                     }
            },
            {
                    token::Statement,                {
                                                             {token::Print},
                                                             {token::Expression}
                                                     }
            },
            {
                    token::Print,                    {
                                                             {token::kw_print,                 token::open_paren, token::Expression, token::close_paren}
                                                     }
            },
            {
                    token::Expression,               {
                                                             {token::MultiplicativeExpression, token::op_plus,    token::Expression},
                                                             {token::MultiplicativeExpression, token::op_minus, token::Expression},
                                                             {token::MultiplicativeExpression},
                                                     }
            },
            {
                    token::MultiplicativeExpression, {
                                                             {token::BaseExpression,           token::op_mult,    token::MultiplicativeExpression},
                                                             {token::BaseExpression,           token::op_div,   token::MultiplicativeExpression},
                                                             {token::BaseExpression, token::op_idiv, token::MultiplicativeExpression},
                                                             {token::BaseExpression, token::op_mod, token::MultiplicativeExpression},
                                                             {token::BaseExpression, token::op_pow, token::MultiplicativeExpression},
                                                             {token::BaseExpression},
                                                     }
            },
            {
                    token::BaseExpression,           {
                                                             {token::open_paren,               token::Expression, token::close_paren},
                                                             {token::identifier},
                                                             {token::literal}
                                                     }
            },
    };

    static vector<token> nonterminals = {
            Mango,
            StatementSuite,
            StatementList,
            Statement,
            Print,
            Expression,
            MultiplicativeExpression,
            BaseExpression
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
            literal,

            eof
    };
}

#endif //MANGOREVISITEDCPPCLION_GRAMMAR_H

