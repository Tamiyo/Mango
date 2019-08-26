// Nodes for the parse tree

#ifndef MANGO_V2_CPP_PARSER_TREE_H
#define MANGO_V2_CPP_PARSER_TREE_H

#include "core.h"
#include "common.h"

// NTS_MANGO -> NTS_STATEMENT_SUITE
struct NodeMango : public Node {
public:
    Node statementsuite;

    Node eval() override {
        return {};
    };

    explicit NodeMango(Node statementsuite) {
        this->statementsuite = std::move(statementsuite);
    }
};

// NTS_STATEMENT_SUITE -> NTS_STATEMENT_LIST
struct NodeStatementSuite : public Node {
public:
    Node statementlist;

    Node eval() override {
        return {};
    };

    explicit NodeStatementSuite(Node statementlist) {
        this->statementlist = std::move(statementlist);
    }
};

// NTS_STATEMENT_SUITE_FUNCTION -> NTS_STATEMENT_LIST_FUNCTION
struct NodeStatementSuiteFunction : public Node {
public:
    Node statementlistfunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementSuiteFunction(Node statementlistfunction) {
        this->statementlistfunction = std::move(statementlistfunction);
    }
};

// NTS_STATEMENT_SUITE_CLASS -> NTS_STATEMENT_LIST_CLASS
struct NodeStatementSuiteClass : public Node {
public:
    Node statementlistclass;

    Node eval() override {
        return {};
    };

    explicit NodeStatementSuiteClass(Node statementlistclass) {
        this->statementlistclass = std::move(statementlistclass);
    }
};

// NTS_STATEMENT_LIST -> NTS_STATEMENT TS_NEWLINE NTS_STATEMENT_LIST
struct NodeStatementList : public Node {
public:
    Node statement;
    Node statementlist;

    Node eval() override {
        return {};
    };

    explicit NodeStatementList(Node statement, Node statementlist) {
        this->statement = std::move(statement);
        this->statementlist = std::move(statementlist);
    }
};

// NTS_STATEMENT_LIST -> NTS_STATEMENT
struct NodeStatementList_Production1 : public Node {
public:
    Node statement;

    Node eval() override {
        return {};
    };

    explicit NodeStatementList_Production1(Node statement) {
        this->statement = std::move(statement);
    }
};

// NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED NTS_STATEMENT_LIST_FUNCTION
struct NodeStatementListFunction : public Node {
public:
    Node statementlimited;
    Node statementlistfunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementListFunction(Node statementlimited, Node statementlistfunction) {
        this->statementlimited = std::move(statementlimited);
        this->statementlistfunction = std::move(statementlistfunction);
    }
};

// NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED
struct NodeStatementListFunction_Production1 : public Node {
public:
    Node statementlimited;

    Node eval() override {
        return {};
    };

    explicit NodeStatementListFunction_Production1(Node statementlimited) {
        this->statementlimited = std::move(statementlimited);
    }
};

// NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED NTS_STATEMENT_LIST_CLASS
struct NodeStatementListClass : public Node {
public:
    Node statementrestricted;
    Node statementlistclass;

    Node eval() override {
        return {};
    };

    explicit NodeStatementListClass(Node statementrestricted, Node statementlistclass) {
        this->statementrestricted = std::move(statementrestricted);
        this->statementlistclass = std::move(statementlistclass);
    }
};

// NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED
struct NodeStatementListClass_Production1 : public Node {
public:
    Node statementrestricted;

    Node eval() override {
        return {};
    };

    explicit NodeStatementListClass_Production1(Node statementrestricted) {
        this->statementrestricted = std::move(statementrestricted);
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_SIMPLE
struct NodeStatement : public Node {
public:
    Node statementsimple;

    Node eval() override {
        return {};
    };

    explicit NodeStatement(Node statementsimple) {
        this->statementsimple = std::move(statementsimple);
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_COMPLEX
struct NodeStatement_Production1 : public Node {
public:
    Node statementcomplex;

    Node eval() override {
        return {};
    };

    explicit NodeStatement_Production1(Node statementcomplex) {
        this->statementcomplex = std::move(statementcomplex);
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_FUNCTION
struct NodeStatement_Production2 : public Node {
public:
    Node statementfunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatement_Production2(Node statementfunction) {
        this->statementfunction = std::move(statementfunction);
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_CLASS
struct NodeStatement_Production3 : public Node {
public:
    Node statementclass;

    Node eval() override {
        return {};
    };

    explicit NodeStatement_Production3(Node statementclass) {
        this->statementclass = std::move(statementclass);
    }
};

// NTS_STATEMENT_LIMITED -> NTS_STATEMENT_SIMPLE
struct NodeStatementLimited : public Node {
public:
    Node statementsimple;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLimited(Node statementsimple) {
        this->statementsimple = std::move(statementsimple);
    }
};

// NTS_STATEMENT_LIMITED -> NTS_STATEMENT_COMPLEX
struct NodeStatementLimited_Production1 : public Node {
public:
    Node statementcomplex;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLimited_Production1(Node statementcomplex) {
        this->statementcomplex = std::move(statementcomplex);
    }
};

// NTS_STATEMENT_RESTRICTED -> NTS_STATEMENT_FUNCTION
struct NodeStatementRestricted : public Node {
public:
    Node statementfunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementRestricted(Node statementfunction) {
        this->statementfunction = std::move(statementfunction);
    }
};

// NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_EXPRESSION
struct NodeStatementSimple : public Node {
public:
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementSimple(Node statementexpression) {
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_ASSIGNMENT
struct NodeStatementSimple_Production1 : public Node {
public:
    Node statementassignment;

    Node eval() override {
        return {};
    };

    explicit NodeStatementSimple_Production1(Node statementassignment) {
        this->statementassignment = std::move(statementassignment);
    }
};

// NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_CONDITIONAL
struct NodeStatementSimple_Production2 : public Node {
public:
    Node statementconditional;

    Node eval() override {
        return {};
    };

    explicit NodeStatementSimple_Production2(Node statementconditional) {
        this->statementconditional = std::move(statementconditional);
    }
};

// NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_LOOP
struct NodeStatementComplex : public Node {
public:
    Node statementloop;

    Node eval() override {
        return {};
    };

    explicit NodeStatementComplex(Node statementloop) {
        this->statementloop = std::move(statementloop);
    }
};

// NTS_STATEMENT_FUNCTION -> TS_AT TS_IDENTIFIER TS_COLON NTS_FUNCTION_PARAMS TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementFunction : public Node {
public:
    Node identifier;
    Node functionparams;
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementFunction(Node identifier, Node functionparams, Node statementsuitefunction) {
        this->identifier = std::move(identifier);
        this->functionparams = std::move(functionparams);
        this->statementsuitefunction = std::move(statementsuitefunction);
    }
};

// NTS_FUNCTION_PARAMS -> NTS_FUNCTION_PARAMS TS_COMMA NTS_STATEMENT_EXPRESSION
struct NodeFunctionParams : public Node {
public:
    Node functionparams;
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeFunctionParams(Node functionparams, Node statementexpression) {
        this->functionparams = std::move(functionparams);
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_FUNCTION_PARAMS -> NTS_STATEMENT_EXPRESSION
struct NodeFunctionParams_Production1 : public Node {
public:
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeFunctionParams_Production1(Node statementexpression) {
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_STATEMENT_CLASS -> TS_AT TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_CLASS TS_RIGHT_CURLY_BRACE
struct NodeStatementClass : public Node {
public:
    Node identifier;
    Node statementsuiteclass;

    Node eval() override {
        return {};
    };

    explicit NodeStatementClass(Node identifier, Node statementsuiteclass) {
        this->identifier = std::move(identifier);
        this->statementsuiteclass = std::move(statementsuiteclass);
    }
};

// NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2 NTS_STATEMENT_EXPRESSION_P
struct NodeStatementExpression : public Node {
public:
    Node statementexpression2;
    Node statementexpressionp;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression(Node statementexpression2, Node statementexpressionp) {
        this->statementexpression2 = std::move(statementexpression2);
        this->statementexpressionp = std::move(statementexpressionp);
    }
};

// NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2
struct NodeStatementExpression_Production1 : public Node {
public:
    Node statementexpression2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression_Production1(Node statementexpression2) {
        this->statementexpression2 = std::move(statementexpression2);
    }
};

// NTS_STATEMENT_EXPRESSION_P -> TS_ADD NTS_STATEMENT_EXPRESSION
struct NodeStatementExpressionP : public Node {
public:
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpressionP(Node statementexpression) {
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_STATEMENT_EXPRESSION_P -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION
struct NodeStatementExpressionP_Production1 : public Node {
public:
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpressionP_Production1(Node statementexpression) {
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3 NTS_STATEMENT_EXPRESSION_2P
struct NodeStatementExpression2 : public Node {
public:
    Node statementexpression3;
    Node statementexpression2p;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2(Node statementexpression3, Node statementexpression2p) {
        this->statementexpression3 = std::move(statementexpression3);
        this->statementexpression2p = std::move(statementexpression2p);
    }
};

// NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3
struct NodeStatementExpression2_Production1 : public Node {
public:
    Node statementexpression3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2_Production1(Node statementexpression3) {
        this->statementexpression3 = std::move(statementexpression3);
    }
};

// NTS_STATEMENT_EXPRESSION_2P -> TS_MULTIPLY NTS_STATEMENT_EXPRESSION_2
struct NodeStatementExpression2p : public Node {
public:
    Node statementexpression2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2p(Node statementexpression2) {
        this->statementexpression2 = std::move(statementexpression2);
    }
};

// NTS_STATEMENT_EXPRESSION_2P -> TS_DIVIDE NTS_STATEMENT_EXPRESSION_2
struct NodeStatementExpression2p_Production1 : public Node {
public:
    Node statementexpression2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2p_Production1(Node statementexpression2) {
        this->statementexpression2 = std::move(statementexpression2);
    }
};

// NTS_STATEMENT_EXPRESSION_2P -> TS_MODULO NTS_STATEMENT_EXPRESSION_2
struct NodeStatementExpression2p_Production2 : public Node {
public:
    Node statementexpression2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression2p_Production2(Node statementexpression2) {
        this->statementexpression2 = std::move(statementexpression2);
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_TERM
struct NodeStatementExpression3 : public Node {
public:
    Node term;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3(Node term) {
        this->term = std::move(term);
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_IDENTIFIER
struct NodeStatementExpression3_Production1 : public Node {
public:
    Node identifier;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3_Production1(Node identifier) {
        this->identifier = std::move(identifier);
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_IDENTIFIER TS_COLON NTS_FUNCTION_PARAMS
struct NodeStatementExpression3_Production2 : public Node {
public:
    Node identifier;
    Node functionparams;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3_Production2(Node identifier, Node functionparams) {
        this->identifier = std::move(identifier);
        this->functionparams = std::move(functionparams);
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_LEFT_PARENTHESIS NTS_STATEMENT_EXPRESSION TS_RIGHT_PARENTHESIS
struct NodeStatementExpression3_Production3 : public Node {
public:
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3_Production3(Node statementexpression) {
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION_3
struct NodeStatementExpression3_Production4 : public Node {
public:
    Node statementexpression3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementExpression3_Production4(Node statementexpression3) {
        this->statementexpression3 = std::move(statementexpression3);
    }
};

// NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS NTS_STATEMENT_EXPRESSION
struct NodeStatementAssignment : public Node {
public:
    Node identifier;
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeStatementAssignment(Node identifier, Node statementexpression) {
        this->identifier = std::move(identifier);
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementConditional : public Node {
public:
    Node conditionalexpression;
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional(Node conditionalexpression, Node statementsuitefunction) {
        this->conditionalexpression = std::move(conditionalexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
    }
};

// NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_2
struct NodeStatementConditional_Production1 : public Node {
public:
    Node conditionalexpression;
    Node statementsuitefunction;
    Node statementconditional2;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional_Production1(Node conditionalexpression, Node statementsuitefunction,
                                                  Node statementconditional2) {
        this->conditionalexpression = std::move(conditionalexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
        this->statementconditional2 = std::move(statementconditional2);
    }
};

// NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
struct NodeStatementConditional_Production2 : public Node {
public:
    Node conditionalexpression;
    Node statementsuitefunction;
    Node statementconditional3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional_Production2(Node conditionalexpression, Node statementsuitefunction,
                                                  Node statementconditional3) {
        this->conditionalexpression = std::move(conditionalexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
        this->statementconditional3 = std::move(statementconditional3);
    }
};

// NTS_STATEMENT_CONDITIONAL_2 -> NTS_STATEMENT_CONDITIONAL_2 TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementConditional2 : public Node {
public:
    Node statementconditional2;
    Node conditionalexpression;
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional2(Node statementconditional2, Node conditionalexpression,
                                       Node statementsuitefunction) {
        this->statementconditional2 = std::move(statementconditional2);
        this->conditionalexpression = std::move(conditionalexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
    }
};

// NTS_STATEMENT_CONDITIONAL_2 -> TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
struct NodeStatementConditional2_Production1 : public Node {
public:
    Node conditionalexpression;
    Node statementsuitefunction;
    Node statementconditional3;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional2_Production1(Node conditionalexpression, Node statementsuitefunction,
                                                   Node statementconditional3) {
        this->conditionalexpression = std::move(conditionalexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
        this->statementconditional3 = std::move(statementconditional3);
    }
};

// NTS_STATEMENT_CONDITIONAL_3 -> TS_ELSE TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementConditional3 : public Node {
public:
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementConditional3(Node statementsuitefunction) {
        this->statementsuitefunction = std::move(statementsuitefunction);
    }
};

// NTS_CONDITIONAL_EXPRESSION -> NTS_STATEMENT_EXPRESSION NTS_COMPARISON_OPERATOR NTS_STATEMENT_EXPRESSION
struct NodeConditionalExpression : public Node {
public:
    Node statementexpression;
    Node comparisonoperator;
    Node statementexpression1;

    Node eval() override {
        return {};
    };

    explicit NodeConditionalExpression(Node statementexpression, Node comparisonoperator, Node statementexpression1) {
        this->statementexpression = std::move(statementexpression);
        this->comparisonoperator = std::move(comparisonoperator);
        this->statementexpression1 = std::move(statementexpression1);
    }
};

// NTS_CONDITIONAL_EXPRESSION -> NTS_COMPARISON_OPERATOR_UNARY NTS_STATEMENT_EXPRESSION
struct NodeConditionalExpression_Production1 : public Node {
public:
    Node comparisonoperatorunary;
    Node statementexpression;

    Node eval() override {
        return {};
    };

    explicit NodeConditionalExpression_Production1(Node comparisonoperatorunary, Node statementexpression) {
        this->comparisonoperatorunary = std::move(comparisonoperatorunary);
        this->statementexpression = std::move(statementexpression);
    }
};

// NTS_COMPARISON_OPERATOR -> TS_LESS_THAN
struct NodeComparisonOperator : public Node {
public:
    Node lessthan;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperator(Node lessthan) {
        this->lessthan = std::move(lessthan);
    }
};

// NTS_COMPARISON_OPERATOR -> TS_LESS_THAN_EQUALS
struct NodeComparisonOperator_Production1 : public Node {
public:
    Node lessthanequals;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperator_Production1(Node lessthanequals) {
        this->lessthanequals = std::move(lessthanequals);
    }
};

// NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN
struct NodeComparisonOperator_Production2 : public Node {
public:
    Node greaterthan;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperator_Production2(Node greaterthan) {
        this->greaterthan = std::move(greaterthan);
    }
};

// NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN_EQUALS
struct NodeComparisonOperator_Production3 : public Node {
public:
    Node greaterthanequals;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperator_Production3(Node greaterthanequals) {
        this->greaterthanequals = std::move(greaterthanequals);
    }
};

// NTS_COMPARISON_OPERATOR -> TS_DOUBLE_EQUALS
struct NodeComparisonOperator_Production4 : public Node {
public:
    Node doubleequals;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperator_Production4(Node doubleequals) {
        this->doubleequals = std::move(doubleequals);
    }
};

// NTS_COMPARISON_OPERATOR -> TS_TRIPLE_EQUALS
struct NodeComparisonOperator_Production5 : public Node {
public:
    Node tripleequals;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperator_Production5(Node tripleequals) {
        this->tripleequals = std::move(tripleequals);
    }
};

// NTS_COMPARISON_OPERATOR_UNARY -> TS_NEGATION
struct NodeComparisonOperatorUnary : public Node {
public:
    Node negation;

    Node eval() override {
        return {};
    };

    explicit NodeComparisonOperatorUnary(Node negation) {
        this->negation = std::move(negation);
    }
};

// NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_FOR
struct NodeStatementLoop : public Node {
public:
    Node statementloopfor;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoop(Node statementloopfor) {
        this->statementloopfor = std::move(statementloopfor);
    }
};

// NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_WHILE
struct NodeStatementLoop_Production1 : public Node {
public:
    Node statementloopwhile;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoop_Production1(Node statementloopwhile) {
        this->statementloopwhile = std::move(statementloopwhile);
    }
};

// NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON NTS_STATEMENT_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementLoopFor : public Node {
public:
    Node identifier;
    Node statementexpression;
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoopFor(Node identifier, Node statementexpression, Node statementsuitefunction) {
        this->identifier = std::move(identifier);
        this->statementexpression = std::move(statementexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
    }
};

// NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON NTS_STATEMENT_EXPRESSION TS_ELLIPSIS NTS_STATEMENT_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementLoopFor_Production1 : public Node {
public:
    Node identifier;
    Node statementexpression;
    Node statementexpression1;
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoopFor_Production1(Node identifier, Node statementexpression, Node statementexpression1,
                                              Node statementsuitefunction) {
        this->identifier = std::move(identifier);
        this->statementexpression = std::move(statementexpression);
        this->statementexpression1 = std::move(statementexpression1);
        this->statementsuitefunction = std::move(statementsuitefunction);
    }
};

// NTS_STATEMENT_LOOP_WHILE -> TS_WHILE NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementLoopWhile : public Node {
public:
    Node conditionalexpression;
    Node statementsuitefunction;

    Node eval() override {
        return {};
    };

    explicit NodeStatementLoopWhile(Node conditionalexpression, Node statementsuitefunction) {
        this->conditionalexpression = std::move(conditionalexpression);
        this->statementsuitefunction = std::move(statementsuitefunction);
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
