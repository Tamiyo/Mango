#include "interpreter.h"

namespace mango {
    void Interpreter::visit(Identifier *n) {
        context.add(variable(n->f0));
    }

    void Interpreter::visit(StringLiteral *n) {
        context.add(variable(n->f0));
    }

    void Interpreter::visit(IntegerLiteral *n) {
        context.add(variable(n->f0));
    }

    void Interpreter::visit(DoubleLiteral *n) {
        context.add(variable(n->f0));
    }

    void Interpreter::visit(Assignment1 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable result = context.get();
        variable identifier = context.get();

        string key = get<2>(identifier);
        context.bind_variable(key, result);
    }

    void Interpreter::visit(BaseExpression1 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(BaseExpression2 *n) {
        n->n1->accept(this);
        variable v = context.get();
        string key = get<2>(v);

        if (context.contains_variable(key)) {
            context.add(context.get_variable(key));
        } else {
            cout << "error : symbol " << key << " not defined" << endl;
            exit(1);
        }
    }

    void Interpreter::visit(BaseExpression3 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(BaseExpression4 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(BaseExpression5 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(BaseExpression6 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(ComplexStatement1 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(ComplexStatement2 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(Expression1 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_plus), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(Expression2 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_minus), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(Expression3 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(Mango1 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(MultiplicativeExpression1 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_mult), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(MultiplicativeExpression2 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_div), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(MultiplicativeExpression3 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_idiv), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(MultiplicativeExpression4 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_mod), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(MultiplicativeExpression5 *n) {
        n->n1->accept(this);
        n->n2->accept(this);

        variable v2 = context.get();
        variable v1 = context.get();

        variable v3 = std::visit(type_checker(op_pow), v1, v2);
        context.add(v3);
    }

    void Interpreter::visit(MultiplicativeExpression6 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(Print1 *n) {
        n->n1->accept(this);

        variable v = context.get();
        cout << variable_to_string(v) << endl;
    }

    void Interpreter::visit(SimpleStatement1 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(Statement1 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(Statement2 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(StatementList1 *n) {
        n->n1->accept(this);
        n->n2->accept(this);
    }

    void Interpreter::visit(StatementList2 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(StatementSuite1 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(ComplexStatement3 *n) {
        n->n1->accept(this);
    }

    void Interpreter::visit(ForLoop1 *n) {
        n->n1->accept(this);
        n->n2->accept(this);
        n->n3->accept(this);

        variable end = context.get();
        variable begin = context.get();

        variable identifier = context.get();
        string key = get<2>(identifier);
        context.bind_variable(key, begin);

        while (true) {
            n->n4->accept(this);

            variable current_iter = context.get_variable(key);
            variable offset(1);
            variable next_iter = std::visit(type_checker(op_plus), current_iter, offset);

            context.bind_variable(key, next_iter);

            if (next_iter >= end) break;
        }
    }
}
