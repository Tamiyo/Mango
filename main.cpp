#include "parsing/parser.h"
#include "utilities/generator.h"
#include "interpreter/interpreter.h"

using std::cout;
using std::endl;

int main() {
    std::cout << "Hello, World!" << std::endl;

    generator g;
    g.generate();
    cout << "# of itemsets: " << g.c.size() << endl;

//    lexer l;
//    l.lex();
//
//    std::cout << l.tokens.print_state() << endl;
//
//    parser p(l.tokens);
//    Mango1 *root = p.parse();
//
//    Interpreter I;
//    I.visit(root);

    return 0;
}