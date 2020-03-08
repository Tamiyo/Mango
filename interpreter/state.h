#ifndef MANGOREVISITEDCPPCLION_STATE_H
#define MANGOREVISITEDCPPCLION_STATE_H

#include "stack"
#include "../tree/tree.h"

#include "variant"

using std::variant;
using std::get;
using std::stack;

class state {
public:
    stack<variant<int, string, double>> intermediate_stack;
};

#endif //MANGOREVISITEDCPPCLION_STATE_H
