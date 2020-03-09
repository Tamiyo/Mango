#ifndef MANGOREVISITEDCPPCLION_SCOPE_H
#define MANGOREVISITEDCPPCLION_SCOPE_H

#include "stack"
#include "variant"
#include "map"
#include "../tree/tree.h"


using std::variant;
using std::get;
using std::stack;
using std::map;

class scope {
public:
    stack<variant<int, string, double>> intermediate_stack;
    map<string, variant<int, string, double>> symbol_table;
};

#endif //MANGOREVISITEDCPPCLION_SCOPE_H
