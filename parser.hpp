//
// Created by zoolo on 3/19/2020.
//

#ifndef MANGO_LOX_PARSER_HPP
#define MANGO_LOX_PARSER_HPP

#include <string>
#include <vector>
#include <utility>

#include "error.hpp"
#include "token.hpp"
#include "expression.hpp"

using namespace std;

namespace mango {
    class parser {
    public:
        explicit parser(vector<token>);

        unique_ptr<expr> parse() {
            try {
                return expression();
            }
            catch (parse_error &error) {
                return nullptr;
            }
        }

    private:
        class parse_error : public exception {
        };

        template<typename ... Args>
        bool match(Args &&...);

        bool check(token_type);

        token advance();

        token peek();

        bool is_at_end();

        token previous();

        token consume(token_type, const string &);

        parse_error error(const token &, const string &);

        void synchronize();

        unique_ptr<expr> expression();

        unique_ptr<expr> equality();

        unique_ptr<expr> comparison();

        unique_ptr<expr> addition();

        unique_ptr<expr> multiplication();

        unique_ptr<expr> unary();

        unique_ptr<expr> primary();

        vector<token> tokens;
        int current = 0;
    };
}

#endif //MANGO_LOX_PARSER_HPP
