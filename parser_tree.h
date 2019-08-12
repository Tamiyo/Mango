#include <utility>

#include <utility>

#include <utility>

#include <utility>

#include <utility>

#include <utility>

//
// Created by Matt on 8/11/2019.
//

#ifndef MANGO_V2_CPP_PARSER_TREE_H
#define MANGO_V2_CPP_PARSER_TREE_H

#include "core.h"
#include "common.h"

struct NodeMango : public Node {
public:
    Node statement_suite;

    Node eval() override {
        return statement_suite.eval();
    };

    explicit NodeMango(Node statement_suite) {
        this->statement_suite = std::move(statement_suite);
    }
};

struct NodeStatementSuite : public Node {
public:
    Node statement_list;

    Node eval() override {
        return statement_list.eval();
    };

    explicit NodeStatementSuite(Node statement_list) {
        this->statement_list = std::move(statement_list);
    }
};

struct NodeStatementSuiteFunction : public Node {
public:
    Node statement_list_function;

    Node eval() override {
        return statement_list_function.eval();
    };

    explicit NodeStatementSuiteFunction(Node statement_list_function) {
        this->statement_list_function = std::move(statement_list_function);
    }
};

struct NodeStatementSuiteClass : public Node {
public:
    Node statement_list_class;

    Node eval() override {
        return statement_list_class.eval();
    };

    explicit NodeStatementSuiteClass(Node statement_list_class) {
        this->statement_list_class = std::move(statement_list_class);
    }
};

struct NodeStatementListRecursive : public Node {
public:
    Node statement;
    Node statement_list;

    Node eval() override {
        statement.eval();
        statement_list.eval();
        return {};
    };

    explicit NodeStatementListRecursive(Node statement, Node statement_list) {
        this->statement = std::move(statement);
        this->statement_list = std::move(statement_list);
    }
};

struct NodeStatementList : public Node {
public:
    Node statement;

    Node eval() override {
        return statement.eval();
    };
};

struct NodeStatementListFunctionRecursive : public Node {
public:
    Node statement_limited;
    Node statement_list_function;

    Node eval() override {
        statement_limited.eval();
        statement_list_function.eval();
        return {};
    };
};

struct NodeStatementListFunction : public Node {
public:
    Node statement_limited;

    Node eval() override {
        return statement_limited.eval();
    };
};

struct NodeStatementListClassRecursive : public Node {
public:
    Node statement_restricted;
    Node statement_list_class;

    Node eval() override {
        statement_restricted.eval();
        statement_list_class.eval();
        return {};
    };
};

struct NodeStatementListClass : public Node {
public:
    Node statement_restricted;

    Node eval() override {
        return statement_restricted.eval();
    };
};

struct NodeStatement : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };
};

struct NodeStatementLimited : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };
};

struct NodeStatementRestricted : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };
};

struct NodeStatementSimple : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };
};

struct NodeStatementComplex : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };
};

struct NodeStatementFunction : public Node {
public:
    Node identifier;
    Node function_params;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeFunctionParamsRecursive : public Node {
public:
    Node function_params;
    Node identifier;

    Node eval() override {
        return {};
    };
};

struct NodeFunctionParams : public Node {
public:
    Node identifier;

    Node eval() override {
        return {};
    };
};

struct NodeStatementClass : public Node {
public:
    Node identifier;
    Node statement_suite_class;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpressionRecursive : public Node {
public:
    Node statement_expression_2;
    Node statement_expression_p;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression : public Node {
public:
    Node statement_expression_2;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpressionP : public Node {
public:
    Node statement_expression;
    Node op;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression2Recursive : public Node {
public:
    Node statement_expression_3;
    Node statement_expression_2p;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression2 : public Node {
public:
    Node statement_expression_3;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression2P : public Node {
public:
    Node statement_expression_2;
    Node op;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression3Negation : public Node {
public:
    Node statement_expression_3;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression3Paren : public Node {
public:
    Node statement_expression;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression3 : public Node {
public:
    Node statement_x;

    Node eval() override {
        return {};
    };
};

struct NodeStatementExpression3Function : public Node {
public:
    Node function_params;
    Node identifier;

    Node eval() override {
        return {};
    };
};

struct NodeStatementAssignment : public Node {
public:
    Node identifier;
    Node statement_expression;

    Node eval() override {
        SCOPED_SYMBOL_TABLE[SCOPE_LEVEL]["x"] = identifier;
        return {};
    };
};

struct NodeStatementConditional : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeStatementConditionalW2 : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;
    Node statement_conditional_2;

    Node eval() override {
        return {};
    };
};

struct NodeStatementConditionalW3 : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;
    Node statement_conditional_3;

    Node eval() override {
        return {};
    };
};

struct NodeStatementConditional2Recursive : public Node {
public:
    Node statement_conditional_2;
    Node conditional_expression;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeStatementConditional2 : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;
    Node statement_conditional_3;

    Node eval() override {
        return {};
    };
};

struct NodeStatementConditional3 : public Node {
public:
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeConditionalOperator : public Node {
public:
    Node term;

    Node eval() override {
        return {};
    };
};

struct NodeConditionalExpression : public Node {
public:
    Node term1;
    Node comparison_operator;
    Node term2;

    Node eval() override {
        return {};
    };
};

struct NodeConditionalExpressionUnary : public Node {
public:
    Node comparison_operator_unary;
    Node term;

    Node eval() override {
        return {};
    };
};

struct NodeComparisonOperatorUnary : public Node {
public:
    Node op;

    Node eval() override {
        return {};
    };
};

struct NodeStatementLoop : public Node {
public:
    Node statement_loop;

    Node eval() override {
        return {};
    };
};

struct NodeStatementLoopFor : public Node {
public:
    Node identifier;
    Node term;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeStatementLoopFor2 : public Node {
public:
    Node identifier;
    Node term1;
    Node term2;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeStatementLoopWhile : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };
};

struct NodeTerm : public Node {
public:
    string token;
    PrimitiveType inferred_type;
    TokenType token_type;

    Node eval() override {
        return *static_cast<Node *>(this);
    };

    NodeTerm() {
        this->token = "";
        this->inferred_type = PrimitiveType::Null;
        this->token_type = TokenType::None;
    }

    NodeTerm(string token, PrimitiveType inferred_type, TokenType token_type) {
        this->token = std::move(token);
        this->inferred_type = inferred_type;
        this->token_type = token_type;
    }
};

struct NodeIdentifier : public Node {
public:
    string token;
    PrimitiveType inferred_type;
    TokenType token_type;

    Node eval() override {
        return *static_cast<Node *>(this);
    };

    NodeIdentifier() {
        this->token = "";
        this->inferred_type = PrimitiveType::Null;
        this->token_type = TokenType::None;
    }

    NodeIdentifier(string token, PrimitiveType inferred_type, TokenType token_type) {
        this->token = std::move(token);
        this->inferred_type = inferred_type;
        this->token_type = token_type;
    }
};

#endif //MANGO_V2_CPP_PARSER_TREE_H
