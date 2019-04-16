//
// Created by Matt on 4/15/2019.
//

#include "keywords.h"

keywords::keywords() {
    KEYWORDS = {
            {"=", 61},
            {"+", 43},
            {"-", 42},
            {"*", 45},
            {"/", 47},
    };

    TYPES = {
            {0, "identifier"},
            {1, "integer"},
            {2, "float"}
    };
}
