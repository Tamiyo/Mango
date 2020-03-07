#ifndef MANGOREVISITEDCPPCLION_TREE_H
#define MANGOREVISITEDCPPCLION_TREE_H

#include "string"

using std::string;

class Literal;
class BaseExpression1;
class BaseExpression2;
class BaseExpression3;
class Expression1;
class Expression2;
class Expression3;
class Mango1;
class MultiplicativeExpression1;
class MultiplicativeExpression2;
class MultiplicativeExpression3;
class MultiplicativeExpression4;
class MultiplicativeExpression5;
class MultiplicativeExpression6;
class Print1;
class Statement1;
class Statement2;
class StatementList1;
class StatementList2;
class StatementSuite1;

class Visitor {
public:
	virtual string visit(Literal *n) { return ""; };
	virtual string visit(BaseExpression1* n) { return ""; };
	virtual string visit(BaseExpression2* n) { return ""; };
	virtual string visit(BaseExpression3* n) { return ""; };
	virtual string visit(Expression1* n) { return ""; };
	virtual string visit(Expression2* n) { return ""; };
	virtual string visit(Expression3* n) { return ""; };
	virtual string visit(Mango1* n) { return ""; };
	virtual string visit(MultiplicativeExpression1* n) { return ""; };
	virtual string visit(MultiplicativeExpression2* n) { return ""; };
	virtual string visit(MultiplicativeExpression3* n) { return ""; };
	virtual string visit(MultiplicativeExpression4* n) { return ""; };
	virtual string visit(MultiplicativeExpression5* n) { return ""; };
	virtual string visit(MultiplicativeExpression6* n) { return ""; };
	virtual string visit(Print1* n) { return ""; };
	virtual string visit(Statement1* n) { return ""; };
	virtual string visit(Statement2* n) { return ""; };
	virtual string visit(StatementList1* n) { return ""; };
	virtual string visit(StatementList2* n) { return ""; };
	virtual string visit(StatementSuite1* n) { return ""; };
};

class Node {
public:
    virtual string accept(Visitor *v) = 0;
};

class Literal : public Node {
public:
    string f0;

    explicit Literal(string n0) {
        f0 = std::move(n0);
    }

    string accept(Visitor *v) override {
        return v->visit(this);
    }
};

class BaseExpression1 : public Node {
public:
	Node* n1;
	explicit BaseExpression1(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class BaseExpression2 : public Node {
public:
	Node* n1;
	explicit BaseExpression2(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class BaseExpression3 : public Node {
public:
	Node* n1;
	explicit BaseExpression3(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class Expression3 : public Node {
public:
	Node* n1;
	explicit Expression3(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class Mango1 : public Node {
public:
	Node* n1;
	explicit Mango1(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
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
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class MultiplicativeExpression6 : public Node {
public:
	Node* n1;
	explicit MultiplicativeExpression6(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class Print1 : public Node {
public:
	Node* n1;
	explicit Print1(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class Statement1 : public Node {
public:
	Node* n1;
	explicit Statement1(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class Statement2 : public Node {
public:
	Node* n1;
	explicit Statement2(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class StatementList1 : public Node {
public:
	Node* n1;
	Node* n2;
	explicit StatementList1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class StatementList2 : public Node {
public:
	Node* n1;
	explicit StatementList2(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

class StatementSuite1 : public Node {
public:
	Node* n1;
	explicit StatementSuite1(Node* a1) {
		n1 = a1;
	}
	string accept(Visitor *v) override {
		return v->visit(this);
	}
};

#endif //MANGOREVISITEDCPPCLION_TREE_H