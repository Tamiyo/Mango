#ifndef MANGO_LOX_ERROR_HPP
#define MANGO_LOX_ERROR_HPP

#include <iostream>
#include <string>
#include <utility>

using namespace std;

namespace mango {
    static void error(int, string);

    void report(int, string, string);

    static bool hadError = false;
}

#endif //MANGO_LOX_ERROR_HPP