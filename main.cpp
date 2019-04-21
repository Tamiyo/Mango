#include <iostream>
#include <stdio.h>
#include "lex/mglex.h"
#include "parser/mgparser.h"
#include "grammar_cpp_gen.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
	auto* gen = new grammar_cpp_gen();
	gen->gen_parse_table();

	//const char *body = "8^9\n";
	//auto *parser = new mgparser(body);
	//parser->ppeval();

	system("pause");
}