//
// Created by Matt on 4/25/2019.
//

#ifndef MANGO_CL_MGLEXER_H
#define MANGO_CL_MGLEXER_H

#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include <cstring>
#include "../tokens/tokens.h"


using std::cout;
using std::endl;
using std::pair;

class mglexer {
private:
    const static int BUFFER_SIZE = 4096;
    char BUFFER[BUFFER_SIZE]{};

    char *lexemeBegin{};
    char *forward{};

    tokens *keys;

    bool isSpecialToken(char *lb, char *lf);
    bool isIdentSpecial(char);
    pair<const char *, tokens::Symbols> charSlice(char *, char *, int);

public:
    explicit mglexer(const char *);

    pair<const char *, tokens::Symbols> lltoken();
};


#endif //MANGO_CL_MGLEXER_H
