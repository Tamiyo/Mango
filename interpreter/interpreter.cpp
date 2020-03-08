#include "interpreter.h"

void Interpreter::visit(Identifier *n) {
    current_state.intermediate_stack.push(n->f0);
}

void Interpreter::visit(StringLiteral *n) {
    current_state.intermediate_stack.push(n->f0);
}

void Interpreter::visit(DoubleLiteral *n) {
    current_state.intermediate_stack.push(n->f0);
}

void Interpreter::visit(IntegerLiteral *n) {
    current_state.intermediate_stack.push(n->f0);
}

void Interpreter::visit(BaseExpression1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(BaseExpression2 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(BaseExpression3 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Expression1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) + get<0>(v2);
            current_state.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            string t = get<1>(v1) + get<1>(v2);
            current_state.intermediate_stack.push(t);
        } else {
            double t = get<2>(v1) + get<2>(v2);
            current_state.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR + NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(Expression2 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) - get<0>(v2);
            current_state.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR - NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = get<2>(v1) - get<2>(v2);
            current_state.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR - NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(Expression3 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Literal1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Literal2 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Literal3 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Mango1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(MultiplicativeExpression1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) * get<0>(v2);
            current_state.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR * NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = get<2>(v1) * get<2>(v2);
            current_state.intermediate_stack.push(t);
        }
    } else {
        if (v1.index() == 1 && v2.index() == 0) {
            string t1 = get<1>(v1);
            string t = t1;

            int t2 = get<0>(v2);

            for (int i = 0; i < (t2 - 1); i++) t += t1;
            current_state.intermediate_stack.push(t);
        } else {
            string t1 = get_type_debug(v1);
            string t2 = get_type_debug(v2);
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR * NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
            exit(1);
        }
    }
}

void Interpreter::visit(MultiplicativeExpression2 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) / get<0>(v2);
            current_state.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR / NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = get<2>(v1) / get<2>(v2);
            current_state.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR / NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(MultiplicativeExpression3 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) / get<0>(v2);
            current_state.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR // NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            int t = (int) (get<2>(v1) / get<2>(v2));
            current_state.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR // NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(MultiplicativeExpression4 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == 0 && v2.index() == 0) {
        int t = get<0>(v1) % get<0>(v2);
        current_state.intermediate_stack.push(t);
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR % NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(MultiplicativeExpression5 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = std::pow(get<0>(v1), get<0>(v2));
            current_state.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR ^ NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = std::pow(get<2>(v1), get<2>(v2));
            current_state.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR ^ NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(MultiplicativeExpression6 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Print1 *n) {
    n->n1->accept(this);
    variant<int, string, double> v1 = current_state.intermediate_stack.top();
    current_state.intermediate_stack.pop();

    if (v1.index() == 0) {
        int t = get<0>(v1);
        cout << t << endl;
    } else if (v1.index() == 1) {
        string t = get<1>(v1);
        cout << t << endl;
    } else {
        double t = get<2>(v1);
        cout << t << endl;
    }
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


string Interpreter::get_type_debug(const variant<int, string, double> &v) {
    string t;
    switch (v.index()) {
        case 0:
            t = "int";
            break;
        case 1:
            t = "string";
            break;
        case 2:
            t = "double";
            break;
    }
    return t;
}