#include "expression.hpp"

namespace mango {

    expr::expr(expr::expr_type etype) {
        this->etype = etype;
    }

    binary::binary(unique_ptr<expr> &left, const token &op, unique_ptr<expr> &right) : expr(BINARY), op(op) {
        this->left = move(left);
        this->right = move(right);
    }


    grouping::grouping(unique_ptr<expr> &expression) : expr(GROUPING) {
        this->expression = move(expression);
    }


    literal::literal(const variable &value) : expr(LITERAL) {
        this->value = value;
    }

    unary::unary(const token &op, unique_ptr<expr> &right) : expr(UNARY), op(op) {
        this->right = move(right);
    }

}
