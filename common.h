//
// Created by Matt on 8/11/2019.
//

// This file contains global variables and abstract classes that are to be utilized throughout the compiler.

#ifndef MANGO_V2_CPP_COMMON_H
#define MANGO_V2_CPP_COMMON_H

#include <map>
#include <string>

using std::map;
using std::string;

// The current scope level that the compiler is at. Defined in main.cpp
extern int SCOPE_LEVEL;
// The maximum scope level of the compiler. Defined in main.cpp
extern int SCOPE_LIMIT;

// The (scoped) symbol table of compiler
extern map<int, map<string, struct Node>> SCOPED_SYMBOL_TABLE;

// Abstract struct used for the parse_tree.h
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
