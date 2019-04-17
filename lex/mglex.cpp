//
// Created by Matt on 4/11/2019.
//

#include "mglex.h"

pair<const char *, keywords::Symbols> mglex::lltoken() {
    forward = lexemeBegin;
    if (!*forward) {
        return {"\\eof", keywords::TS_EOF};
    } else {
        if (*forward == '\n') {
            lexemeBegin++;
            return {"\n", keywords::TS_ENDL};

        } else if (*forward == '\"' && *(forward + 1) != '\n') {
            lexemeBegin++;
            forward++;
            while (*forward != '"') {
                // This lexeme is apart of the STRING, and we keep moving.
                if (isalnum(*forward)) {
                    forward++;
                }
                    // This lexeme is NOT apart of the STRING, end analysis.
                else {
                    return charSlice(lexemeBegin, forward, 3);
                }
            }
            return charSlice(lexemeBegin, forward, 3);
        } else if ((isalpha(*forward) || ispunct(*forward)) && !isspace(*forward)) {
            while (*forward) {
                // This lexeme is apart of the IDENTIFIER, and we keep moving.
                if ((isalpha(*forward) || ispunct(*forward)) && !isspace(*forward)) {
                    forward++;
                }
                    // This lexeme is NOT apart of the IDENTIFIER, end analysis.
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

pair<const char *, keywords::Symbols> mglex::charSlice(char *lb, char *lf, int TYPE) {
    int LEXEME_SIZE = (lf - lb + 1) * sizeof(*lb);
    char *lexeme = new char[LEXEME_SIZE];
    snprintf(lexeme, LEXEME_SIZE, "%s", lb);
    lexemeBegin = lf;

    if (TYPE == 0) {
        if (keys->KEYWORDS.count(lexeme)) {
            return {lexeme, keys->KEYWORDS[lexeme]};
        } else {
            return {lexeme, keywords::TS_IDENT};
        }
    } else {
        return {lexeme, keys->TYPES[TYPE]};
    }
}

mglex::mglex(const char *body) {
    keys = new keywords();
    lexemeBegin = const_cast<char *>(body);
    forward = lexemeBegin;
}
