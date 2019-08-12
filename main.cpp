#include <iostream>
#include <string>
#include <vector>

#include "lexer.h"
#include "parser.h"

#include "common.h"

using std::string;
using std::vector;
using std::pair;

int SCOPE_LEVEL = 1;
int SCOPE_LIMIT = 256;
map<int, map<string, Node>> SCOPED_SYMBOL_TABLE = {};

int main() {
    string input = "3 * 3+3 / 1$";
    Lexer lexer = {input};
    vector<LexerResult> tokens = lexer.lex();

    Parser parser = Parser(tokens);
    parser.parse();
}