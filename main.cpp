#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

#include "lexer.h"
#include "parser.h"

#include "common.h"

using std::string;
using std::vector;
using std::pair;
using std::cout;
using std::endl;

int SCOPE_LEVEL = 1;
int SCOPE_DEPTH_LIMIT = 256;
map<int, map<string, Node *>> SCOPED_SYMBOL_TABLE = {};

int main() {
    string input = "myident = 12.0\nmyident2 = 2$";
    Lexer lexer = {input};
    vector<LexerResult> tokens = lexer.lex();

    Parser parser = Parser(tokens);
    NodeMango *root = dynamic_cast<NodeMango *>(parser.parse());
    root->eval();

    std::cout << "Scoped Table at \"Level 1\" Items..." << std::endl;
    std::for_each(SCOPED_SYMBOL_TABLE[1].begin(), SCOPED_SYMBOL_TABLE[1].end(), [](const std::pair<string, Node *> &p) {
        std::cout << p.first << std::endl;
    });

}