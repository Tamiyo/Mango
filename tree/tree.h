#ifndef MANGOREVISITEDCPPCLION_TREE_H
#define MANGOREVISITEDCPPCLION_TREE_H

#include "string"

using std::string;

class Identifier;
class StringLiteral;
class IntegerLiteral;
class DoubleLiteral;
class Assignment1;
class BaseExpression1;
class BaseExpression2;
class BaseExpression3;
class BaseExpression4;
class BaseExpression5;
class ComplexStatement1;
class ComplexStatement2;
class ComplexStatement3;
class ComplexStatement4;
class ConditionalExpression1;
class ConditionalExpression2;
class ConditionalExpression3;
class ConditionalExpression4;
class ConditionalExpression5;
class ConditionalExpression6;
class ConditionalExpression7;
class ConditionalExpression8;
class ConditionalExpression9;
class Expression1;
class Expression2;
class Expression3;
class ForLoop1;
class ForLoop2;
class Literal1;
class Literal2;
class Literal3;
class Mango1;
class MultiplicativeExpression1;
class MultiplicativeExpression2;
class MultiplicativeExpression3;
class MultiplicativeExpression4;
class MultiplicativeExpression5;
class MultiplicativeExpression6;
class NewlineLoop1;
class NewlineLoop2;
class Print1;
class SimpleStatement1;
class Statement1;
class Statement2;
class StatementList1;
class StatementList2;
class StatementList3;
class StatementSuite1;
class StatementSuite2;
class WhileLoop1;

class Visitor {
public:
	virtual void visit(Identifier *n) { return; };
	virtual void visit(StringLiteral *n) { return; };
	virtual void visit(DoubleLiteral *n) { return; };
	virtual void visit(IntegerLiteral *n) { return; };
	virtual void visit(Assignment1* n) { return; };
	virtual void visit(BaseExpression1* n) { return; };
	virtual void visit(BaseExpression2* n) { return; };
	virtual void visit(BaseExpression3* n) { return; };
	virtual void visit(BaseExpression4* n) { return; };
	virtual void visit(BaseExpression5* n) { return; };
	virtual void visit(ComplexStatement1* n) { return; };
	virtual void visit(ComplexStatement2* n) { return; };
	virtual void visit(ComplexStatement3* n) { return; };
	virtual void visit(ComplexStatement4* n) { return; };
	virtual void visit(ConditionalExpression1* n) { return; };
	virtual void visit(ConditionalExpression2* n) { return; };
	virtual void visit(ConditionalExpression3* n) { return; };
	virtual void visit(ConditionalExpression4* n) { return; };
	virtual void visit(ConditionalExpression5* n) { return; };
	virtual void visit(ConditionalExpression6* n) { return; };
	virtual void visit(ConditionalExpression7* n) { return; };
	virtual void visit(ConditionalExpression8* n) { return; };
	virtual void visit(ConditionalExpression9* n) { return; };
	virtual void visit(Expression1* n) { return; };
	virtual void visit(Expression2* n) { return; };
	virtual void visit(Expression3* n) { return; };
	virtual void visit(ForLoop1* n) { return; };
	virtual void visit(ForLoop2* n) { return; };
	virtual void visit(Literal1* n) { return; };
	virtual void visit(Literal2* n) { return; };
	virtual void visit(Literal3* n) { return; };
	virtual void visit(Mango1* n) { return; };
	virtual void visit(MultiplicativeExpression1* n) { return; };
	virtual void visit(MultiplicativeExpression2* n) { return; };
	virtual void visit(MultiplicativeExpression3* n) { return; };
	virtual void visit(MultiplicativeExpression4* n) { return; };
	virtual void visit(MultiplicativeExpression5* n) { return; };
	virtual void visit(MultiplicativeExpression6* n) { return; };
	virtual void visit(NewlineLoop1* n) { return; };
	virtual void visit(NewlineLoop2* n) { return; };
	virtual void visit(Print1* n) { return; };
	virtual void visit(SimpleStatement1* n) { return; };
	virtual void visit(Statement1* n) { return; };
	virtual void visit(Statement2* n) { return; };
	virtual void visit(StatementList1* n) { return; };
	virtual void visit(StatementList2* n) { return; };
	virtual void visit(StatementList3* n) { return; };
	virtual void visit(StatementSuite1* n) { return; };
	virtual void visit(StatementSuite2* n) { return; };
	virtual void visit(WhileLoop1* n) { return; };
};

class Node {
public:
    virtual void accept(Visitor *v) = 0;
};

class Identifier : public Node {
public:
    string f0;

    explicit Identifier(string n0) {
        f0 = std::move(n0);
    }

    void accept(Visitor *v) override {
        v->visit(this);
    }
};
class StringLiteral : public Node {
public:
    string f0;

    explicit StringLiteral(string n0) {
        f0 = std::move(n0);
    }

    void accept(Visitor *v) override {
        v->visit(this);
    }
};

class DoubleLiteral : public Node {
public:
    double f0;

    explicit DoubleLiteral(string n0) {
        f0 = std::stod(n0);
    }

    void accept(Visitor *v) override {
        v->visit(this);
    }
};

class IntegerLiteral : public Node {
public:
    int f0;

    explicit IntegerLiteral(string n0) {
        f0 = std::stoi(n0);
    }

    void accept(Visitor *v) override {
        v->visit(this);
    }
};

class Assignment1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit Assignment1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class BaseExpression1 : public Node {
public:
	Node* n1;
	explicit BaseExpression1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class BaseExpression2 : public Node {
public:
	Node* n1;
	explicit BaseExpression2(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class BaseExpression3 : public Node {
public:
	Node* n1;
	explicit BaseExpression3(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class BaseExpression4 : public Node {
public:
	Node* n1;
	explicit BaseExpression4(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class BaseExpression5 : public Node {
public:
	Node* n1;
	explicit BaseExpression5(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ComplexStatement1 : public Node {
public:
	Node* n1;
	explicit ComplexStatement1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ComplexStatement2 : public Node {
public:
	Node* n1;
	explicit ComplexStatement2(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ComplexStatement3 : public Node {
public:
	Node* n1;
	explicit ComplexStatement3(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ComplexStatement4 : public Node {
public:
	Node* n1;
	explicit ComplexStatement4(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression2 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression2(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression3 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression3(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression4 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression4(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression5 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression5(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression6 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression6(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression7 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression7(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression8 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit ConditionalExpression8(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ConditionalExpression9 : public Node {
public:
	Node* n1;
	explicit ConditionalExpression9(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Expression1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit Expression1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Expression2 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit Expression2(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Expression3 : public Node {
public:
	Node* n1;
	explicit Expression3(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ForLoop1 : public Node {
public:
	Node* n1;
	Node* n2;
	Node* n3;
	explicit ForLoop1(Node* a1, Node* a2, Node* a3) {
		n1 = a1;
		n2 = a2;
		n3 = a3;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class ForLoop2 : public Node {
public:
	Node* n1;
	Node* n2;
	Node* n3;
	Node* n4;
	explicit ForLoop2(Node* a1, Node* a2, Node* a3, Node* a4) {
		n1 = a1;
		n2 = a2;
		n3 = a3;
		n4 = a4;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Literal1 : public Node {
public:
	Node* n1;
	explicit Literal1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Literal2 : public Node {
public:
	Node* n1;
	explicit Literal2(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Literal3 : public Node {
public:
	Node* n1;
	explicit Literal3(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Mango1 : public Node {
public:
	Node* n1;
	explicit Mango1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class MultiplicativeExpression1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit MultiplicativeExpression1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class MultiplicativeExpression2 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit MultiplicativeExpression2(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class MultiplicativeExpression3 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit MultiplicativeExpression3(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class MultiplicativeExpression4 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit MultiplicativeExpression4(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class MultiplicativeExpression5 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit MultiplicativeExpression5(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class MultiplicativeExpression6 : public Node {
public:
	Node* n1;
	explicit MultiplicativeExpression6(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class NewlineLoop1 : public Node {
public:
	Node* n1;
	explicit NewlineLoop1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class NewlineLoop2 : public Node {
public:
	explicit NewlineLoop2() {
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Print1 : public Node {
public:
	Node* n1;
	explicit Print1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class SimpleStatement1 : public Node {
public:
	Node* n1;
	explicit SimpleStatement1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Statement1 : public Node {
public:
	Node* n1;
	explicit Statement1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class Statement2 : public Node {
public:
	Node* n1;
	explicit Statement2(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementList1 : public Node {
public:
	Node* n1;
	Node* n2;
	Node* n3;
	explicit StatementList1(Node* a1, Node* a2, Node* a3) {
		n1 = a1;
		n2 = a2;
		n3 = a3;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementList2 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit StatementList2(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementList3 : public Node {
public:
	Node* n1;
	explicit StatementList3(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementSuite1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit StatementSuite1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementSuite2 : public Node {
public:
	Node* n1;
	explicit StatementSuite2(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class WhileLoop1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit WhileLoop1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

#endif //MANGOREVISITEDCPPCLION_TREE_H