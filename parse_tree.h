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

    explicit NodeStatementList(Node statement) {
        this->statement = std::move(statement);
    }
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

    explicit NodeStatementListFunctionRecursive(Node statement_limited, Node statement_list_function) {
        this->statement_limited = std::move(statement_limited);
        this->statement_list_function = std::move(statement_list_function);
    }
};

struct NodeStatementListFunction : public Node {
public:
    Node statement_limited;

    Node eval() override {
        return statement_limited.eval();
    };

    explicit NodeStatementListFunction(Node statement_limited) {
        this->statement_limited = std::move(statement_limited);
    }
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

    explicit NodeStatementListClassRecursive(Node statement_restricted, Node statement_list_class) {
        this->statement_restricted = std::move(statement_restricted);
        this->statement_list_class = std::move(statement_list_class);
    }
};

struct NodeStatementListClass : public Node {
public:
    Node statement_restricted;

    Node eval() override {
        return statement_restricted.eval();
    };

    explicit NodeStatementListClass(Node statement_restricted) {
        this->statement_restricted = std::move(statement_restricted);
    }
};

struct NodeStatement : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };

    explicit NodeStatement(Node statement_x) {
        this->statement_x = std::move(statement_x);
    }
};

struct NodeStatementLimited : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };

    explicit NodeStatementLimited(Node statement_x) {
        this->statement_x = std::move(statement_x);
    }
};

struct NodeStatementRestricted : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };

    explicit NodeStatementRestricted(Node statement_x) {
        this->statement_x = std::move(statement_x);
    }
};

struct NodeStatementSimple : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };

    explicit NodeStatementSimple(Node statement_x) {
        this->statement_x = std::move(statement_x);
    }
};

struct NodeStatementComplex : public Node {
public:
    Node statement_x;

    Node eval() override {
        return statement_x.eval();
    };

    explicit NodeStatementComplex(Node statement_x) {
        this->statement_x = std::move(statement_x);
    }
};

struct NodeStatementFunction : public Node {
public:
    Node identifier;
    Node function_params;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };

    explicit NodeStatementFunction(Node identifier, Node function_params, Node statement_suite_function) {
        this->identifier = std::move(identifier);
        this->function_params = std::move(function_params);
        this->statement_suite_function = std::move(statement_suite_function);
    }
};

struct NodeFunctionParamsRecursive : public Node {
public:
    Node function_params;
    Node identifier;

    Node eval() override {
        return {};
    };

    explicit NodeFunctionParamsRecursive(Node function_params, Node identifier) {
        this->function_params = std::move(function_params);
        this->identifier = std::move(identifier);
    }
};

struct NodeFunctionParams : public Node {
public:
    Node identifier;

    Node eval() override {
        return {};
    };

    explicit NodeFunctionParams(Node identifier) {
        this->identifier = std::move(identifier);
    }
};

struct NodeStatementClass : public Node {
public:
    Node identifier;
    Node statement_suite_class;

    Node eval() override {
        return {};
    };

    explicit NodeStatementClass(Node identifier, Node statement_suite_class) {
        this->identifier = std::move(identifier);
        this->statement_suite_class = std::move(statement_suite_class);
    }
};

struct NodeStatementExpressionRecursive : public Node {
public:
    Node statement_expression_2;
    Node statement_expression_p;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpressionRecursive(Node statement_expression_2, Node statement_expression_p) {
        this->statement_expression_2 = std::move(statement_expression_2);
        this->statement_expression_p = std::move(statement_expression_p);
    }
};

struct NodeStatementExpression : public Node {
public:
    Node statement_expression_2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression(Node statement_expression_2) {
        this->statement_expression_2 = std::move(statement_expression_2);
    }
};

struct NodeStatementExpressionP : public Node {
public:
    Node statement_expression;
    TokenType op;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpressionP(Node statement_expression, TokenType op) {
        this->statement_expression = std::move(statement_expression);
        this->op = op;
    }
};

struct NodeStatementExpression2Recursive : public Node {
public:
    Node statement_expression_3;
    Node statement_expression_2p;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2Recursive(Node statement_expression_3, Node statement_expression_2p) {
        this->statement_expression_3 = std::move(statement_expression_3);
        this->statement_expression_2p = std::move(statement_expression_2p);
    }
};

struct NodeStatementExpression2 : public Node {
public:
    Node statement_expression_3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2(Node statement_expression_3) {
        this->statement_expression_3 = std::move(statement_expression_3);
    }
};

struct NodeStatementExpression2P : public Node {
public:
    Node statement_expression_2;
    TokenType op;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2P(Node statement_expression_2, TokenType op) {
        this->statement_expression_2 = std::move(statement_expression_2);
        this->op = op;
    }
};

struct NodeStatementExpression3Negation : public Node {
public:
    Node statement_expression_3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3Negation(Node statement_expression_3) {
        this->statement_expression_3 = std::move(statement_expression_3);
    }
};

struct NodeStatementExpression3Paren : public Node {
public:
    Node statement_expression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3Paren(Node statement_expression) {
        this->statement_expression = std::move(statement_expression);
    }
};

struct NodeStatementExpression3 : public Node {
public:
    Node statement_x;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3(Node statement_x) {
        this->statement_x = std::move(statement_x);
    }
};

struct NodeStatementExpression3Function : public Node {
public:
    Node function_params;
    Node identifier;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3Function(Node function_params, Node identifier) {
        this->function_params = std::move(function_params);
        this->identifier = std::move(identifier);
    }
};

struct NodeStatementAssignment : public Node {
public:
    Node identifier;
    Node statement_expression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementAssignment(Node identifier, Node statement_expression) {
        this->identifier = std::move(identifier);
        this->statement_expression = std::move(statement_expression);
    }
};

struct NodeStatementConditional : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional(Node conditional_expression, Node statement_suite_function) {
        this->conditional_expression = std::move(conditional_expression);
        this->statement_suite_function = std::move(statement_suite_function);
    }
};

struct NodeStatementConditionalW2 : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;
    Node statement_conditional_2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditionalW2(Node conditional_expression, Node statement_suite_function,
                                        Node statement_conditional_2) {
        this->conditional_expression = std::move(conditional_expression);
        this->statement_suite_function = std::move(statement_suite_function);
        this->statement_conditional_2 = std::move(statement_conditional_2);
    }
};

struct NodeStatementConditionalW3 : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;
    Node statement_conditional_3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditionalW3(Node conditional_expression, Node statement_suite_function,
                                        Node statement_conditional_3) {
        this->conditional_expression = std::move(conditional_expression);
        this->statement_suite_function = std::move(statement_suite_function);
        this->statement_conditional_3 = std::move(statement_conditional_3);
    }
};

struct NodeStatementConditional2Recursive : public Node {
public:
    Node statement_conditional_2;
    Node conditional_expression;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional2Recursive(Node statement_conditional_2, Node conditional_expression,
                                                Node statement_suite_function) {
        this->statement_conditional_2 = std::move(statement_conditional_2);
        this->conditional_expression = std::move(conditional_expression);
        this->statement_suite_function = std::move(statement_suite_function);
    }
};

struct NodeStatementConditional2 : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;
    Node statement_conditional_3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional2(Node conditional_expression, Node statement_suite_function,
                                       Node statement_conditional_3) {
        this->conditional_expression = std::move(conditional_expression);
        this->statement_suite_function = std::move(statement_suite_function);
        this->statement_conditional_3 = std::move(statement_conditional_3);
    }
};

struct NodeStatementConditional3 : public Node {
public:
    Node statement_suite_function;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional3(Node statement_suite_function) {
        this->statement_suite_function = std::move(statement_suite_function);
    }
};

struct NodeConditionalOperator : public Node {
public:
    Node term;

    Node eval() override {
        return {};
    };

    explicit NodeConditionalOperator(Node term) {
        this->term = std::move(term);
    }
};

struct NodeConditionalExpression : public Node {
public:
    Node term1;
    Node comparison_operator;
    Node term2;

    Node eval() override {
        return {};
    };

    explicit NodeConditionalExpression(Node term1, Node comparison_operator,
                                       Node term2) {
        this->term1 = std::move(term1);
        this->comparison_operator = std::move(comparison_operator);
        this->term2 = std::move(term2);
    }
};

struct NodeConditionalExpressionUnary : public Node {
public:
    Node comparison_operator_unary;
    Node term;

    Node eval() override {
        return {};
    };

    explicit NodeConditionalExpressionUnary(Node comparison_operator_unary, Node term) {
        this->comparison_operator_unary = std::move(comparison_operator_unary);
        this->term = std::move(term);
    }
};

struct NodeComparisonOperatorUnary : public Node {
public:
    Node op;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperatorUnary(Node op) {
        this->op = std::move(op);
    }
};

struct NodeStatementLoop : public Node {
public:
    Node statement_loop;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoop(Node statement_loop) {
        this->statement_loop = std::move(statement_loop);
    }
};

struct NodeStatementLoopFor : public Node {
public:
    Node identifier;
    Node term;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoopFor(Node identifier, Node term,
                                  Node statement_suite_function) {
        this->identifier = std::move(identifier);
        this->term = std::move(term);
        this->statement_suite_function = std::move(statement_suite_function);
    }
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

    explicit NodeStatementLoopFor2(Node identifier, Node term1,
                                   Node term2, Node statement_suite_function) {
        this->identifier = std::move(identifier);
        this->term1 = std::move(term1);
        this->term2 = std::move(term2);
        this->statement_suite_function = std::move(statement_suite_function);
    }
};

struct NodeStatementLoopWhile : public Node {
public:
    Node conditional_expression;
    Node statement_suite_function;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoopWhile(Node conditional_expression, Node statement_suite_function) {
        this->conditional_expression = std::move(conditional_expression);
        this->statement_suite_function = std::move(statement_suite_function);
    }
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
