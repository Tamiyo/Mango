#include <iostream>
#include <stdio.h>
#include "lex/mglex.h"
#include "parser/mgparser.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
//    const char *body = "myString = 150\nyourString = 250.10";
//
//    auto *lexer = new mglex(body);
//
//    pair<const char*, keywords::Symbols> token = lexer->lltoken();
//    cout << token.first << endl;
//    while (token.second != keywords::TS_EOF) {
//        token = lexer->lltoken();
//        cout << token.first << endl;
//    }

    const char *body = "myString = 150\nyourString = 250.10";
    auto *parser = new mgparser(body);
    parser->ppeval();
}