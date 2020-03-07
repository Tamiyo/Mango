#ifndef MANGOREVISITEDCPPCLION_PARSE_TABLE_H
#define MANGOREVISITEDCPPCLION_PARSE_TABLE_H

#include "map"
#include "iostream"
#include "sstream"
#include "algorithm"
#include "../core/grammar.h"
#include "../tree/tree.h"

using std::map;
using grammar::token;
using std::pair;

template<typename T>
class stack {
public:
    explicit stack() {
        stk = {};
    }

    std::vector<T> stk;

    void push(T v) {
        stk.emplace_back(v);
    }

    T pop() {
        T x = stk.back();
        stk.pop_back();
        return x;
    }

    T peek() {
        return stk.back();
    }

    bool empty() {
        return stk.empty();
    }

    int size() {
        return stk.size();
    }

    void reverse() {
        std::reverse(stk.begin(), stk.end());
    }

    string print_state() {
        auto ss = std::ostringstream();
        for(const auto& elem : stk) {
            ss << elem << " ";
        }
        return ss.str();
    }
};
static map<pair<int, token>, int> taction = {
        { {0, token::kw_print}, 8},
        { {0, token::open_paren}, 9},
        { {0, token::identifier}, 10},
        { {0, token::literal}, 11},
        { {1, token::eof}, 0},
        { {2, token::eof}, -2},
        { {3, token::newline}, 12},
        { {4, token::newline}, -5},
        { {5, token::newline}, -6},
        { {6, token::op_plus}, 13},
        { {6, token::op_minus}, 14},
        { {6, token::close_paren}, -10},
        { {6, token::newline}, -10},
        { {7, token::op_plus}, -16},
        { {7, token::op_minus}, -16},
        { {7, token::op_mult}, 15},
        { {7, token::op_div}, 16},
        { {7, token::op_idiv}, 17},
        { {7, token::op_mod}, 18},
        { {7, token::op_pow}, 19},
        { {7, token::close_paren}, -16},
        { {7, token::newline}, -16},
        { {8, token::open_paren}, 20},
        { {9, token::open_paren}, 24},
        { {9, token::identifier}, 25},
        { {9, token::literal}, 26},
        { {10, token::op_plus}, -18},
        { {10, token::op_minus}, -18},
        { {10, token::op_mult}, -18},
        { {10, token::op_div}, -18},
        { {10, token::op_idiv}, -18},
        { {10, token::op_mod}, -18},
        { {10, token::op_pow}, -18},
        { {10, token::close_paren}, -18},
        { {10, token::newline}, -18},
        { {11, token::op_plus}, -19},
        { {11, token::op_minus}, -19},
        { {11, token::op_mult}, -19},
        { {11, token::op_div}, -19},
        { {11, token::op_idiv}, -19},
        { {11, token::op_mod}, -19},
        { {11, token::op_pow}, -19},
        { {11, token::close_paren}, -19},
        { {11, token::newline}, -19},
        { {12, token::kw_print}, 8},
        { {12, token::open_paren}, 9},
        { {12, token::identifier}, 10},
        { {12, token::literal}, 11},
        { {12, token::eof}, -4},
        { {13, token::open_paren}, 9},
        { {13, token::identifier}, 10},
        { {13, token::literal}, 11},
        { {14, token::open_paren}, 9},
        { {14, token::identifier}, 10},
        { {14, token::literal}, 11},
        { {15, token::open_paren}, 9},
        { {15, token::identifier}, 10},
        { {15, token::literal}, 11},
        { {16, token::open_paren}, 9},
        { {16, token::identifier}, 10},
        { {16, token::literal}, 11},
        { {17, token::open_paren}, 9},
        { {17, token::identifier}, 10},
        { {17, token::literal}, 11},
        { {18, token::open_paren}, 9},
        { {18, token::identifier}, 10},
        { {18, token::literal}, 11},
        { {19, token::open_paren}, 9},
        { {19, token::identifier}, 10},
        { {19, token::literal}, 11},
        { {20, token::open_paren}, 24},
        { {20, token::identifier}, 25},
        { {20, token::literal}, 26},
        { {21, token::close_paren}, 36},
        { {22, token::op_plus}, 37},
        { {22, token::op_minus}, 38},
        { {22, token::close_paren}, -10},
        { {22, token::newline}, -10},
        { {23, token::op_plus}, -16},
        { {23, token::op_minus}, -16},
        { {23, token::op_mult}, 39},
        { {23, token::op_div}, 40},
        { {23, token::op_idiv}, 41},
        { {23, token::op_mod}, 42},
        { {23, token::op_pow}, 43},
        { {23, token::close_paren}, -16},
        { {23, token::newline}, -16},
        { {24, token::open_paren}, 24},
        { {24, token::identifier}, 25},
        { {24, token::literal}, 26},
        { {25, token::op_plus}, -18},
        { {25, token::op_minus}, -18},
        { {25, token::op_mult}, -18},
        { {25, token::op_div}, -18},
        { {25, token::op_idiv}, -18},
        { {25, token::op_mod}, -18},
        { {25, token::op_pow}, -18},
        { {25, token::close_paren}, -18},
        { {25, token::newline}, -18},
        { {26, token::op_plus}, -19},
        { {26, token::op_minus}, -19},
        { {26, token::op_mult}, -19},
        { {26, token::op_div}, -19},
        { {26, token::op_idiv}, -19},
        { {26, token::op_mod}, -19},
        { {26, token::op_pow}, -19},
        { {26, token::close_paren}, -19},
        { {26, token::newline}, -19},
        { {27, token::eof}, -3},
        { {28, token::close_paren}, -8},
        { {28, token::newline}, -8},
        { {29, token::close_paren}, -9},
        { {29, token::newline}, -9},
        { {30, token::op_plus}, -11},
        { {30, token::op_minus}, -11},
        { {30, token::close_paren}, -11},
        { {30, token::newline}, -11},
        { {31, token::op_plus}, -12},
        { {31, token::op_minus}, -12},
        { {31, token::close_paren}, -12},
        { {31, token::newline}, -12},
        { {32, token::op_plus}, -13},
        { {32, token::op_minus}, -13},
        { {32, token::close_paren}, -13},
        { {32, token::newline}, -13},
        { {33, token::op_plus}, -14},
        { {33, token::op_minus}, -14},
        { {33, token::close_paren}, -14},
        { {33, token::newline}, -14},
        { {34, token::op_plus}, -15},
        { {34, token::op_minus}, -15},
        { {34, token::close_paren}, -15},
        { {34, token::newline}, -15},
        { {35, token::close_paren}, 45},
        { {36, token::op_plus}, -17},
        { {36, token::op_minus}, -17},
        { {36, token::op_mult}, -17},
        { {36, token::op_div}, -17},
        { {36, token::op_idiv}, -17},
        { {36, token::op_mod}, -17},
        { {36, token::op_pow}, -17},
        { {36, token::close_paren}, -17},
        { {36, token::newline}, -17},
        { {37, token::open_paren}, 24},
        { {37, token::identifier}, 25},
        { {37, token::literal}, 26},
        { {38, token::open_paren}, 24},
        { {38, token::identifier}, 25},
        { {38, token::literal}, 26},
        { {39, token::open_paren}, 24},
        { {39, token::identifier}, 25},
        { {39, token::literal}, 26},
        { {40, token::open_paren}, 24},
        { {40, token::identifier}, 25},
        { {40, token::literal}, 26},
        { {41, token::open_paren}, 24},
        { {41, token::identifier}, 25},
        { {41, token::literal}, 26},
        { {42, token::open_paren}, 24},
        { {42, token::identifier}, 25},
        { {42, token::literal}, 26},
        { {43, token::open_paren}, 24},
        { {43, token::identifier}, 25},
        { {43, token::literal}, 26},
        { {44, token::close_paren}, 53},
        { {45, token::newline}, -7},
        { {46, token::close_paren}, -8},
        { {46, token::newline}, -8},
        { {47, token::close_paren}, -9},
        { {47, token::newline}, -9},
        { {48, token::op_plus}, -11},
        { {48, token::op_minus}, -11},
        { {48, token::close_paren}, -11},
        { {48, token::newline}, -11},
        { {49, token::op_plus}, -12},
        { {49, token::op_minus}, -12},
        { {49, token::close_paren}, -12},
        { {49, token::newline}, -12},
        { {50, token::op_plus}, -13},
        { {50, token::op_minus}, -13},
        { {50, token::close_paren}, -13},
        { {50, token::newline}, -13},
        { {51, token::op_plus}, -14},
        { {51, token::op_minus}, -14},
        { {51, token::close_paren}, -14},
        { {51, token::newline}, -14},
        { {52, token::op_plus}, -15},
        { {52, token::op_minus}, -15},
        { {52, token::close_paren}, -15},
        { {52, token::newline}, -15},
        { {53, token::op_plus}, -17},
        { {53, token::op_minus}, -17},
        { {53, token::op_mult}, -17},
        { {53, token::op_div}, -17},
        { {53, token::op_idiv}, -17},
        { {53, token::op_mod}, -17},
        { {53, token::op_pow}, -17},
        { {53, token::close_paren}, -17},
        { {53, token::newline}, -17},
};
static map<pair<int, token>, int> tgoto = {
        { {0, token::StatementSuite}, 1},
        { {0, token::StatementList}, 2},
        { {0, token::Statement}, 3},
        { {0, token::Print}, 4},
        { {0, token::Expression}, 5},
        { {0, token::MultiplicativeExpression}, 6},
        { {0, token::BaseExpression}, 7},
        { {9, token::Expression}, 21},
        { {9, token::MultiplicativeExpression}, 22},
        { {9, token::BaseExpression}, 23},
        { {12, token::StatementList}, 27},
        { {12, token::Statement}, 3},
        { {12, token::Print}, 4},
        { {12, token::Expression}, 5},
        { {12, token::MultiplicativeExpression}, 6},
        { {12, token::BaseExpression}, 7},
        { {13, token::Expression}, 28},
        { {13, token::MultiplicativeExpression}, 6},
        { {13, token::BaseExpression}, 7},
        { {14, token::Expression}, 29},
        { {14, token::MultiplicativeExpression}, 6},
        { {14, token::BaseExpression}, 7},
        { {15, token::MultiplicativeExpression}, 30},
        { {15, token::BaseExpression}, 7},
        { {16, token::MultiplicativeExpression}, 31},
        { {16, token::BaseExpression}, 7},
        { {17, token::MultiplicativeExpression}, 32},
        { {17, token::BaseExpression}, 7},
        { {18, token::MultiplicativeExpression}, 33},
        { {18, token::BaseExpression}, 7},
        { {19, token::MultiplicativeExpression}, 34},
        { {19, token::BaseExpression}, 7},
        { {20, token::Expression}, 35},
        { {20, token::MultiplicativeExpression}, 22},
        { {20, token::BaseExpression}, 23},
        { {24, token::Expression}, 44},
        { {24, token::MultiplicativeExpression}, 22},
        { {24, token::BaseExpression}, 23},
        { {37, token::Expression}, 46},
        { {37, token::MultiplicativeExpression}, 22},
        { {37, token::BaseExpression}, 23},
        { {38, token::Expression}, 47},
        { {38, token::MultiplicativeExpression}, 22},
        { {38, token::BaseExpression}, 23},
        { {39, token::MultiplicativeExpression}, 48},
        { {39, token::BaseExpression}, 23},
        { {40, token::MultiplicativeExpression}, 49},
        { {40, token::BaseExpression}, 23},
        { {41, token::MultiplicativeExpression}, 50},
        { {41, token::BaseExpression}, 23},
        { {42, token::MultiplicativeExpression}, 51},
        { {42, token::BaseExpression}, 23},
        { {43, token::MultiplicativeExpression}, 52},
        { {43, token::BaseExpression}, 23},
};

static void reduce(int decision, stack<Node*> *value_stack) {
    switch(decision) {
        case 1: {
            auto* StatementSuite = value_stack->pop();
            auto* node = new Mango1 {StatementSuite};
            value_stack->push(node);
            break;
        }
        case 2: {
            auto* StatementList = value_stack->pop();
            auto* node = new StatementSuite1 {StatementList};
            value_stack->push(node);
            break;
        }
        case 3: {
            auto* Statement = value_stack->pop();
            auto* StatementList = value_stack->pop();
            auto* node = new StatementList1 {Statement, StatementList};
            value_stack->push(node);
            break;
        }
        case 4: {
            auto* Statement = value_stack->pop();
            auto* node = new StatementList2 {Statement};
            value_stack->push(node);
            break;
        }
        case 5: {
            auto* Print = value_stack->pop();
            auto* node = new Statement1 {Print};
            value_stack->push(node);
            break;
        }
        case 6: {
            auto* Expression = value_stack->pop();
            auto* node = new Statement2 {Expression};
            value_stack->push(node);
            break;
        }
        case 7: {
            auto* Expression = value_stack->pop();
            auto* node = new Print1 {Expression};
            value_stack->push(node);
            break;
        }
        case 8: {
            auto* MultiplicativeExpression = value_stack->pop();
            auto* Expression = value_stack->pop();
            auto* node = new Expression1 {MultiplicativeExpression, Expression};
            value_stack->push(node);
            break;
        }
        case 9: {
            auto* MultiplicativeExpression = value_stack->pop();
            auto* Expression = value_stack->pop();
            auto* node = new Expression2 {MultiplicativeExpression, Expression};
            value_stack->push(node);
            break;
        }
        case 10: {
            auto* MultiplicativeExpression = value_stack->pop();
            auto* node = new Expression3 {MultiplicativeExpression};
            value_stack->push(node);
            break;
        }
        case 11: {
            auto* BaseExpression = value_stack->pop();
            auto* MultiplicativeExpression = value_stack->pop();
            auto* node = new MultiplicativeExpression1 {BaseExpression, MultiplicativeExpression};
            value_stack->push(node);
            break;
        }
        case 12: {
            auto* BaseExpression = value_stack->pop();
            auto* MultiplicativeExpression = value_stack->pop();
            auto* node = new MultiplicativeExpression2 {BaseExpression, MultiplicativeExpression};
            value_stack->push(node);
            break;
        }
        case 13: {
            auto* BaseExpression = value_stack->pop();
            auto* MultiplicativeExpression = value_stack->pop();
            auto* node = new MultiplicativeExpression3 {BaseExpression, MultiplicativeExpression};
            value_stack->push(node);
            break;
        }
        case 14: {
            auto* BaseExpression = value_stack->pop();
            auto* MultiplicativeExpression = value_stack->pop();
            auto* node = new MultiplicativeExpression4 {BaseExpression, MultiplicativeExpression};
            value_stack->push(node);
            break;
        }
        case 15: {
            auto* BaseExpression = value_stack->pop();
            auto* MultiplicativeExpression = value_stack->pop();
            auto* node = new MultiplicativeExpression5 {BaseExpression, MultiplicativeExpression};
            value_stack->push(node);
            break;
        }
        case 16: {
            auto* BaseExpression = value_stack->pop();
            auto* node = new MultiplicativeExpression6 {BaseExpression};
            value_stack->push(node);
            break;
        }
        case 17: {
            auto* Expression = value_stack->pop();
            auto* node = new BaseExpression1 {Expression};
            value_stack->push(node);
            break;
        }
        case 18: {
            auto* identifier = value_stack->pop();
            auto* node = new BaseExpression2 {identifier};
            value_stack->push(node);
            break;
        }
        case 19: {
            auto* literal = value_stack->pop();
            auto* node = new BaseExpression3 {literal};
            value_stack->push(node);
            break;
        }
        default:
            exit(1);
    }
}

#endif