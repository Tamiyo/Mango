#ifndef MANGOREVISITEDCPPCLION_INTERPRETER_H
#define MANGOREVISITEDCPPCLION_INTERPRETER_H

#include "../tree/tree.h"
#include "iostream"

class Interpreter : public Visitor {
public:
	string visit(Literal *n) override;
	string visit(BaseExpression1* n) override;
	string visit(BaseExpression2* n) override;
	string visit(BaseExpression3* n) override;
	string visit(Expression1* n) override;
	string visit(Expression2* n) override;
	string visit(Expression3* n) override;
	string visit(Mango1* n) override;
	string visit(MultiplicativeExpression1* n) override;
	string visit(MultiplicativeExpression2* n) override;
	string visit(MultiplicativeExpression3* n) override;
	string visit(MultiplicativeExpression4* n) override;
	string visit(MultiplicativeExpression5* n) override;
	string visit(MultiplicativeExpression6* n) override;
	string visit(Print1* n) override;
	string visit(Statement1* n) override;
	string visit(Statement2* n) override;
	string visit(StatementList1* n) override;
	string visit(StatementList2* n) override;
	string visit(StatementSuite1* n) override;
};

#endif //MANGOREVISITEDCPPCLION_INTERPRETER_H