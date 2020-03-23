#ifndef MANGO_LOX_EXPRESSION_HPP
#define MANGO_LOX_EXPRESSION_HPP

#include <memory>
#include <utility>

#include "token.hpp"

using namespace std;

namespace mango {
    class expr {
    public:
        enum expr_type {
            BINARY,
            GROUPING,
            LITERAL,
            UNARY
        };

        explicit expr(expr_type);

        virtual void debug() {
            cout << "Visited " << etype << " expr." << endl;
        };

        expr_type etype;
    };

    class binary : public expr {
    public:
        binary(unique_ptr<expr> &, const token &, unique_ptr<expr> &);

        unique_ptr<expr> left;
        token op;
        unique_ptr<expr> right;
    };

    class grouping : public expr {
    public:
        explicit grouping(unique_ptr<expr> &);

        unique_ptr<expr> expression;
    };

    class literal : public expr {
    public:
        explicit literal(const variable &);

        variable value;
    };

    class unary : public expr {
    public:
        unary(const token &, unique_ptr<expr> &);

        token op;
        unique_ptr<expr> right;
    };

    template<typename T>
    class expression_visitor {
    public:
        T visit(unique_ptr<expr> &exp) {
            switch (exp->etype) {
                case expr::expr_type::BINARY:
                    return visit_binary(exp);
                case expr::expr_type::GROUPING:
                    return visit_grouping(exp);
                case expr::expr_type::LITERAL:
                    return visit_literal(exp);
                case expr::expr_type::UNARY:
                    return visit_unary(exp);
                default:
                    break;
            }
        }

    private:
        virtual T visit_binary(unique_ptr<expr> &) = 0;

        virtual T visit_grouping(unique_ptr<expr> &) = 0;

        virtual T visit_literal(unique_ptr<expr> &) = 0;

        virtual T visit_unary(unique_ptr<expr> &) = 0;
    };
}
#endif //MANGO_LOX_EXPRESSION_HPP
