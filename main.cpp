#include <iostream>
#include <stdio.h>
#include "lex/mglex.h"
#include "parser/mgparser.h"
#include "grammar_cpp_gen.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
//	const char *body = "8^9";
	//auto *parser = new mgparser(body);
	//parser->ppeval();
	
	
	auto* gen = new grammar_cpp_gen();
	gen->gen();	
	system("pause");
}