#ifndef MANGOREVISITEDCPPCLION_TREE_H
#define MANGOREVISITEDCPPCLION_TREE_H

#include "string"

using std::string;

class Identifier;
class StringLiteral;
class IntegerLiteral;
class DoubleLiteral;
class BaseExpression1;
class BaseExpression2;
class BaseExpression3;
class Expression1;
class Expression2;
class Expression3;
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
class Print1;
class Statement1;
class Statement2;
class StatementList1;
class StatementList2;
class StatementSuite1;

class Visitor {
public:
	virtual void visit(Identifier *n) { return; };
	virtual void visit(StringLiteral *n) { return; };
	virtual void visit(DoubleLiteral *n) { return; };
	virtual void visit(IntegerLiteral *n) { return; };
	virtual void visit(BaseExpression1* n) { return; };
	virtual void visit(BaseExpression2* n) { return; };
	virtual void visit(BaseExpression3* n) { return; };
	virtual void visit(Expression1* n) { return; };
	virtual void visit(Expression2* n) { return; };
	virtual void visit(Expression3* n) { return; };
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
	virtual void visit(Print1* n) { return; };
	virtual void visit(Statement1* n) { return; };
	virtual void visit(Statement2* n) { return; };
	virtual void visit(StatementList1* n) { return; };
	virtual void visit(StatementList2* n) { return; };
	virtual void visit(StatementSuite1* n) { return; };
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
	explicit StatementList1(Node* a1, Node* a2) {
		n1 = a1;
		n2 = a2;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementList2 : public Node {
public:
	Node* n1;
	explicit StatementList2(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

class StatementSuite1 : public Node {
public:
	Node* n1;
	explicit StatementSuite1(Node* a1) {
		n1 = a1;
	}
	void accept(Visitor *v) override {
		v->visit(this);
	}
};

#endif //MANGOREVISITEDCPPCLION_TREE_H