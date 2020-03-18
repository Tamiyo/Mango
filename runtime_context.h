#ifndef MANGOREVISITEDCPPCLION_RUNTIME_CONTEXT_H
#define MANGOREVISITEDCPPCLION_RUNTIME_CONTEXT_H

#include <stack>

#include "semantic_analyzer.h"
#include "function.h"
#include "scope.h"

using namespace std;

namespace mango {
    class runtime_context {
    public:
        runtime_context() {
            _scope.push(new scope());
        }

        variable get();

        void add(const variable &);

        variable get_variable(const string &);

        void bind_variable(const string &, const variable &);

        bool contains_variable(const string &);

        void create_function_scope(const vector<string> &args);

        void exit_scope();

    private:
        scope* get_scope() {
            return _scope.top();
        }

        stack<scope*> _scope;
    };
}


#endif //MANGOREVISITEDCPPCLION_RUNTIME_CONTEXT_H
