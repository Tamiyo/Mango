//
// Created by Matt on 4/25/2019.
//

#include "mglexer.h"

/// TODO Can probably get rid of the pair and keep just the tokens::Symbols
pair<const char *, tokens::Symbols> mglexer::lltoken() {
    forward = lexemeBegin;
    if (!*forward) {
        return { "$", tokens::TS_EOF };
    }
    else {
        if (*forward == '\n') {
            lexemeBegin++;
            return { "\n", tokens::TS_NEWLINE };
        }
        else if (ispunct(*forward)) {
            while (*forward) {
                // This lexeme is apart of the IDENTIFIER, and we keep moving.
                if (ispunct(*forward)) {
                    forward++;
                }
                if(isSpecialToken(lexemeBegin, forward)) {
                    return charSlice(lexemeBegin, forward, 0);
                }
                    // This lexeme is NOT apart of the IDENTIFIER, end analysis.
                else {
                    return charSlice(lexemeBegin, forward, 0);
                }
            }
            return charSlice(lexemeBegin, forward, 0);
        }
        else if (*forward == '\"' && *(forward + 1) != '\n') {
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
        }
        else if (isalpha(*forward) && !isspace(*forward)) {
            while (*forward) {
                // This lexeme is apart of the IDENTIFIER, and we keep moving.
                if ((isalpha(*forward) || isIdentSpecial(*forward)) && !isspace(*forward)) {
                    forward++;
                }
                    // This lexeme is NOT apart of the IDENTIFIER, end analysis.
                else {
                    return charSlice(lexemeBegin, forward, 0);
                }
            }
            return charSlice(lexemeBegin, forward, 0);
        }
        else if (isdigit(*forward)) {
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
        }
        else {
            lexemeBegin++;
        }
    }
}

bool mglexer::isIdentSpecial(char c) {
    return c == '-' || c == '_';
}

bool mglexer::isSpecialToken(char *lb, char *lf) {
    int LEXEME_SIZE = (lf - lb + 1) * sizeof(*lb);
    char *lexeme = new char[LEXEME_SIZE];
    snprintf(lexeme, LEXEME_SIZE, "%s", lb);

    return keys->TOKENS.count(lexeme) != 0;
}

pair<const char *, tokens::Symbols> mglexer::charSlice(char *lb, char *lf, int TYPE) {
    int LEXEME_SIZE = (lf - lb + 1) * sizeof(*lb);
    char *lexeme = new char[LEXEME_SIZE];
    snprintf(lexeme, LEXEME_SIZE, "%s", lb);
    lexemeBegin = lf;

    printf("Found lexeme: %s\n", lexeme);

    if (TYPE == 0) {
        if (keys->TOKENS.count(lexeme)) {
            return { lexeme, keys->TOKENS[lexeme] };
        }
        else {
            return { lexeme, tokens::TS_IDENTIFIER };
        }
    }
    else if (TYPE == 4) {
        return { lexeme, keys->TOKENS[lexeme] };
    }
    else {
        return { lexeme, keys->TYPES[TYPE] };
    }
}

mglexer::mglexer(const char *body) {
    keys = new tokens();
    lexemeBegin = const_cast<char *>(body);
    forward = lexemeBegin;
}
