#include "semantic_analyzer.h"

namespace mango {
    variable type_checker::operator()(int a, int b) {
        {
            switch (op) {
                case op_plus:
                    return a + b;
                case op_minus:
                    return a - b;
                case op_mult:
                    return a * b;
                case op_idiv:
                case op_div:
                    return a / b;
                case op_mod:
                    return a % b;
                case op_pow:
                    return (int) pow(a, b);
                case op_lt:
                    return (bool) (a < b);
                case op_lte:
                    return (bool) (a <= b);
                case op_gt:
                    return (bool) (a > b);
                case op_gte:
                    return (bool) (a >= b);
                case op_double_equals:
                    return (bool) (a == b);
                default:
                    cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                    exit(1);
            }
        }
    }

    variable type_checker::operator()(int a, double b) {
        switch (op) {
            case op_plus:
                return a + b;
            case op_minus:
                return a - b;
            case op_mult:
                return a * b;
            case op_idiv:
                return (int) (a / b);
            case op_div:
                return a / b;
            case op_pow:
                return pow(a, b);
            case op_lt:
                return (bool) (a < b);
            case op_lte:
                return (bool) (a <= b);
            case op_gt:
                return (bool) (a > b);
            case op_gte:
                return (bool) (a >= b);
            case op_double_equals:
                return (bool) (a == b);
            case op_mod:
            default:
                cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                exit(1);
        }
    }

    variable type_checker::operator()(int a, const string &b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(int a, bool b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(double a, int b) {
        switch (op) {
            case op_plus:
                return a + b;
            case op_minus:
                return a - b;
            case op_mult:
                return a * b;
            case op_idiv:
            case op_div:
                return a / b;
            case op_pow:
                return pow(a, b);
            case op_lt:
                return (bool) (a < b);
            case op_lte:
                return (bool) (a <= b);
            case op_gt:
                return (bool) (a > b);
            case op_gte:
                return (bool) (a >= b);
            case op_double_equals:
                return (bool) (a == b);
            case op_mod:
            default:
                cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                exit(1);
        }
    }

    variable type_checker::operator()(double a, double b) {
        switch (op) {
            case op_plus:
                return a + b;
            case op_minus:
                return a - b;
            case op_mult:
                return a * b;
            case op_idiv:
                return (int) (a / b);
            case op_div:
                return a / b;
            case op_pow:
                return pow(a, b);
            case op_lt:
                return (bool) (a < b);
            case op_lte:
                return (bool) (a <= b);
            case op_gt:
                return (bool) (a > b);
            case op_gte:
                return (bool) (a >= b);
            case op_double_equals:
                return (bool) (a == b);
            case op_mod:
            default:
                cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                exit(1);
        }
    }

    variable type_checker::operator()(double a, const string &b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(double a, bool b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(const string &a, int b) {
        string t = a;
        switch (op) {
            case op_mult:
                for (int i = 0; i < b - 1; i++) t += a;
                return t;
            case op_plus:
            case op_minus:
            case op_div:
            case op_idiv:
            case op_mod:
            case op_pow:
            case op_lt:
            case op_lte:
            case op_gt:
            case op_gte:
            case op_double_equals:
            default:
                cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                exit(1);
        }
    }

    variable type_checker::operator()(const string &a, double b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(const string &a, const string &b) {
        switch (op) {
            case op_plus:
                return a + b;
            case op_double_equals:
                return (bool) (a == b);
            case op_minus:
            case op_mult:
            case op_idiv:
            case op_div:
            case op_mod:
            case op_pow:
            case op_lt:
            case op_lte:
            case op_gt:
            case op_gte:
            default:
                cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                exit(1);
        }
    }

    variable type_checker::operator()(const string &a, bool b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(bool a, int b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(bool a, double b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(bool a, const string &b) {
        cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
        exit(1);
    }

    variable type_checker::operator()(bool a, bool b) {
        switch (op) {
            case logical_and:
                return a && b;
            case logical_or:
                return a || b;
            default:
                cout << "error : operator " << op << " not defined for " << a << " and " << b << endl;
                exit(1);
        }
    }

    string variable_to_string(const variable &v) {
        if (v.index() == 0) {
            int i = get<0>(v);
            return std::to_string(i);
        } else if (v.index() == 1) {
            double d = get<1>(v);
            return std::to_string(d);
        } else if (v.index() == 2) {
            string s = get<2>(v);
            return s;
        } else if (v.index() == 3) {
            bool b = get<3>(v);
            return b ? "true" : "false";
        } else {
            cout << "error : to_string has no non-null unit" << endl;
            exit(1);
        }
    }

}