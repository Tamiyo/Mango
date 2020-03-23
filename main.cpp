#include <iostream>
#include <fstream>
#include <string>
#include <memory>

#include "lexer.hpp"
#include "parser.hpp"
#include "expression.hpp"
#include "interpreter.hpp"

using namespace std;
using namespace mango;

int main() {
    ifstream t("../test.f");
    stringstream buffer;
    buffer << t.rdbuf();
    string source = buffer.str();

    cout << "source: " << source << endl;

    lexer l(source);
    vector<token> tokens = l.scan_tokens();
//    for (const auto &token : tokens) cout << token.lexeme << endl;

    parser p(tokens);
    unique_ptr<expr> exp = p.parse();

    interpreter i;
    i.interpret(exp);

    return 0;
}
