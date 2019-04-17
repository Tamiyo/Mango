//
// Created by Matt on 4/11/2019.
//

#ifndef CLIONLANG_LEX_H
#define CLIONLANG_LEX_H

#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include <cstring>
#include "../sys/keywords.h"

using std::cout;
using std::endl;
using std::pair;

class mglex {
private:
    const static int BUFFER_SIZE = 4096;
    char BUFFER[BUFFER_SIZE]{};

    char *lexemeBegin{};
    char *forward{};

    keywords *keys;

    pair<const char *, keywords::Symbols> charSlice(char *, char *, int);

public:
    explicit mglex(const char *);

    pair<const char *, keywords::Symbols> lltoken();
};


#endif //CLIONLANG_LEX_H
