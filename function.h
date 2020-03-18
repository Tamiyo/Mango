#ifndef MANGOREVISITEDCPPCLION_FUNCTION_H
#define MANGOREVISITEDCPPCLION_FUNCTION_H

#include <vector>

#include "tree.h"

using namespace std;

namespace mango {
    class func {
    public:
        explicit func(Node *n, int i) {
            node = n;
            num_params = i;
        }

    private:
        Node *node;
        int num_params;
    };
}

#endif //MANGOREVISITEDCPPCLION_FUNCTION_H
