//
// Created by Matt on 4/15/2019.
//

#include "keywords.h"

keywords::keywords() {
    KEYWORDS = {
            {"\n", TS_ENDL},
            {"=",  TS_EQU},
            {"+",  TS_PLUS},
            {"-",  TS_MINUS},
            {"*",  TS_MUL},
            {"/",  TS_DIV},
            {"<",  TS_LT},
            {">",  TS_GT},
            {">=", TS_GTE},
            {"<=", TS_LTE},
    };

    TYPES = {
            {0, TS_IDENT},
            {1, TS_INT},
            {2, TS_FLOAT},
            {3, TS_STRING},
    };
}
