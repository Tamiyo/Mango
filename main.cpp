#include <iostream>
#include <stdio.h>
#include "parser/mgparser.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
    const char *body = "70 + 20\n$";
    auto *parser = new mgparser(body);
    parser->ppeval();
}