#ifndef MANGOREVISITEDCPPCLION_INTERPRETER_H
#define MANGOREVISITEDCPPCLION_INTERPRETER_H

#include "iostream"
#include "algorithm"
#include "cmath"

#include "../tree/tree.h"
#include "scope.h"

using std::cout;
using std::endl;

class Interpreter : public Visitor {
public:
	void visit(Identifier *n) override;
	void visit(StringLiteral *n) override;
	void visit(DoubleLiteral *n) override;
	void visit(IntegerLiteral *n) override;
	void visit(Assignment1* n) override;
	void visit(BaseExpression1* n) override;
	void visit(BaseExpression2* n) override;
	void visit(BaseExpression3* n) override;
	void visit(BaseExpression4* n) override;
	void visit(BaseExpression5* n) override;
	void visit(ComplexStatement1* n) override;
	void visit(ComplexStatement2* n) override;
	void visit(ComplexStatement3* n) override;
	void visit(ComplexStatement4* n) override;
	void visit(ConditionalExpression1* n) override;
	void visit(ConditionalExpression2* n) override;
	void visit(ConditionalExpression3* n) override;
	void visit(ConditionalExpression4* n) override;
	void visit(ConditionalExpression5* n) override;
	void visit(ConditionalExpression6* n) override;
	void visit(ConditionalExpression7* n) override;
	void visit(ConditionalExpression8* n) override;
	void visit(ConditionalExpression9* n) override;
	void visit(Expression1* n) override;
	void visit(Expression2* n) override;
	void visit(Expression3* n) override;
	void visit(ForLoop1* n) override;
	void visit(ForLoop2* n) override;
	void visit(Literal1* n) override;
	void visit(Literal2* n) override;
	void visit(Literal3* n) override;
	void visit(Mango1* n) override;
	void visit(MultiplicativeExpression1* n) override;
	void visit(MultiplicativeExpression2* n) override;
	void visit(MultiplicativeExpression3* n) override;
	void visit(MultiplicativeExpression4* n) override;
	void visit(MultiplicativeExpression5* n) override;
	void visit(MultiplicativeExpression6* n) override;
	void visit(NewlineLoop1* n) override;
	void visit(NewlineLoop2* n) override;
	void visit(Print1* n) override;
	void visit(SimpleStatement1* n) override;
	void visit(Statement1* n) override;
	void visit(Statement2* n) override;
	void visit(StatementList1* n) override;
	void visit(StatementList2* n) override;
	void visit(StatementList3* n) override;
	void visit(StatementSuite1* n) override;
	void visit(StatementSuite2* n) override;
	void visit(WhileLoop1* n) override;

private:
	stack<scope> scope_stack;
	scope cur_scope;
	static string get_type_debug(const variant<int, string, double>&);
};

#endif //MANGOREVISITEDCPPCLION_INTERPRETER_H