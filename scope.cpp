#include "scope.h"

#include <utility>

namespace mango {

    variable scope::get() {
        variable v = stack.top();
        stack.pop();
        return v;
    }

    void scope::add(const variable &v) {
        stack.emplace(v);
    }

    void scope::add_variable(const string &key, variable value) {
        vtable[key] = std::move(value);
    }

    void scope::add_function(const string &key, Node *value) {
        vtable[key] = value;
    }

    variable scope::get_variable(const string &key) {
        if (has_variable(key)) {
            return vtable[key];
        } else {
            cout << "error : " << key << " is undefined" << endl;
            exit(1);
        }
    }


    void scope::get_function(const string &key) {
        if (has_function(key)) {
        } else {
            cout << "error : " << key << " is undefined" << endl;
            exit(1);
        }
    }

    bool scope::has_variable(const string &key) {
        return vtable.find(key) != vtable.end();
    }

    bool scope::has_function(const string &key) {
        return ftable.find(key) != ftable.end();
    }
}