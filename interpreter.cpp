#include "interpreter.hpp"

namespace mango {

    void interpreter::interpret(unique_ptr<expr> &exp) {
        try {
            variable value = evaluate(exp);
            cout << "output: " << stringify(value) << endl;
        } catch (exception &e) {
            cout << e.what() << endl;
        }
    }

    variable interpreter::evaluate(unique_ptr<expr> &exp) {
        return visit(exp);
    }

    variable interpreter::visit_binary(unique_ptr<expr> &exp) {
        auto *b = dynamic_cast<binary *>(exp.get());

        variable left = evaluate(b->left);
        variable right = evaluate(b->right);

        return evaluate_expression(left, b->op, right);
    }

    variable interpreter::visit_grouping(unique_ptr<expr> &exp) {
        auto *g = dynamic_cast<grouping *>(exp.get());
        return evaluate(g->expression);
    }

    variable interpreter::visit_literal(unique_ptr<expr> &exp) {
        auto *l = dynamic_cast<literal *>(exp.get());
        return l->value;
    }

    variable interpreter::visit_unary(unique_ptr<expr> &exp) {
        auto *u = dynamic_cast<unary *>(exp.get());
        variable right = evaluate(u->right);

        return evaluate_expression(-1, u->op, right);
    }
}