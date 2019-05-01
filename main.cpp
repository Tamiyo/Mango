#include <iostream>
#include <stdio.h>
#include "generator/generator_parser.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
    auto* gen = new generator_parser();
    gen->gen_parse_table();

    // const char *body = "70 + 20\n";
    // auto *parser = new mgparser(body);
    // parser->ppeval();
}