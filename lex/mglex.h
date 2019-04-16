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

class mglex {
public:
    explicit mglex(const char *);

    char *lltoken();

private:
    const static int BUFFER_SIZE = 4096;
    char BUFFER[BUFFER_SIZE]{};

    char *lexemeBegin{};
    char *forward{};

    const char *endOfFile = "\\eof";
    keywords *keys;

    char *charSlice(char *, char *, int);
};


#endif //CLIONLANG_LEX_H
