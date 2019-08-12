//
// Created by Matt on 8/11/2019.
//
#ifndef MANGO_V2_CPP_COMMON_H
#define MANGO_V2_CPP_COMMON_H

#include <map>
#include <string>

using std::map;
using std::string;

extern int SCOPE_LEVEL;
extern int SCOPE_LIMIT;

extern map<int, map<string, struct Node>> SCOPED_SYMBOL_TABLE;

struct Node {
public:
    virtual Node eval() {
        return {};
    }

    Node debug() {
        return {};
    }
};

#endif //MANGO_V2_CPP_COMMON_H
