#include <iostream>
#include <stdio.h>
#include "lex/mglex.h"
#include "parser/mgparser.h"

using std::cout;
using std::endl;
using std::pair;

int main() {
	const char *body = "8^9";
	auto *parser = new mgparser(body);
	parser->ppeval();
	system("pause");
}