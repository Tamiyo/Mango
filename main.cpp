#include <iostream>
#include "lex/mglex.h"

using std::cout;
using std::endl;

int main() {
    const char *body = "myString = 50";
    auto *lexer = new mglex();
    lexer->lltoken(body);
}