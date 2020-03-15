#ifndef MANGOREVISITEDCPPCLION_INTERPRETER_H
#define MANGOREVISITEDCPPCLION_INTERPRETER_H

#include <map>
#include <string>
#include <vector>
#include <memory>

#include "tree.h"

using namespace std;

namespace mango {
    class Interpreter : public Visitor {
    public:
        void visit(Identifier* n) override;
        void visit(StringLiteral* n) override;
        void visit(IntegerLiteral* n) override;
        void visit(DoubleLiteral* n) override;
        void visit(Assignment1* n) override;
        void visit(BaseExpression1* n) override;
        void visit(BaseExpression2* n) override;
        void visit(BaseExpression3* n) override;
        void visit(BaseExpression4* n) override;
        void visit(BaseExpression5* n) override;
        void visit(BaseExpression6* n) override;
        void visit(ComplexStatement1* n) override;
        void visit(ComplexStatement2* n) override;
        void visit(Expression1* n) override;
        void visit(Expression2* n) override;
        void visit(Expression3* n) override;
        void visit(Mango1* n) override;
        void visit(MultiplicativeExpression1* n) override;
        void visit(MultiplicativeExpression2* n) override;
        void visit(MultiplicativeExpression3* n) override;
        void visit(MultiplicativeExpression4* n) override;
        void visit(MultiplicativeExpression5* n) override;
        void visit(MultiplicativeExpression6* n) override;
        void visit(Print1* n) override;
        void visit(SimpleStatement1* n) override;
        void visit(Statement1* n) override;
        void visit(Statement2* n) override;
        void visit(StatementList1* n) override;
        void visit(StatementList2* n) override;
        void visit(StatementSuite1* n) override;

    private:
    };
}

#endif //MANGOREVISITEDCPPCLION_INTERPRETER_H