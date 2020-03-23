//
// Created by zoolo on 3/21/2020.
//

#ifndef MANGO_LOX_EVAL_HPP
#define MANGO_LOX_EVAL_HPP

#include <sstream>
#include <variant>
#include <cmath>

#include "token.hpp"
#include "error.hpp"

using namespace std;

namespace mango {
    variable evaluate_expression(variable, const token &, variable);

    struct evaluator {
    public:
        explicit evaluator(token_type, int);

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

    private:
        token_type op;
        int line;
    };
}


#endif //MANGO_LOX_EVAL_HPP
