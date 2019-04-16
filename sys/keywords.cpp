//
// Created by Matt on 4/15/2019.
//

#include "keywords.h"

keywords::keywords() {
    KEYWORDS = {
            {"\n",    20},
            {"=",     61},
            {"+",     43},
            {"-",     42},
            {"*",     45},
            {"/",     47},
            {"<",     48},
            {">",     49},
            {">=",    50},
            {"<=",    51},
            {"=",     52},
            {"if",    101},
            {"while", 102}
    };

    TYPES = {
            {-1, "keyword"},
            {0,  "identifier"},
            {1,  "integer"},
            {2,  "float"},
            {3,  "string"},
    };
}
