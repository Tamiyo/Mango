#include "parsing/parser.h"
#include "utils/generator.h"
#include "interpreter/interpreter.h"

using std::cout;
using std::endl;

int main() {
//    generator g;
//    g.generate();
//    cout << "# of itemsets: " << g.c.size() << endl;

    lexer l;
    l.lex();

//    auto q = l.tokens;
//    while(!q.empty()) {
//        cout << q.front() << endl;
//        q.pop();
//    }

    parser p(l.tokens);
    Mango1 *root = p.parse();

    Interpreter I;
    I.visit(root);

    return 0;
}