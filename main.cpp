#include <iostream>
#include <stdio.h>
#include "lex/mglex.h"

using std::cout;
using std::endl;

int main() {
    const char *body = "myString = 150.1";

    auto *lexer = new mglex(body);

    char* firstLexeme = lexer->lltoken();
    while(firstLexeme != nullptr) {
       firstLexeme = lexer->lltoken();
    }
}