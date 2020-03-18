#include "runtime_context.h"

namespace mango {
    variable runtime_context::get() {
        return get_scope()->get();
    }

    void runtime_context::add(const variable &v) {
        get_scope()->add(v);
    }

    variable runtime_context::get_variable(const string &key) {
        if (get_scope()->has_variable(key)) {
            return get_scope()->get_variable(key);
        } else {
            cout << "error : " << key << " is undefined.";
            exit(1);
        }
    }

    void runtime_context::bind_variable(const string &key, const variable &v) {
        get_scope()->add_variable(key, v);
    }

    bool runtime_context::contains_variable(const string &key) {
        return get_scope()->has_variable(key);
    }

    void runtime_context::create_function_scope(const vector<string> &args) {
        scope* tmp = new scope();

        for (const auto &arg : args) {
            variable v = get_scope()->get_variable(arg);
            tmp->add_variable(arg, v);
        }

        _scope.emplace(tmp);
    }

    void runtime_context::exit_scope() {
        _scope.pop();
    }
}