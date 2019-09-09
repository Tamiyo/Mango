// Nodes for the parse tree

#ifndef MANGO_V2_CPP_PARSER_TREE_H
#define MANGO_V2_CPP_PARSER_TREE_H

#include <algorithm>

#include "core.h"
#include "common.h"
#include "semantic_analyzer.h"

using std::count;
using std::string;
using std::vector;
using std::pair;
using std::cout;
using std::endl;

class NodeTerm : public virtual Node {
public:
    string token;
    PrimitiveType inferred_type;
    TokenType token_type;

    Node *eval() override {
        return static_cast<Node *>(this);
    };

    Node debug() override {
        printf("NodeTerm Output: %s", this->token.c_str());
    }

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

class NodeIdentifier : public virtual Node {
public:
    string token;
    PrimitiveType inferred_type;
    TokenType token_type;

    Node *eval() override {
        return static_cast<Node *>(this);
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

// NTS_MANGO -> NTS_STATEMENT_SUITE
class NodeMango : public virtual Node {
public:
    Node *statementsuite;

    Node *eval() override {
        return statementsuite->eval();
    };

    explicit NodeMango(Node *statementsuite) {
        this->statementsuite = statementsuite;
    }
};

// NTS_STATEMENT_SUITE -> NTS_STATEMENT_LIST
class NodeStatementSuite : public virtual Node {
public:
    Node *statementlist;

    Node *eval() override {
        return statementlist->eval();
    };

    explicit NodeStatementSuite(Node *statementlist) {
        this->statementlist = statementlist;
    }
};

// NTS_STATEMENT_SUITE_FUNCTION -> NTS_STATEMENT_LIST_FUNCTION
class NodeStatementSuiteFunction : public virtual Node {
public:
    Node *statementlistfunction;

    Node *eval() override {
        Node *tmp = statementlistfunction->eval();
        cout << "done with function call" << endl;
        return tmp;
    };

    explicit NodeStatementSuiteFunction(Node *statementlistfunction) {
        this->statementlistfunction = statementlistfunction;
    }
};

// NTS_STATEMENT_SUITE_CLASS -> NTS_STATEMENT_LIST_CLASS
class NodeStatementSuiteClass : public virtual Node {
public:
    Node *statementlistclass;

    Node *eval() override {
        Node *tmp = statementlistclass->eval();
        SCOPE_LEVEL--;
        return tmp;
    };

    explicit NodeStatementSuiteClass(Node *statementlistclass) {
        this->statementlistclass = statementlistclass;
    }
};

// NTS_STATEMENT_LIST -> NTS_STATEMENT TS_NEWLINE NTS_STATEMENT_LIST
class NodeStatementList : public virtual Node {
public:
    Node *statement;
    Node *statementlist;

    Node *eval() override {
        statement->eval();
        statementlist->eval();
        return nullptr;
    };

    explicit NodeStatementList(Node *statement, Node *statementlist) {
        this->statement = statement;
        this->statementlist = statementlist;
    }
};

// NTS_STATEMENT_LIST -> NTS_STATEMENT
class NodeStatementList_Production1 : public virtual Node {
public:
    Node *statement;

    Node *eval() override {
        return statement->eval();
    };

    explicit NodeStatementList_Production1(Node *statement) {
        this->statement = statement;
    }
};

// NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED NTS_STATEMENT_LIST_FUNCTION
class NodeStatementListFunction : public virtual Node {
public:
    Node *statementlimited;
    Node *statementlistfunction;

    // TODO - Apply this to the other recursive statement blocks
    Node *eval() override {
        Node *a = statementlimited->eval();
        Node *b = statementlistfunction->eval();
        if (a) {
            return a;
        } else if (b) {
            return b;
        } else {
            return nullptr;
        }
    };

    explicit NodeStatementListFunction(Node *statementlimited, Node *statementlistfunction) {
        this->statementlimited = statementlimited;
        this->statementlistfunction = statementlistfunction;
    }
};

// NTS_STATEMENT_LIST_FUNCTION -> NTS_STATEMENT_LIMITED
class NodeStatementListFunction_Production1 : public virtual Node {
public:
    Node *statementlimited;

    Node *eval() override {
        return statementlimited->eval();
    };

    explicit NodeStatementListFunction_Production1(Node *statementlimited) {
        this->statementlimited = statementlimited;
    }
};

// NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED NTS_STATEMENT_LIST_CLASS
class NodeStatementListClass : public virtual Node {
public:
    Node *statementrestricted;
    Node *statementlistclass;

    Node *eval() override {
        statementrestricted->eval();
        statementlistclass->eval();
        return nullptr;
    };

    explicit NodeStatementListClass(Node *statementrestricted, Node *statementlistclass) {
        this->statementrestricted = statementrestricted;
        this->statementlistclass = statementlistclass;
    }
};

// NTS_STATEMENT_LIST_CLASS -> NTS_STATEMENT_RESTRICTED
class NodeStatementListClass_Production1 : public virtual Node {
public:
    Node *statementrestricted;

    Node *eval() override {
        return statementrestricted->eval();
    };

    explicit NodeStatementListClass_Production1(Node *statementrestricted) {
        this->statementrestricted = statementrestricted;
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_SIMPLE
class NodeStatement : public virtual Node {
public:
    Node *statementsimple;

    Node *eval() override {
        return statementsimple->eval();
    };

    explicit NodeStatement(Node *statementsimple) {
        this->statementsimple = statementsimple;
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_COMPLEX
class NodeStatement_Production1 : public virtual Node {
public:
    Node *statementcomplex;

    Node *eval() override {
        return statementcomplex->eval();
    };

    explicit NodeStatement_Production1(Node *statementcomplex) {
        this->statementcomplex = statementcomplex;
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_FUNCTION
class NodeStatement_Production2 : public virtual Node {
public:
    Node *statementfunction;

    Node *eval() override {
        return statementfunction->eval();
    };

    explicit NodeStatement_Production2(Node *statementfunction) {
        this->statementfunction = statementfunction;
    }
};

// NTS_STATEMENT -> NTS_STATEMENT_CLASS
class NodeStatement_Production3 : public virtual Node {
public:
    Node *statementclass;

    Node *eval() override {
        return statementclass->eval();
    };

    explicit NodeStatement_Production3(Node *statementclass) {
        this->statementclass = statementclass;
    }
};

// NTS_STATEMENT_LIMITED -> NTS_STATEMENT_SIMPLE
class NodeStatementLimited : public virtual Node {
public:
    Node *statementsimple;

    Node *eval() override {
        return statementsimple->eval();
    };

    explicit NodeStatementLimited(Node *statementsimple) {
        this->statementsimple = statementsimple;
    }
};

// NTS_STATEMENT_LIMITED -> NTS_STATEMENT_COMPLEX
class NodeStatementLimited_Production1 : public virtual Node {
public:
    Node *statementcomplex;

    Node *eval() override {
        return statementcomplex->eval();
    };

    explicit NodeStatementLimited_Production1(Node *statementcomplex) {
        this->statementcomplex = statementcomplex;
    }
};

// NTS_STATEMENT_LIMITED -> NTS_STATEMENT_RETURN
struct NodeStatementLimited_Production2 : virtual Node {
public:
    Node *statementreturn;

    Node *eval() override {
        return statementreturn->eval();
    };

    explicit NodeStatementLimited_Production2(Node *statementreturn) {
        this->statementreturn = statementreturn;
    }
};

// NTS_STATEMENT_RETURN -> TS_RETURN NTS_STATEMENT_EXPRESSION
struct NodeStatementReturn : virtual Node {
public:
    Node *statementexpression;

    Node *eval() override {
        return statementexpression->eval();
    };

    explicit NodeStatementReturn(Node *statementexpression) {
        this->statementexpression = statementexpression;
    }
};

// NTS_STATEMENT_RESTRICTED -> NTS_STATEMENT_FUNCTION
class NodeStatementRestricted : public virtual Node {
public:
    Node *statementfunction;

    Node *eval() override {
        return statementfunction->eval();
    };

    explicit NodeStatementRestricted(Node *statementfunction) {
        this->statementfunction = statementfunction;
    }
};

// NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_EXPRESSION
class NodeStatementSimple : public virtual Node {
public:
    Node *statementexpression;

    Node *eval() override {
        return statementexpression->eval();
    };

    explicit NodeStatementSimple(Node *statementexpression) {
        this->statementexpression = statementexpression;
    }
};

// NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_ASSIGNMENT
class NodeStatementSimple_Production1 : public virtual Node {
public:
    Node *statementassignment;

    Node *eval() override {
        return statementassignment->eval();
    };

    explicit NodeStatementSimple_Production1(Node *statementassignment) {
        this->statementassignment = statementassignment;
    }
};

// NTS_STATEMENT_SIMPLE -> NTS_STATEMENT_CONDITIONAL
class NodeStatementSimple_Production2 : public virtual Node {
public:
    Node *statementconditional;

    Node *eval() override {
        return statementconditional->eval();
    };

    explicit NodeStatementSimple_Production2(Node *statementconditional) {
        this->statementconditional = statementconditional;
    }
};

// NTS_STATEMENT_COMPLEX -> NTS_STATEMENT_LOOP
class NodeStatementComplex : public virtual Node {
public:
    Node *statementloop;

    Node *eval() override {
        return statementloop->eval();
    };

    explicit NodeStatementComplex(Node *statementloop) {
        this->statementloop = statementloop;
    }
};

// NTS_FUNCTION_PARAMS_DEFINE -> TS_IDENTIFIER
struct NodeFunctionParamsDefine_Production1 : virtual Node {
public:
    Node *identifier;

    vector<Node *> eval(vector<Node *> parameters) {
        cout << "in singular defined paramter, stop" << endl;
        parameters.push_back(identifier);
        return parameters;
    };

    explicit NodeFunctionParamsDefine_Production1(Node *identifier) {
        this->identifier = identifier;
    }
};

// NTS_FUNCTION_PARAMS_DEFINE -> NTS_FUNCTION_PARAMS_DEFINE TS_COMMA TS_IDENTIFIER
struct NodeFunctionParamsDefine : public virtual Node {
public:
    Node *identifier;
    Node *functionparamsdefine;

    vector<Node *> eval(vector<Node *> parameters) {
        cout << "In defined params" << endl;
        if (auto *nodeFunctionParamsDefine = dynamic_cast<NodeFunctionParamsDefine *>(functionparamsdefine)) {
            cout << "Recursive?" << endl;
            vector<Node *> tmp = nodeFunctionParamsDefine->eval(parameters);
            tmp.push_back(identifier);
            return tmp;
        } else if (auto *nodeFunctionParamsDefine = dynamic_cast<NodeFunctionParamsDefine_Production1 *>(functionparamsdefine)) {
            cout << "Non-Recursive?" << endl;
            vector<Node *> tmp = nodeFunctionParamsDefine->eval(parameters);
            tmp.push_back(identifier);
            return tmp;
        } else {
            throw "AST function parameter defined error";
        }
    };

    explicit NodeFunctionParamsDefine(Node *identifier, Node *functionparamsdefine) {
        this->identifier = identifier;
        this->functionparamsdefine = functionparamsdefine;
    }
};


// NTS_FUNCTION_PARAMS -> NTS_STATEMENT_EXPRESSION
class NodeFunctionParams_Production1 : public virtual Node {
public:
    Node *statementexpression;

    vector<Node *> eval(vector<Node *> parameters) {
        parameters.push_back(statementexpression->eval());
        return parameters;
    };

    explicit NodeFunctionParams_Production1(Node *statementexpression) {
        this->statementexpression = statementexpression;
    }
};

// NTS_FUNCTION_PARAMS -> NTS_FUNCTION_PARAMS TS_COMMA NTS_STATEMENT_EXPRESSION
class NodeFunctionParams : public virtual Node {
public:
    Node *functionparams;
    Node *statementexpression;

    vector<Node *> eval(vector<Node *> parameters) {
        cout << "in params" << endl;
        if (auto *nodeFunctionParams = dynamic_cast<NodeFunctionParams *>(functionparams)) {
            vector<Node *> tmp = nodeFunctionParams->eval(parameters);
            tmp.push_back(statementexpression->eval());
            return tmp;
        } else if (auto *nodeFunctionParams = dynamic_cast<NodeFunctionParams_Production1 *>(functionparams)) {
            vector<Node *> tmp = nodeFunctionParams->eval(parameters);
            tmp.push_back(statementexpression->eval());
            return tmp;
        }
    };

    explicit NodeFunctionParams(Node *functionparams, Node *statementexpression) {
        this->functionparams = functionparams;
        this->statementexpression = statementexpression;
    }
};


// NTS_STATEMENT_FUNCTION -> TS_AT TS_IDENTIFIER TS_COLON NTS_FUNCTION_PARAMS_DEFINE TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementFunction : public virtual Node {
public:
    Node *identifier;
    Node *functionparamsdefine;
    Node *statementsuitefunction;

    Node *eval() override {
        NodeIdentifier *nodeIdentifierEval = dynamic_cast<NodeIdentifier *>(identifier->eval());
        SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifierEval->token] = this;
        return nullptr;
    };

    explicit NodeStatementFunction(Node *identifier, Node *functionparamsdefine, Node *statementsuitefunction) {
        this->identifier = identifier;
        this->functionparamsdefine = functionparamsdefine;
        this->statementsuitefunction = statementsuitefunction;
    }
};

// NTS_STATEMENT_FUNCTION -> TS_AT TS_IDENTIFIER TS_COLON TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementFunction_Production1 : public virtual Node {
public:
    Node *identifier;
    Node *statementsuitefunction;

    Node *eval() override {
        NodeIdentifier *nodeIdentifierEval = dynamic_cast<NodeIdentifier *>(identifier->eval());

        SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifierEval->token] = this;
        return nullptr;
    };

    explicit NodeStatementFunction_Production1(Node *identifier, Node *statementsuitefunction) {
        this->identifier = identifier;
        this->statementsuitefunction = statementsuitefunction;
    }
};


// NTS_STATEMENT_CLASS -> TS_AT TS_IDENTIFIER TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_CLASS TS_RIGHT_CURLY_BRACE
class NodeStatementClass : public virtual Node {
public:
    Node *identifier;
    Node *statementsuiteclass;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeStatementClass(Node *identifier, Node *statementsuiteclass) {
        this->identifier = identifier;
        this->statementsuiteclass = statementsuiteclass;
    }
};

// NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2
class NodeStatementExpression_Production1 : public virtual Node {
public:
    Node *statementexpression2;

    Node *eval() override {
        cout << "called expr to expr2" << endl;
        return statementexpression2->eval();
    };

    explicit NodeStatementExpression_Production1(Node *statementexpression2) {
        this->statementexpression2 = statementexpression2;
    }
};

// NTS_STATEMENT_EXPRESSION_P -> TS_ADD NTS_STATEMENT_EXPRESSION
class NodeStatementExpressionP : public virtual Node {
public:
    Node *statementexpression;

    Node *eval() override {
        return statementexpression->eval();
    };

    explicit NodeStatementExpressionP(Node *statementexpression) {
        this->statementexpression = statementexpression;
    }
};

// NTS_STATEMENT_EXPRESSION_P -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION
class NodeStatementExpressionP_Production1 : public virtual Node {
public:
    Node *statementexpression;

    Node *eval() override {
        return statementexpression->eval();
    };

    explicit NodeStatementExpressionP_Production1(Node *statementexpression) {
        this->statementexpression = statementexpression;
    }
};

// NTS_STATEMENT_EXPRESSION -> NTS_STATEMENT_EXPRESSION_2 NTS_STATEMENT_EXPRESSION_P
class NodeStatementExpression : public virtual Node {
public:
    Node *statementexpression2;
    Node *statementexpressionp;

    Node *eval() override {
        // This is an add operator
        if (auto *addNode = dynamic_cast<NodeStatementExpressionP *>(statementexpressionp)) {
            auto *statementexpression2Eval = dynamic_cast<NodeTerm *>(statementexpression2->eval());
            auto *addNodeEval = dynamic_cast<NodeTerm *>(addNode->eval());

            PrimitiveType type1 = statementexpression2Eval->inferred_type;
            PrimitiveType type2 = addNodeEval->inferred_type;
            if (type1 == type2) {
                string result = doArithmetic(statementexpression2Eval->token, addNodeEval->token, type1, "+");
                if (result != "") {
                    return new NodeTerm{result, type1, addNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
            } else if (count(primitive_type_conversions[type1].begin(),
                             primitive_type_conversions[type1].end(),
                             type2)) {

                string converted_string = convert(statementexpression2Eval->token, addNodeEval->token, type1, type2);
                string result = doArithmetic(statementexpression2Eval->token, converted_string, type1, "+");
                if (result != "") {
                    return new NodeTerm{result, type1, addNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
                return nullptr;
            } else {
                throw "Conversion Exception: Cannot convert expression to different type.";
                return nullptr;
            }
        }
            // This is an sub operator
        else if (auto *subNode = dynamic_cast<NodeStatementExpressionP_Production1 *>(statementexpressionp)) {
            auto *statementexpression2Eval = dynamic_cast<NodeTerm *>(statementexpression2->eval());
            auto *subNodeEval = dynamic_cast<NodeTerm *>(subNode->eval());

            PrimitiveType type1 = statementexpression2Eval->inferred_type;
            PrimitiveType type2 = subNodeEval->inferred_type;
            if (type1 == type2) {
                string result = doArithmetic(statementexpression2Eval->token, subNodeEval->token, type1, "-");
                if (result != "") {
                    return new NodeTerm{result, type1, subNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
            } else if (count(primitive_type_conversions[type1].begin(),
                             primitive_type_conversions[type1].end(),
                             type2)) {

                string converted_string = convert(statementexpression2Eval->token, subNodeEval->token, type1, type2);
                string result = doArithmetic(statementexpression2Eval->token, converted_string, type1, "-");
                if (result != "") {
                    return new NodeTerm{result, type1, subNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
                return nullptr;
            } else {
                throw "Conversion Exception: Cannot convert expression to different type.";
                return nullptr;
            }
        }

        return nullptr;
    };

    explicit NodeStatementExpression(Node *statementexpression2, Node *statementexpressionp) {
        this->statementexpression2 = statementexpression2;
        this->statementexpressionp = statementexpressionp;
    }
};


// NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3
class NodeStatementExpression2_Production1 : public virtual Node {
public:
    Node *statementexpression3;

    Node *eval() override {
        cout << "called expr2 to expr3" << endl;
        return statementexpression3->eval();
    };

    explicit NodeStatementExpression2_Production1(Node *statementexpression3) {
        this->statementexpression3 = statementexpression3;
    }
};

// NTS_STATEMENT_EXPRESSION_2P -> TS_MULTIPLY NTS_STATEMENT_EXPRESSION_2
class NodeStatementExpression2p : public virtual Node {
public:
    Node *statementexpression2;

    Node *eval() override {
        return statementexpression2->eval();
    };

    explicit NodeStatementExpression2p(Node *statementexpression2) {
        this->statementexpression2 = statementexpression2;
    }
};

// NTS_STATEMENT_EXPRESSION_2P -> TS_DIVIDE NTS_STATEMENT_EXPRESSION_2
class NodeStatementExpression2p_Production1 : public virtual Node {
public:
    Node *statementexpression2;

    Node *eval() override {
        return statementexpression2->eval();
    };

    explicit NodeStatementExpression2p_Production1(Node *statementexpression2) {
        this->statementexpression2 = statementexpression2;
    }
};

// NTS_STATEMENT_EXPRESSION_2P -> TS_MODULO NTS_STATEMENT_EXPRESSION_2
class NodeStatementExpression2p_Production2 : public virtual Node {
public:
    Node *statementexpression2;

    Node *eval() override {
        return statementexpression2->eval();
    };

    explicit NodeStatementExpression2p_Production2(Node *statementexpression2) {
        this->statementexpression2 = statementexpression2;
    }
};

// NTS_STATEMENT_EXPRESSION_2 -> NTS_STATEMENT_EXPRESSION_3 NTS_STATEMENT_EXPRESSION_2P
class NodeStatementExpression2 : public virtual Node {
public:
    Node *statementexpression3;
    Node *statementexpression2p;

    Node *eval() override {
        // This is an mult operator
        if (auto *multNode = dynamic_cast<NodeStatementExpression2p *>(statementexpression2p)) {
            auto *statementexpression3Eval = dynamic_cast<NodeTerm *>(statementexpression3->eval());
            auto *multNodeEval = dynamic_cast<NodeTerm *>(multNode->eval());

            PrimitiveType type1 = statementexpression3Eval->inferred_type;
            PrimitiveType type2 = multNodeEval->inferred_type;
            if (type1 == type2) {
                string result = doArithmetic(statementexpression3Eval->token, multNodeEval->token, type1, "*");
                if (result != "") {
                    return new NodeTerm{result, type1, multNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
            } else if (count(primitive_type_conversions[type1].begin(),
                             primitive_type_conversions[type1].end(),
                             type2)) {

                string converted_string = convert(statementexpression3Eval->token, multNodeEval->token, type1, type2);
                string result = doArithmetic(statementexpression3Eval->token, converted_string, type1, "*");
                if (result != "") {
                    return new NodeTerm{result, type1, multNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
                return nullptr;
            } else {
                throw "Conversion Exception: Cannot convert expression to different type.";
                return nullptr;
            }
        }
            // This is an div operator

        else if (auto *divNode = dynamic_cast<NodeStatementExpression2p_Production1 *>(statementexpression2p)) {
            auto *statementexpression3Eval = dynamic_cast<NodeTerm *>(statementexpression3->eval());
            auto *divNodeEval = dynamic_cast<NodeTerm *>(divNode->eval());

            PrimitiveType type1 = statementexpression3Eval->inferred_type;
            PrimitiveType type2 = divNodeEval->inferred_type;
            if (type1 == type2) {
                string result = doArithmetic(statementexpression3Eval->token, divNodeEval->token, type1, "/");
                if (result != "") {
                    return new NodeTerm{result, type1, divNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
            } else if (count(primitive_type_conversions[type1].begin(),
                             primitive_type_conversions[type1].end(),
                             type2)) {

                string converted_string = convert(statementexpression3Eval->token, divNodeEval->token, type1, type2);
                string result = doArithmetic(statementexpression3Eval->token, converted_string, type1, "/");
                if (result != "") {
                    return new NodeTerm{result, type1, divNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
                return nullptr;
            } else {
                throw "Conversion Exception: Cannot convert expression to different type.";
                return nullptr;
            }
        }

            // This is a mod operation
        else if (auto *modNode = dynamic_cast<NodeStatementExpression2p_Production2 *>(statementexpression2p)) {
            auto *statementexpression3Eval = dynamic_cast<NodeTerm *>(statementexpression3->eval());
            auto *modNodeEval = dynamic_cast<NodeTerm *>(modNode->eval());

            PrimitiveType type1 = statementexpression3Eval->inferred_type;
            PrimitiveType type2 = modNodeEval->inferred_type;
            if (type1 == type2) {
                string result = doArithmetic(statementexpression3Eval->token, modNodeEval->token, type1, "%");
                if (result != "") {
                    return new NodeTerm{result, type1, modNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
            } else if (count(primitive_type_conversions[type1].begin(),
                             primitive_type_conversions[type1].end(),
                             type2)) {

                string converted_string = convert(statementexpression3Eval->token, modNodeEval->token, type1, type2);
                string result = doArithmetic(statementexpression3Eval->token, converted_string, type1, "%");
                if (result != "") {
                    return new NodeTerm{result, type1, modNodeEval->token_type};
                } else {
                    throw "Conversion Exception: Operation not defined.";
                    return nullptr;
                }
                return nullptr;
            } else {
                throw "Conversion Exception: Cannot convert expression to different type.";
                return nullptr;
            }
        }
        return nullptr;
    };

    explicit NodeStatementExpression2(Node *statementexpression3, Node *statementexpression2p) {
        this->statementexpression3 = statementexpression3;
        this->statementexpression2p = statementexpression2p;
    }
};


// NTS_STATEMENT_EXPRESSION_3 -> TS_TERM
class NodeStatementExpression3 : public virtual Node {
public:
    Node *term;

    Node *eval() override {
        return term->eval();
    };

    explicit NodeStatementExpression3(Node *term) {
        this->term = term;
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_IDENTIFIER
class NodeStatementExpression3_Production1 : public virtual Node {
public:
    Node *identifier;

    Node *eval() override {
        auto *nodeIdentifier = dynamic_cast<NodeIdentifier *>(identifier->eval());

        cout << "Grabbing Value in expression for: " << nodeIdentifier->token << endl;
        return SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifier->token];
    };

    explicit NodeStatementExpression3_Production1(Node *identifier) {
        this->identifier = identifier;
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_IDENTIFIER TS_LEFT_PARENTHESIS NTS_FUNCTION_PARAMS TS_RIGHT_PARENTHESIS
class NodeStatementExpression3_Production2 : public virtual Node {
public:
    Node *identifier;
    Node *functionparams;

    Node *eval() override {
        cout << "evaluating a function" << endl;
        auto *nodeIdentifier = dynamic_cast<NodeIdentifier *>(identifier->eval());
        cout << "nodeIdentifier (non-void): " << nodeIdentifier << endl;

        if (NodeFunctionParams_Production1 *nodeFunctionParams = dynamic_cast<NodeFunctionParams_Production1 *>(functionparams)) {
            if (SCOPED_SYMBOL_TABLE[SCOPE_LEVEL].find(nodeIdentifier->token) !=
                SCOPED_SYMBOL_TABLE[SCOPE_LEVEL].end()) {
                auto *nodeFunction = dynamic_cast<NodeStatementFunction *>(SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifier->token]);

                if (auto *nodeFunctionDefine = dynamic_cast<NodeFunctionParamsDefine * >(nodeFunction->functionparamsdefine)) {
                    cout << "Begin gathering parameters" << endl;

                    vector<Node *> givenParameters;
                    givenParameters = nodeFunctionParams->eval(givenParameters);
                    cout << "givenParameters.size: " << givenParameters.size() << endl;

                    vector<Node *> expectedParameters;
                    expectedParameters = nodeFunctionDefine->eval(expectedParameters);
                    cout << "expectedParameters.size: " << expectedParameters.size() << endl;

                    cout << "Done gathering parameters" << endl;

                    if (givenParameters.size() == expectedParameters.size()) {
                        cout << "parameters match function" << endl;
                        for (int i = 0; i < expectedParameters.size(); i++) {
                            cout << "in loop" << endl;
                            auto *nodeTerm = dynamic_cast<NodeTerm *>(givenParameters.at(i));
                            cout << "nodeTerm: " << nodeTerm << endl;
                            auto *expectedTerm = dynamic_cast<NodeIdentifier *>(expectedParameters.at(i));
                            cout << "expectedTerm: " << expectedTerm << endl;
                            SCOPED_SYMBOL_TABLE[SCOPE_LEVEL + 1][expectedTerm->token] = nodeTerm;
                        }
                        return nodeFunction->statementsuitefunction->eval();
                    } else {
                        throw "Number of parameters doesn't match function definition for " + nodeIdentifier->token;
                    }
                } else if (auto *nodeFunctionDefine = dynamic_cast<NodeFunctionParamsDefine_Production1 * >(nodeFunction->functionparamsdefine)) {
                    cout << "Begin gathering parameters" << endl;

                    vector<Node *> givenParameters;
                    givenParameters = nodeFunctionParams->eval(givenParameters);
                    cout << "givenParameters.size: " << givenParameters.size() << endl;

                    vector<Node *> expectedParameters;
                    expectedParameters = nodeFunctionDefine->eval(expectedParameters);
                    cout << "expectedParameters.size: " << expectedParameters.size() << endl;

                    cout << "Done gathering parameters" << endl;

                    if (givenParameters.size() == expectedParameters.size()) {
                        cout << "parameters match function" << endl;
                        for (int i = 0; i < expectedParameters.size(); i++) {
                            cout << "in loop" << endl;
                            auto *nodeTerm = dynamic_cast<NodeTerm *>(givenParameters.at(i));
                            cout << "nodeTerm: " << nodeTerm << endl;
                            auto *expectedTerm = dynamic_cast<NodeIdentifier *>(expectedParameters.at(i));
                            cout << "expectedTerm: " << expectedTerm << endl;
                            SCOPED_SYMBOL_TABLE[SCOPE_LEVEL + 1][expectedTerm->token] = nodeTerm;
                        }
                        return nodeFunction->statementsuitefunction->eval();
                    } else {
                        throw "Number of parameters doesn't match function definition for " + nodeIdentifier->token;
                    }
                } else {
                    throw "AST error, inner function parameters";
                }
            } else {
                throw "Function not defined: " + nodeIdentifier->token;
            }
        } else if (NodeFunctionParams *nodeFunctionParams = dynamic_cast<NodeFunctionParams *>(functionparams)) {
            if (SCOPED_SYMBOL_TABLE[SCOPE_LEVEL].find(nodeIdentifier->token) !=
                SCOPED_SYMBOL_TABLE[SCOPE_LEVEL].end()) {
                auto *nodeFunction = dynamic_cast<NodeStatementFunction *>(SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifier->token]);

                if (auto *nodeFunctionDefine = dynamic_cast<NodeFunctionParamsDefine * >(nodeFunction->functionparamsdefine)) {
                    cout << "Begin gathering parameters (recursive)" << endl;

                    vector<Node *> givenParameters;
                    givenParameters = nodeFunctionParams->eval(givenParameters);
                    cout << "givenParameters.size: " << givenParameters.size() << endl;

                    vector<Node *> expectedParameters;
                    expectedParameters = nodeFunctionDefine->eval(expectedParameters);
                    cout << "expectedParameters.size: " << expectedParameters.size() << endl;

                    cout << "Done gathering parameters" << endl;

                    if (givenParameters.size() == expectedParameters.size()) {
                        cout << "parameters match function" << endl;
                        for (int i = 0; i < expectedParameters.size(); i++) {
                            cout << "in loop" << endl;
                            auto *nodeTerm = dynamic_cast<NodeTerm *>(givenParameters.at(i));
                            cout << "nodeTerm: " << nodeTerm << endl;
                            auto *expectedTerm = dynamic_cast<NodeIdentifier *>(expectedParameters.at(i));
                            cout << "expectedTerm: " << expectedTerm << endl;
                            SCOPED_SYMBOL_TABLE[SCOPE_LEVEL + 1][expectedTerm->token] = nodeTerm;
                        }
                        return nodeFunction->statementsuitefunction->eval();
                    } else {
                        throw "Number of parameters doesn't match function definition for " + nodeIdentifier->token;
                    }
                } else if (auto *nodeFunctionDefine = dynamic_cast<NodeFunctionParamsDefine_Production1 * >(nodeFunction->functionparamsdefine)) {
                    cout << "Begin gathering parameters" << endl;

                    vector<Node *> givenParameters;
                    givenParameters = nodeFunctionParams->eval(givenParameters);
                    cout << "givenParameters.size: " << givenParameters.size() << endl;

                    vector<Node *> expectedParameters;
                    expectedParameters = nodeFunctionDefine->eval(expectedParameters);
                    cout << "expectedParameters.size: " << expectedParameters.size() << endl;

                    cout << "Done gathering parameters" << endl;

                    if (givenParameters.size() == expectedParameters.size()) {
                        cout << "parameters match function" << endl;
                        for (int i = 0; i < expectedParameters.size(); i++) {
                            cout << "in loop" << endl;
                            auto *nodeTerm = dynamic_cast<NodeTerm *>(givenParameters.at(i));
                            cout << "nodeTerm: " << nodeTerm << endl;
                            auto *expectedTerm = dynamic_cast<NodeIdentifier *>(expectedParameters.at(i));
                            cout << "expectedTerm: " << expectedTerm << endl;
                            SCOPED_SYMBOL_TABLE[SCOPE_LEVEL + 1][expectedTerm->token] = nodeTerm;
                        }
                        return nodeFunction->statementsuitefunction->eval();
                    } else {
                        throw "Number of parameters doesn't match function definition for " + nodeIdentifier->token;
                    }
                } else {
                    throw "AST error, inner function parameters";
                }
            } else {
                throw "Function not defined: " + nodeIdentifier->token;
            }
        } else {
            throw "Parameters cannot be cast AST error";
        }

        /*
         * If the item of key identifier exists within the symbol table
         *      If the number of function params is equal to the number of function params provided...
         */

        return nullptr;
    };

    explicit NodeStatementExpression3_Production2(Node *identifier, Node *functionparams) {
        this->identifier = identifier;
        this->functionparams = functionparams;
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_IDENTIFIER TS_LEFT_PARENTHESIS TS_RIGHT_PARENTHESIS
struct NodeStatementExpression3_Production3 : public virtual Node {
public:
    Node *identifier;

    Node *eval() override {
        auto *nodeIdentifier = dynamic_cast<NodeIdentifier *>(identifier->eval());
        cout << "evaluating a function (void): " << nodeIdentifier->token << endl;

        if (SCOPED_SYMBOL_TABLE[SCOPE_LEVEL].find(nodeIdentifier->token) != SCOPED_SYMBOL_TABLE[SCOPE_LEVEL].end()) {

            auto *nodeStatementFunction_Production1 = dynamic_cast<NodeStatementFunction_Production1 *> (SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifier->token]);
            cout << "nodeStatementFunction: " << nodeStatementFunction_Production1 << endl;

            auto *nodeStatementSuiteFunction = dynamic_cast<NodeStatementSuiteFunction *> (nodeStatementFunction_Production1->statementsuitefunction);
            cout << "nodeStatementSuiteFunction: " << nodeStatementSuiteFunction << endl;

            cout << "parameters match function (void): " << nodeIdentifier->token << endl;

            SCOPE_LEVEL++;
            Node *result = nodeStatementSuiteFunction->eval();
            SCOPE_LEVEL--;

            return result;
        } else {
            throw "Function not defined: " + nodeIdentifier->token;
        }

        /*
         * If the item of key identifier exists within the symbol table
         *      If the number of function params is equal to the number of function params provided...
         */

        return nullptr;
    };

    explicit NodeStatementExpression3_Production3(Node *identifier) {
        this->identifier = identifier;
    }
};


// NTS_STATEMENT_EXPRESSION_3 -> TS_LEFT_PARENTHESIS NTS_STATEMENT_EXPRESSION TS_RIGHT_PARENTHESIS
class NodeStatementExpression3_Production4 : public virtual Node {
public:
    Node *statementexpression;

    Node *eval() override {
        return statementexpression->eval();
    };

    explicit NodeStatementExpression3_Production4(Node *statementexpression) {
        this->statementexpression = statementexpression;
    }
};

// NTS_STATEMENT_EXPRESSION_3 -> TS_SUBTRACT NTS_STATEMENT_EXPRESSION_3
class NodeStatementExpression3_Production5 : public virtual Node {
public:
    Node *statementexpression3;

    Node *eval() override {
        auto *nodeTerm = dynamic_cast<NodeTerm *>(statementexpression3->eval());
        nodeTerm->token = "-" + nodeTerm->token;
        return nodeTerm;
    };

    explicit NodeStatementExpression3_Production5(Node *statementexpression3) {
        this->statementexpression3 = statementexpression3;
    }
};

// NTS_STATEMENT_ASSIGNMENT -> TS_IDENTIFIER TS_EQUALS NTS_STATEMENT_EXPRESSION
class NodeStatementAssignment : public virtual Node {
public:
    Node *identifier;
    Node *statementexpression;

    Node *eval() override {
        statementexpression->debug();
        // Need to add error handling
        NodeIdentifier *nodeIdentifierEval = dynamic_cast<NodeIdentifier *>(identifier->eval());
        NodeTerm *statementexpressionEval = dynamic_cast<NodeTerm *>(statementexpression->eval());

        cout << "Called Assignment for:  " << nodeIdentifierEval->token << endl;

        SCOPED_SYMBOL_TABLE[SCOPE_LEVEL][nodeIdentifierEval->token] = statementexpressionEval;
        return nullptr;
    };

    explicit NodeStatementAssignment(Node *identifier, Node *statementexpression) {
        this->identifier = identifier;
        this->statementexpression = statementexpression;
    }
};

// NTS_COMPARISON_OPERATOR -> TS_LESS_THAN
class NodeComparisonOperator : public virtual Node {
public:
    Node *lessthan;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeComparisonOperator(Node *lessthan) {
        this->lessthan = lessthan;
    }
};

// NTS_COMPARISON_OPERATOR -> TS_LESS_THAN_EQUALS
class NodeComparisonOperator_Production1 : public virtual Node {
public:
    Node *lessthanequals;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeComparisonOperator_Production1(Node *lessthanequals) {
        this->lessthanequals = lessthanequals;
    }
};

// NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN
class NodeComparisonOperator_Production2 : public virtual Node {
public:
    Node *greaterthan;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeComparisonOperator_Production2(Node *greaterthan) {
        this->greaterthan = greaterthan;
    }
};

// NTS_COMPARISON_OPERATOR -> TS_GREATER_THAN_EQUALS
class NodeComparisonOperator_Production3 : public virtual Node {
public:
    Node *greaterthanequals;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeComparisonOperator_Production3(Node *greaterthanequals) {
        this->greaterthanequals = greaterthanequals;
    }
};

// NTS_COMPARISON_OPERATOR -> TS_DOUBLE_EQUALS
class NodeComparisonOperator_Production4 : public virtual Node {
public:
    Node *doubleequals;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeComparisonOperator_Production4(Node *doubleequals) {
        this->doubleequals = doubleequals;
    }
};

// NTS_CONDITIONAL_EXPRESSION -> NTS_STATEMENT_EXPRESSION NTS_COMPARISON_OPERATOR NTS_STATEMENT_EXPRESSION
class NodeConditionalExpression : public virtual Node {
public:
    Node *statementexpression;
    Node *comparisonoperator;
    Node *statementexpression1;

    Node *eval() {
        return {};
    }

    string eval_() {

        NodeTerm *term1 = dynamic_cast<NodeTerm *>(statementexpression->eval());
        NodeTerm *term2 = dynamic_cast<NodeTerm *>(statementexpression1->eval());

        // Less Than
        if (auto *comparisonOperatorEval = dynamic_cast<NodeComparisonOperator *> (comparisonoperator)) {
            return doArithmetic(term1->token, term2->token, term1->inferred_type, "<");
        }
            // Less than equals
        else if (auto *comparisonOperatorEval = dynamic_cast<NodeComparisonOperator_Production1 *> (comparisonoperator)) {
            return doArithmetic(term1->token, term2->token, term1->inferred_type, "<=");
        }
            // Greater than
        else if (auto *comparisonOperatorEval = dynamic_cast<NodeComparisonOperator_Production2 *> (comparisonoperator)) {
            return doArithmetic(term1->token, term2->token, term1->inferred_type, ">");
        }
            // Greater than equals
        else if (auto *comparisonOperatorEval = dynamic_cast<NodeComparisonOperator_Production3 *> (comparisonoperator)) {
            return doArithmetic(term1->token, term2->token, term1->inferred_type, ">=");
        }
            // Double Equals
        else if (auto *comparisonOperatorEval = dynamic_cast<NodeComparisonOperator_Production4 *> (comparisonoperator)) {
            return doArithmetic(term1->token, term2->token, term1->inferred_type, "==");
        } else {
            throw "Undefined Operator Exception";
        }
        return "";
    };

    explicit NodeConditionalExpression(Node *statementexpression, Node *comparisonoperator,
                                       Node *statementexpression1) {
        this->statementexpression = statementexpression;
        this->comparisonoperator = comparisonoperator;
        this->statementexpression1 = statementexpression1;
    }
};

// NTS_CONDITIONAL_EXPRESSION -> NTS_COMPARISON_OPERATOR_UNARY NTS_STATEMENT_EXPRESSION
class NodeConditionalExpression_Production1 : public virtual Node {
public:
    Node *comparisonoperatorunary;
    Node *statementexpression;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeConditionalExpression_Production1(Node *comparisonoperatorunary, Node *statementexpression) {
        this->comparisonoperatorunary = comparisonoperatorunary;
        this->statementexpression = statementexpression;
    }
};

// NTS_COMPARISON_OPERATOR_UNARY -> TS_NEGATION
class NodeComparisonOperatorUnary : public virtual Node {
public:
    Node *negation;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeComparisonOperatorUnary(Node *negation) {
        this->negation = negation;
    }
};

// NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementConditional : public virtual Node {
public:
    Node *conditionalexpression;
    Node *statementsuitefunction;

    Node *eval() override {
        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            return statementsuitefunction->eval();
        }

        return nullptr;
    };

    explicit NodeStatementConditional(Node *conditionalexpression, Node *statementsuitefunction) {
        this->conditionalexpression = conditionalexpression;
        this->statementsuitefunction = statementsuitefunction;
    }
};

// NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_2
class NodeStatementConditional_Production1 : public virtual Node {
public:
    Node *conditionalexpression;
    Node *statementsuitefunction;
    Node *statementconditional2;

    Node *eval() override {
        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            return statementsuitefunction->eval();
        }

        statementconditional2->eval();
        return nullptr;
    };

    explicit NodeStatementConditional_Production1(Node *conditionalexpression, Node *statementsuitefunction,
                                                  Node *statementconditional2) {
        this->conditionalexpression = conditionalexpression;
        this->statementsuitefunction = statementsuitefunction;
        this->statementconditional2 = statementconditional2;
    }
};

// NTS_STATEMENT_CONDITIONAL -> TS_IF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
class NodeStatementConditional_Production2 : public virtual Node {
public:
    Node *conditionalexpression;
    Node *statementsuitefunction;
    Node *statementconditional3;

    Node *eval() override {
        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            return statementsuitefunction->eval();
        }

        statementconditional3->eval();
        return nullptr;
    };

    explicit NodeStatementConditional_Production2(Node *conditionalexpression, Node *statementsuitefunction,
                                                  Node *statementconditional3) {
        this->conditionalexpression = conditionalexpression;
        this->statementsuitefunction = statementsuitefunction;
        this->statementconditional3 = statementconditional3;
    }
};

// NTS_STATEMENT_CONDITIONAL_2 -> NTS_STATEMENT_CONDITIONAL_2 TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementConditional2 : public virtual Node {
public:
    Node *statementconditional2;
    Node *conditionalexpression;
    Node *statementsuitefunction;

    Node *eval() override {

        statementconditional2->eval();

        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            return statementsuitefunction->eval();
        }
        return nullptr;
    };

    explicit NodeStatementConditional2(Node *statementconditional2, Node *conditionalexpression,
                                       Node *statementsuitefunction) {
        this->statementconditional2 = statementconditional2;
        this->conditionalexpression = conditionalexpression;
        this->statementsuitefunction = statementsuitefunction;
    }
};

// NTS_STATEMENT_CONDITIONAL_2 -> TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE NTS_STATEMENT_CONDITIONAL_3
class NodeStatementConditional2_Production1 : public virtual Node {
public:
    Node *conditionalexpression;
    Node *statementsuitefunction;
    Node *statementconditional3;

    Node *eval() override {
        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            return statementsuitefunction->eval();
        }

        statementconditional3->eval();
        return nullptr;
    };

    explicit NodeStatementConditional2_Production1(Node *conditionalexpression, Node *statementsuitefunction,
                                                   Node *statementconditional3) {
        this->conditionalexpression = conditionalexpression;
        this->statementsuitefunction = statementsuitefunction;
        this->statementconditional3 = statementconditional3;
    }
};

// NTS_STATEMENT_CONDITIONAL_2 -> TS_ELIF NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
struct NodeStatementConditional2_Production2 : public virtual Node {
public:
    Node *statementsuitefunction;
    Node *conditionalexpression;

    Node *eval() override {
        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            return statementsuitefunction->eval();
        }

        return nullptr;
    };

    explicit NodeStatementConditional2_Production2(Node *statementsuitefunction, Node *conditionalexpression) {
        this->statementsuitefunction = statementsuitefunction;
        this->conditionalexpression = conditionalexpression;
    }
};

// NTS_STATEMENT_CONDITIONAL_3 -> TS_ELSE TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementConditional3 : public virtual Node {
public:
    Node *statementsuitefunction;

    Node *eval() override {
        return statementsuitefunction->eval();
    };

    explicit NodeStatementConditional3(Node *statementsuitefunction) {
        this->statementsuitefunction = statementsuitefunction;
    }
};


// NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_FOR
class NodeStatementLoop : public virtual Node {
public:
    Node *statementloopfor;

    Node *eval() override {
        return statementloopfor->eval();
    };

    explicit NodeStatementLoop(Node *statementloopfor) {
        this->statementloopfor = statementloopfor;
    }
};

// NTS_STATEMENT_LOOP -> NTS_STATEMENT_LOOP_WHILE
class NodeStatementLoop_Production1 : public virtual Node {
public:
    Node *statementloopwhile;

    Node *eval() override {
        return statementloopwhile->eval();
    };

    explicit NodeStatementLoop_Production1(Node *statementloopwhile) {
        this->statementloopwhile = statementloopwhile;
    }
};

// NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON NTS_STATEMENT_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementLoopFor : public virtual Node {
public:
    Node *identifier;
    Node *statementexpression;
    Node *statementsuitefunction;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeStatementLoopFor(Node *identifier, Node *statementexpression, Node *statementsuitefunction) {
        this->identifier = identifier;
        this->statementexpression = statementexpression;
        this->statementsuitefunction = statementsuitefunction;
    }
};

// NTS_STATEMENT_LOOP_FOR -> TS_FOR TS_IDENTIFIER TS_COLON NTS_STATEMENT_EXPRESSION TS_ELLIPSIS NTS_STATEMENT_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementLoopFor_Production1 : public virtual Node {
public:
    Node *identifier;
    Node *statementexpression;
    Node *statementexpression1;
    Node *statementsuitefunction;

    Node *eval() override {
        return nullptr;
    };

    explicit NodeStatementLoopFor_Production1(Node *identifier, Node *statementexpression, Node *statementexpression1,
                                              Node *statementsuitefunction) {
        this->identifier = identifier;
        this->statementexpression = statementexpression;
        this->statementexpression1 = statementexpression1;
        this->statementsuitefunction = statementsuitefunction;
    }
};

// NTS_STATEMENT_LOOP_WHILE -> TS_WHILE NTS_CONDITIONAL_EXPRESSION TS_LEFT_CURLY_BRACE NTS_STATEMENT_SUITE_FUNCTION TS_RIGHT_CURLY_BRACE
class NodeStatementLoopWhile : public virtual Node {
public:
    Node *conditionalexpression;
    Node *statementsuitefunction;

    Node *eval() override {

        loop_begin:

        NodeConditionalExpression *nodeConditionalExpression = dynamic_cast<NodeConditionalExpression *>(conditionalexpression);
        string output = nodeConditionalExpression->eval_();

        if (output == "1") {
            statementsuitefunction->eval();
            goto loop_begin;
        }

        return nullptr;
    };

    explicit NodeStatementLoopWhile(Node *conditionalexpression, Node *statementsuitefunction) {
        this->conditionalexpression = conditionalexpression;
        this->statementsuitefunction = statementsuitefunction;
    }
};


#endif //MANGO_V2_CPP_PARSER_TREE_H
