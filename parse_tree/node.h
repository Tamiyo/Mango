#ifndef MANGO_CL_NODE_H
#define MANGO_CL_NODE_H
#include<string>
#include<iostream>
using std::string; using std::cout; using std::endl;
class node {};
class TS_IDENTIFIER : public node {public:    string identifier;    explicit TS_IDENTIFIER (string identifier) {        this->identifier = std::move(identifier);    }};class TS_TERM : public node {public:    string term;    int infered_type;    explicit TS_TERM (string term, int infered_type) {        this->term = std::move(term);        this->infered_type = infered_type;    }};

class NTS_MANGO_NTS_STATEMENT_SUITE : public node {
public:
	node NTS_STATEMENT_SUITE;

	explicit NTS_MANGO_NTS_STATEMENT_SUITE (node NTS_STATEMENT_SUITE) {
		cout << "NTS_MANGO" << endl;
		this->NTS_STATEMENT_SUITE = NTS_STATEMENT_SUITE;
	}
};


class NTS_STATEMENT_SUITE_NTS_STATEMENT_LIST_TS_SYMBOL_NEWLINE : public node {
public:
	node NTS_STATEMENT_LIST;

	explicit NTS_STATEMENT_SUITE_NTS_STATEMENT_LIST_TS_SYMBOL_NEWLINE (node NTS_STATEMENT_LIST) {
		cout << "NTS_STATEMENT_SUITE" << endl;
		this->NTS_STATEMENT_LIST = NTS_STATEMENT_LIST;
	}
};


class NTS_STATEMENT_LIST_NTS_STATEMENT_TS_SYMBOL_NEWLINE_NTS_STATEMENT_LIST : public node {
public:
	node NTS_STATEMENT;
	node NTS_STATEMENT_LIST;

	explicit NTS_STATEMENT_LIST_NTS_STATEMENT_TS_SYMBOL_NEWLINE_NTS_STATEMENT_LIST (node NTS_STATEMENT, node NTS_STATEMENT_LIST) {
		cout << "NTS_STATEMENT_LIST" << endl;
		this->NTS_STATEMENT = NTS_STATEMENT;
		this->NTS_STATEMENT_LIST = NTS_STATEMENT_LIST;
	}
};


class NTS_STATEMENT_LIST_NTS_STATEMENT : public node {
public:
	node NTS_STATEMENT;

	explicit NTS_STATEMENT_LIST_NTS_STATEMENT (node NTS_STATEMENT) {
		cout << "NTS_STATEMENT_LIST" << endl;
		this->NTS_STATEMENT = NTS_STATEMENT;
	}
};


class NTS_STATEMENT_NTS_STATEMENT_SIMPLE : public node {
public:
	node NTS_STATEMENT_SIMPLE;

	explicit NTS_STATEMENT_NTS_STATEMENT_SIMPLE (node NTS_STATEMENT_SIMPLE) {
		cout << "NTS_STATEMENT" << endl;
		this->NTS_STATEMENT_SIMPLE = NTS_STATEMENT_SIMPLE;
	}
};


class NTS_STATEMENT_SIMPLE_NTS_STATEMENT_ASSIGNMENT : public node {
public:
	node NTS_STATEMENT_ASSIGNMENT;

	explicit NTS_STATEMENT_SIMPLE_NTS_STATEMENT_ASSIGNMENT (node NTS_STATEMENT_ASSIGNMENT) {
		cout << "NTS_STATEMENT_SIMPLE" << endl;
		this->NTS_STATEMENT_ASSIGNMENT = NTS_STATEMENT_ASSIGNMENT;
	}
};


class NTS_STATEMENT_ASSIGNMENT_TS_IDENTIFIER_TS_OPERATOR_EQUALS_TS_TERM : public node {
public:
	node TS_IDENTIFIER;
	node TS_TERM;

	explicit NTS_STATEMENT_ASSIGNMENT_TS_IDENTIFIER_TS_OPERATOR_EQUALS_TS_TERM (node TS_IDENTIFIER, node TS_TERM) {
		cout << "NTS_STATEMENT_ASSIGNMENT" << endl;
		this->TS_IDENTIFIER = TS_IDENTIFIER;
		this->TS_TERM = TS_TERM;
	}
};

#endif //MANGO_CL_NODE_H