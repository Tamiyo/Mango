#include "parser.hpp"

namespace mango {

    parser::parser(vector<token> tokens) : tokens(move(tokens)) {}

    template<typename... Args>
    bool parser::match(Args &&... args) {
        vector<token_type> types = {args...};
        for (const auto &type : types) {
            if (check(type)) {
                advance();
                return true;
            }
        }
        return false;
    }


    bool parser::check(token_type type) {
        if (is_at_end()) return false;
        return peek().type == type;
    }


    token parser::advance() {
        if (!is_at_end()) current++;
        return previous();
    }


    token parser::peek() {
        return tokens[current];
    }


    bool parser::is_at_end() {
        return peek().type == END_OF_FILE;
    }


    token parser::previous() {
        return tokens[current - 1];
    }


    token parser::consume(token_type type, const string &message) {
        if (check(type)) return advance();
        throw runtime_error(message);
    }


    typename parser::parse_error parser::error(const token &token, const string &message) {
        mango::error(token, message);
        return parse_error();
    }


    void parser::synchronize() {
        advance();
        while (!is_at_end()) {
            if (previous().type == SEMICOLON) return;

            switch (peek().type) {
                case FOR:
                case WHILE:
                case IF:
                case PRINT:
                case RETURN:
                case DEFINITION:
                    return;
            }

            advance();
        }
    }


    unique_ptr<expr> parser::expression() {
        return equality();
    }

    unique_ptr<expr> parser::equality() {
        unique_ptr<expr> exp = comparison();

        while (match(BANG_EQUAL, EQUAL_EQUAL)) {
            token op = previous();
            unique_ptr<expr> right = comparison();
            exp = unique_ptr<expr>(new binary(exp, op, right));
        }

        return exp;
    }

    unique_ptr<expr> parser::comparison() {
        unique_ptr<expr> exp = addition();

        while (match(LT, GT, GTE, LTE)) {
            token op = previous();
            unique_ptr<expr> right = addition();
            exp = unique_ptr<expr>(new binary(exp, op, right));
        }

        return exp;
    }


    unique_ptr<expr> parser::addition() {
        unique_ptr<expr> exp = multiplication();

        while (match(ADD, SUB)) {
            token op = previous();
            unique_ptr<expr> right = multiplication();
            exp = unique_ptr<expr>(new binary(exp, op, right));
        }
        return exp;
    }


    unique_ptr<expr> parser::multiplication() {
        unique_ptr<expr> exp = unary();

        while (match(MUL, DIV, IDIV, MOD, POW)) {
            token op = previous();
            unique_ptr<expr> right = unary();
            exp = unique_ptr<expr>(new binary(exp, op, right));
        }
        return exp;
    }


    unique_ptr<expr> parser::unary() {
        if (match(BANG, SUB)) {
            token op = previous();
            unique_ptr<expr> right = unary();
            return unique_ptr<expr>(new mango::unary(op, right));
        }
        return primary();
    }


    unique_ptr<expr> parser::primary() {
        if (match(FALSE)) return unique_ptr<expr>(new literal(false));
        if (match(TRUE)) return unique_ptr<expr>(new literal(true));
        if (match(NONE)) return unique_ptr<expr>(new literal(nullptr));

        if (match(NUMBER, STRING)) {
            return unique_ptr<expr>(new literal(previous().literal));
        }

        if (match(OPEN_PAREN)) {
            unique_ptr<expr> exp = expression();
            consume(CLOSE_PAREN, "Expect ')' after expression.");
            return unique_ptr<expr>(new grouping(exp));
        }

        throw error(peek(), "Expected expression.");
    }
}
