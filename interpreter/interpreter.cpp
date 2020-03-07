#include "interpreter.h"

string Interpreter::visit(Mango1 *n) {
    return n->n1->accept(this);
};



