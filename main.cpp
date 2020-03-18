#include "generator.h"
#include "lexer.h"
#include "parser.h"
#include "interpreter.h"


int main() {
//    generator g;
//    g.generate();
//    cout << "# of itemsets: " << g.c.size() << endl;

    lexer l;
    l.lex();

    parser p(l.tokens);
    auto root = p.parse();

    Interpreter I;
    I.visit(root);
}

