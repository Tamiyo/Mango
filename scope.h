#ifndef MANGOREVISITEDCPPCLION_SCOPE_H
#define MANGOREVISITEDCPPCLION_SCOPE_H

#include <stack>

#include "semantic_analyzer.h"
#include "tree.h"
#include "function.h"

namespace mango {
    class scope {
    public:
        scope() = default;

        variable get();
        void add(const variable &v);

        void add_variable(const string&, variable);
        void add_function(const string&, Node*);

        variable get_variable(const string&);
        void get_function(const string&);

        bool has_variable(const string&);
        bool has_function(const string&);

        map<string, variable> vtable;
        map<string, func> ftable;

    private:
        stack<variable> stack;
    };
}

#endif //MANGOREVISITEDCPPCLION_SCOPE_H
