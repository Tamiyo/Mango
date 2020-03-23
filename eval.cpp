#include "eval.hpp"

namespace mango {
    variable evaluate_expression(variable a, const token &op, variable b) {
        return std::visit(evaluator(op.type, op.line), a, b);
    }

    evaluator::evaluator(token_type op, int line) : op(op), line(line) {
    }

    variable evaluator::operator()(int a, int b) {
        switch (op) {
            case ADD:
                return a + b;
            case SUB:
                return a - b;
            case MUL:
                return a * b;
            case DIV:
            case IDIV:
                return a / b;
            case MOD:
                return a % b;
            case POW:
                return static_cast<int>(pow(a, b));
            case EQUAL_EQUAL:
                return a == b;
            case LT:
                return a < b;
            case LTE:
                return a <= b;
            case GT:
                return a > b;
            case GTE:
                return a >= b;
            case BANG_EQUAL:
                return a != b;
            default:
                stringstream s;
                s << "operator " << op << " is not defined for int and int.";
                error(line, s.str());
                break;
        }
    }

    variable evaluator::operator()(int a, double b) {
        switch (op) {
            case ADD:
                return a + b;
            case SUB:
                return a - b;
            case MUL:
                return a * b;
            case DIV:
                return a / b;
            case IDIV:
                return (int) (a / b);
            case POW:
                return static_cast<int>(pow(a, b));
            case EQUAL_EQUAL:
                return a == b;
            case LT:
                return a < b;
            case LTE:
                return a <= b;
            case GT:
                return a > b;
            case GTE:
                return a >= b;
            case BANG_EQUAL:
                return a != b;
            default:
                break;
        }
    }

    variable evaluator::operator()(int, const string &) {
        stringstream s;
        s << "operator " << op << " is not defined for int and string.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(int, bool) {
        stringstream s;
        s << "operator " << op << " is not defined for int and bool.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(double a, int b) {
        switch (op) {
            case ADD:
                return a + b;
            case SUB:
                return a - b;
            case MUL:
                return a * b;
            case DIV:
                return a / b;
            case IDIV:
                return (int) (a / b);
            case POW:
                return (int) (pow(a, b));
            case EQUAL_EQUAL:
                return a == b;
            case LT:
                return a < b;
            case LTE:
                return a <= b;
            case GT:
                return a > b;
            case GTE:
                return a >= b;
            case BANG_EQUAL:
                return a != b;
            default:
                stringstream s;
                s << "operator " << op << " is not defined for double and int.";
                error(line, s.str());
        }
    }

    variable evaluator::operator()(double a, double b) {
        switch (op) {
            case ADD:
                return a + b;
            case SUB:
                return a - b;
            case MUL:
                return a * b;
            case DIV:
                return a / b;
            case IDIV:
                return (int) (a / b);
            case POW:
                return (int) (pow(a, b));
            case EQUAL_EQUAL:
                return a == b;
            case LT:
                return a < b;
            case LTE:
                return a <= b;
            case GT:
                return a > b;
            case GTE:
                return a >= b;
            case BANG_EQUAL:
                return a != b;
            default:
                stringstream s;
                s << "operator " << op << " is not defined for double and double.";
                error(line, s.str());
        }
    }

    variable evaluator::operator()(double, const string &) {
        stringstream s;
        s << "operator " << op << " is not defined for double and string.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(double, bool) {
        stringstream s;
        s << "operator " << op << " is not defined for double and bool.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(const string &a, int b) {
        string tmp;
        switch (op) {
            case MUL:
                tmp = a;
                for (int i = 0; i < (b - 1); i++) tmp += a;
                return tmp;
            default:
                stringstream s;
                s << "operator " << op << " is not defined for string and int.";
                error(line, s.str());
                return {};
        }
    }

    variable evaluator::operator()(const string &, double) {
        stringstream s;
        s << "operator " << op << " is not defined for string and double.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(const string &a, const string &b) {
        switch (op) {
            case ADD:
                return a + b;
            case EQUAL_EQUAL:
                return a == b;
            case BANG_EQUAL:
                return a != b;
            default:
                stringstream s;
                s << "operator " << op << " is not defined for string and string.";
                error(line, s.str());
        }
    }

    variable evaluator::operator()(const string &, bool) {
        stringstream s;
        s << "operator " << op << " is not defined for string and bool.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(bool, int) {
        stringstream s;
        s << "operator " << op << " is not defined for bool and int.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(bool, double) {
        stringstream s;
        s << "operator " << op << " is not defined for bool and double.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(bool, const string &) {
        stringstream s;
        s << "operator " << op << " is not defined for bool and string.";
        error(line, s.str());
        return {};
    }

    variable evaluator::operator()(bool a, bool b) {
        switch (op) {
            case EQUAL_EQUAL:
                return a == b;
            case BANG_EQUAL:
                return a != b;
            case AND:
                return a && b;
            case OR:
                return a || b;
            default:
                stringstream s;
                s << "operator " << op << " is not defined for bool and bool.";
                error(line, s.str());
        }
    }
}
