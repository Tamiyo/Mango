#include "interpreter.h"

void Interpreter::visit(Identifier *n) {
    cur_scope.intermediate_stack.push(n->f0);
}

void Interpreter::visit(StringLiteral *n) {
    cur_scope.intermediate_stack.push(n->f0);
}

void Interpreter::visit(DoubleLiteral *n) {
    cur_scope.intermediate_stack.push(n->f0);
}

void Interpreter::visit(IntegerLiteral *n) {
    cur_scope.intermediate_stack.push(n->f0);
}

void Interpreter::visit(BaseExpression1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(BaseExpression2 *n) {
    n->n1->accept(this);
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == 1) {
        string ident = get<1>(v1);

        bool found = (cur_scope.symbol_table.find(ident) != cur_scope.symbol_table.end());

        if (found) {
            variant<int, string, double> t = cur_scope.symbol_table[ident];
            cur_scope.intermediate_stack.push(t);
        } else {
            cout << "ERROR :: VARIABLE NOT FOUND :: VARIABLE " << ident << " IS UNDEFINED" << endl;
        }
    } else {
        cout << "ERROR :: INVALID IDENTIFIER :: IDENTIFIER NOT VALID" << endl;
    }
}

void Interpreter::visit(BaseExpression3 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(Expression1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) + get<0>(v2);
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            string t = get<1>(v1) + get<1>(v2);
            cur_scope.intermediate_stack.push(t);
        } else {
            double t = get<2>(v1) + get<2>(v2);
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR + NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(BaseExpression5 *n) {
    n->n1->accept(this);
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == 0) {
        int t1 = get<0>(v1);
        int t = -1 * t1;
        cur_scope.intermediate_stack.push(t);
    } else if (v1.index() == 1) {
        string t1 = get_type_debug(v1);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR  NOT DEFINED FOR " << t1 << endl;
    } else {
        double t1 = get<2>(v1);
        double t = -1.0 * t1;
        cur_scope.intermediate_stack.push(t);
    }
}

void Interpreter::visit(Expression2 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) - get<0>(v2);
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR - NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = get<2>(v1) - get<2>(v2);
            cur_scope.intermediate_stack.push(t);
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
    scope_stack.push({});
    cur_scope = scope_stack.top();
    n->n1->accept(this);
}

void Interpreter::visit(MultiplicativeExpression1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = get<0>(v1) * get<0>(v2);
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR * NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = get<2>(v1) * get<2>(v2);
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        if (v1.index() == 1 && v2.index() == 0) {
            string t1 = get<1>(v1);
            string t = t1;

            int t2 = get<0>(v2);

            for (int i = 0; i < (t2 - 1); i++) t += t1;
            cur_scope.intermediate_stack.push(t);
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

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            if (t2 == 0) {
                cout << "ERROR :: DIVIDE BY ZERO" << endl;
                exit(1);
            }

            int t = t1 / t2;
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR / NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);
            double t = t1 / t2;

            if (t2 == 0.0) {
                cout << "ERROR :: DIVIDE BY ZERO" << endl;
                exit(1);
            }

            cur_scope.intermediate_stack.push(t);
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

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            if (t2 == 0) {
                cout << "ERROR :: DIVIDE BY ZERO" << endl;
                exit(1);
            }

            int t = t1 / t2;
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR // NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);
            int t = (int) (t1 / t2);

            if (t2 == 0.0) {
                cout << "ERROR :: DIVIDE BY ZERO" << endl;
                exit(1);
            }

            cur_scope.intermediate_stack.push(t);
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

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == 0 && v2.index() == 0) {
        int t = get<0>(v1) % get<0>(v2);
        cur_scope.intermediate_stack.push(t);
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

    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t = std::pow(get<0>(v1), get<0>(v2));
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            cout << "ERROR :: INVALID OPERATOR :: OPERATOR ^ NOT DEFINED FOR string AND string" << endl;
            exit(1);
        } else {
            double t = std::pow(get<2>(v1), get<2>(v2));
            cur_scope.intermediate_stack.push(t);
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
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

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
    n->n3->accept(this);
}

void Interpreter::visit(StatementList2 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
}

void Interpreter::visit(StatementList3 *n) {
    n->n1->accept(this);
}


void Interpreter::visit(StatementSuite1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
}

void Interpreter::visit(StatementSuite2 *n) {
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

void Interpreter::visit(Assignment1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == 1) {
        string ident = get<1>(v1);
        cur_scope.symbol_table[ident] = v2;
    } else {
        string t1 = get_type_debug(v1);
        cout << "ERROR :: INVALID IDENTIFIER :: TYPE " << t1 << " IS NOT A VALID IDENTIFIER" << endl;
        exit(1);
    }
}

void Interpreter::visit(ComplexStatement1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(ComplexStatement2 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(SimpleStatement1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(NewlineLoop1 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(NewlineLoop2 *n) {}

void Interpreter::visit(BaseExpression4 *n) {
    n->n1->accept(this);
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == 0) {
        int t1 = get<0>(v1);
        int t = !t1;
        cur_scope.intermediate_stack.push(t);
    } else {
        string t1 = get_type_debug(v1);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR ! NOT DEFINED FOR " << t1 << endl;
    }
}

void Interpreter::visit(ConditionalExpression1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            int t = t1 == t2;
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            string t1 = get<1>(v1);
            string t2 = get<1>(v2);

            int t = t1 == t2;
            cur_scope.intermediate_stack.push(t);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);

            int t = t1 == t2;
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR == NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression2 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index() && (v1.index() != 1)) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            int t = t1 < t2;
            cur_scope.intermediate_stack.push(t);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);

            int t = t1 < t2;
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR < NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression3 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index() && (v1.index() != 1)) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            int t = t1 <= t2;
            cur_scope.intermediate_stack.push(t);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);

            int t = t1 <= t2;
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR <= NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression4 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index() && (v1.index() != 1)) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            int t = t1 > t2;
            cur_scope.intermediate_stack.push(t);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);

            int t = t1 > t2;
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR > NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression5 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index() && (v1.index() != 1)) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            int t = t1 >= t2;
            cur_scope.intermediate_stack.push(t);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);

            int t = t1 >= t2;
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR >= NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression6 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index() && (v1.index() == 0)) {
        int t1 = get<0>(v1);
        int t2 = get<0>(v2);

        int t = t1 && t2;
        cur_scope.intermediate_stack.push(t);
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR && NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression7 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index() && (v1.index() == 0)) {
        int t1 = get<0>(v1);
        int t2 = get<0>(v2);

        int t = t1 || t2;
        cur_scope.intermediate_stack.push(t);
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR || NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression8 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    if (v1.index() == v2.index()) {
        if (v1.index() == 0) {
            int t1 = get<0>(v1);
            int t2 = get<0>(v2);

            int t = t1 != t2;
            cur_scope.intermediate_stack.push(t);
        } else if (v1.index() == 1) {
            string t1 = get<1>(v1);
            string t2 = get<1>(v2);

            int t = t1 != t2;
            cur_scope.intermediate_stack.push(t);
        } else {
            double t1 = get<2>(v1);
            double t2 = get<2>(v2);

            int t = t1 != t2;
            cur_scope.intermediate_stack.push(t);
        }
    } else {
        string t1 = get_type_debug(v1);
        string t2 = get_type_debug(v2);
        cout << "ERROR :: INVALID OPERATOR :: OPERATOR != NOT DEFINED FOR " << t1 << " AND " << t2 << endl;
        exit(1);
    }
}

void Interpreter::visit(ConditionalExpression9 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(ComplexStatement3 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(ComplexStatement4 *n) {
    n->n1->accept(this);
}

void Interpreter::visit(ForLoop1 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    string ident = get<1>(v1);
    int from = 0;
    int to = get<0>(v2);


    if (cur_scope.symbol_table.find(ident) == cur_scope.symbol_table.end()) {
        scope_stack.push(cur_scope);
        cur_scope = scope_stack.top();

        cur_scope.symbol_table[ident] = from;

        while (get<0>(cur_scope.symbol_table[ident]) < to) {
            n->n3->accept(this);
            get<0>(cur_scope.symbol_table[ident]) += 1;
        }
        scope_stack.pop();
        cur_scope = scope_stack.top();
    } else {
        cout << "ERROR :: INDEX ALREADY INITIALIZED :: INDEX " << ident << " HAS ALREADY BEEN INITIALIZED" << endl;
    }
}

void Interpreter::visit(ForLoop2 *n) {
    n->n1->accept(this);
    n->n2->accept(this);
    n->n3->accept(this);
    variant<int, string, double> v3 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v2 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();
    variant<int, string, double> v1 = cur_scope.intermediate_stack.top();
    cur_scope.intermediate_stack.pop();

    string ident = get<1>(v1);
    int from = std::min(get<0>(v2), get<0>(v3));
    int to = std::max(get<0>(v2), get<0>(v3));


    if (cur_scope.symbol_table.find(ident) == cur_scope.symbol_table.end()) {
        scope_stack.push(cur_scope);
        cur_scope = scope_stack.top();

        cur_scope.symbol_table[ident] = from;

        while (get<0>(cur_scope.symbol_table[ident]) < to) {
            n->n4->accept(this);
            get<0>(cur_scope.symbol_table[ident]) += 1;
        }

        scope_stack.pop();
        cur_scope = scope_stack.top();
    } else {
        cout << "ERROR :: INDEX ALREADY INITIALIZED :: INDEX " << ident << " HAS ALREADY BEEN INITIALIZED" << endl;
    }
}

void Interpreter::visit(WhileLoop1 *n) {
    Visitor::visit(n);
}
