#ifndef MANGO_SEMANTIC_ANALYZER_H
#define MANGO_SEMANTIC_ANALYZER_H

#include <variant>
#include <cmath>
#include <iostream>

#include "grammar.h"

using namespace std;

namespace mango {
    using variable = variant<int, double, string, bool>;

    struct type_checker {
        explicit type_checker(token tok) {
            op = tok;
        }

        token op;

        variable operator()(int, int);

        variable operator()(int, double);

        variable operator()(int, const string &);

        variable operator()(int, bool);

        variable operator()(double, int);

        variable operator()(double, double);

        variable operator()(double, const string &);

        variable operator()(double, bool);

        variable operator()(const string &, int);

        variable operator()(const string &, double);

        variable operator()(const string &, const string &);

        variable operator()(const string &, bool);

        variable operator()(bool, int);

        variable operator()(bool, double);

        variable operator()(bool, const string &);

        variable operator()(bool, bool);
    };

    string variable_to_string(const variable &);

}

#endif //MANGO_SEMANTIC_ANALYZER_H
