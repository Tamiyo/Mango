//
// Created by Matt on 4/11/2019.
//

#include "mglex.h"

void mglex::lltoken(const char *str) {
    lexemeBegin = const_cast<char *>(str);
    forward = lexemeBegin;

    if (isalpha(*forward++)) {
        while (*forward) {
            if (isalnum(*forward)) {
                // We're OK
            } else {
                // End Lexeme Analysis
            }
        }
    } else if (isdigit(*forward++)) {
        bool canBeFloat = true;
        while (*forward) {
            if (isdigit(*forward)) {
                // We're OK, assume type INT
            } else if (*forward == '.' && canBeFloat) {
                canBeFloat = false;
                // We change to type float
            } else {
                // End Lexeme Analysis
            }
        }
    }
}

mglex::mglex() = default;
