#include <iostream>
#include <stdio.h>
#include "parser/mgparser.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
    const char *body = "@myfunction():\n$";
    auto *parser = new mgparser(body);
    parser->ppeval();
}