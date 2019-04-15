//
// Created by Matt on 4/11/2019.
//

#ifndef CLIONLANG_LEX_H
#define CLIONLANG_LEX_H

#include <iostream>
#include <cstring>

using std::cout;
using std::endl;

class mglex {
public:
    mglex();

    void lltoken(const char *);

private:
    const static int BUFFER_SIZE = 4096;
    char BUFFER[BUFFER_SIZE]{};
    char *lexemeBegin{};
    char *forward{};
};


#endif //CLIONLANG_LEX_H
