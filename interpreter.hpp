#ifndef MANGO_LOX_INTERPRETER_HPP
#define MANGO_LOX_INTERPRETER_HPP

#include "token.hpp"
#include "eval.hpp"
#include "expression.hpp"

namespace mango {
    class interpreter : public expression_visitor<variable> {
    public:

        void interpret(unique_ptr<expr>&);

        variable evaluate(unique_ptr<expr> &);

        variable visit_binary(unique_ptr<expr> &) override;

        variable visit_grouping(unique_ptr<expr> &) override;

        variable visit_literal(unique_ptr<expr> &) override;

        variable visit_unary(unique_ptr<expr> &) override;
    };
}


#endif //MANGO_LOX_INTERPRETER_HPP
