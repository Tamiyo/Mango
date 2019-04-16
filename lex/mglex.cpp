//
// Created by Matt on 4/11/2019.
//

#include "mglex.h"

char *mglex::lltoken() {
    forward = lexemeBegin;
    if (!*forward) {
        return nullptr;
    } else {
        if (isalpha(*forward)) {
            while (*forward) {
                // This lexeme is apart of the STRING, and we keep moving.
                if (isalnum(*forward)) {
                    forward++;
                }
                    // This lexeme is NOT apart of the STRING, end analysis.
                else {
                    return charSlice(lexemeBegin, forward, 0);
                }
            }
            return charSlice(lexemeBegin, forward, 0);
        } else if (isdigit(*forward)) {
            bool canBeFloat = true;
            while (*forward) {
                // This lexeme is apart of the NUMBER, and we keep moving.
                if (isdigit(*forward)) {
                    forward++;
                }
                    // This lexeme is apart of the NUMBER, and we keep moving.
                else if (*forward == '.' && canBeFloat) {
                    canBeFloat = false;
                    forward++;
                }
                    // This lexeme is NOT apart of the NUMBER, end analysis.
                else {
                    return canBeFloat ? charSlice(lexemeBegin, forward, 1) : charSlice(lexemeBegin, forward, 2);
                }
            }
            return canBeFloat ? charSlice(lexemeBegin, forward, 1) : charSlice(lexemeBegin, forward, 2);
        } else {
            lexemeBegin++;
        }
    }
}

char *mglex::charSlice(char *lb, char *lf, int TYPE) {
    int LEXEME_SIZE = (lf - lb + 1) * sizeof(*lb);
    char *lexeme = new char[LEXEME_SIZE];
    snprintf(lexeme, LEXEME_SIZE, "%s", lb);
    lexeme[(lf - lb) * sizeof(*lb)] = '\0';
    lexemeBegin = lf;
    printf("lexeme: %s ; %s\n", lexeme, keys->TYPES[TYPE]);
    return lexeme;
}

mglex::mglex(const char *body) {
    keys = new keywords();
    lexemeBegin = const_cast<char *>(body);
    forward = lexemeBegin;
}
