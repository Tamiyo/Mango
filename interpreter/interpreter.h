#ifndef MANGOREVISITEDCPPCLION_INTERPRETER_H
#define MANGOREVISITEDCPPCLION_INTERPRETER_H

#include "iostream"
#include "cmath"

#include "../tree/tree.h"
#include "state.h"

using std::cout;
using std::endl;

class Interpreter : public Visitor {
public:
	void visit(Identifier *n) override;
	void visit(StringLiteral *n) override;
	void visit(DoubleLiteral *n) override;
	void visit(IntegerLiteral *n) override;
	void visit(BaseExpression1* n) override;
	void visit(BaseExpression2* n) override;
	void visit(BaseExpression3* n) override;
	void visit(Expression1* n) override;
	void visit(Expression2* n) override;
	void visit(Expression3* n) override;
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
	void visit(Print1* n) override;
	void visit(Statement1* n) override;
	void visit(Statement2* n) override;
	void visit(StatementList1* n) override;
	void visit(StatementList2* n) override;
	void visit(StatementSuite1* n) override;
private:
	state current_state;
	static string get_type_debug(const variant<int, string, double>&);
};

#endif //MANGOREVISITEDCPPCLION_INTERPRETER_H