#ifndef MANGOREVISITEDCPPCLION_PARSE_TABLE_H
#define MANGOREVISITEDCPPCLION_PARSE_TABLE_H

#include "map"
#include "stack"
#include "../core/grammar.h"
#include "../tree/tree.h"

using std::map;
using std::pair;
using std::stack;

using namespace grammar;

static map<pair<int, token>, int> taction = {
	{ {0, token::kw_print}, 9},
	{ {0, token::open_paren}, 10},
	{ {0, token::identifier}, 11},
	{ {0, token::type_string}, 12},
	{ {0, token::type_double}, 13},
	{ {0, token::type_int}, 14},
	{ {1, token::eof}, 0},
	{ {2, token::eof}, -2},
	{ {3, token::newline}, 15},
	{ {4, token::newline}, -5},
	{ {5, token::newline}, -6},
	{ {6, token::op_plus}, 16},
	{ {6, token::op_minus}, 17},
	{ {6, token::close_paren}, -10},
	{ {6, token::newline}, -10},
	{ {7, token::op_plus}, -16},
	{ {7, token::op_minus}, -16},
	{ {7, token::op_mult}, 18},
	{ {7, token::op_div}, 19},
	{ {7, token::op_idiv}, 20},
	{ {7, token::op_mod}, 21},
	{ {7, token::op_pow}, 22},
	{ {7, token::close_paren}, -16},
	{ {7, token::newline}, -16},
	{ {8, token::op_plus}, -19},
	{ {8, token::op_minus}, -19},
	{ {8, token::op_mult}, -19},
	{ {8, token::op_div}, -19},
	{ {8, token::op_idiv}, -19},
	{ {8, token::op_mod}, -19},
	{ {8, token::op_pow}, -19},
	{ {8, token::close_paren}, -19},
	{ {8, token::newline}, -19},
	{ {9, token::open_paren}, 23},
	{ {10, token::open_paren}, 28},
	{ {10, token::identifier}, 29},
	{ {10, token::type_string}, 30},
	{ {10, token::type_double}, 31},
	{ {10, token::type_int}, 32},
	{ {11, token::op_plus}, -18},
	{ {11, token::op_minus}, -18},
	{ {11, token::op_mult}, -18},
	{ {11, token::op_div}, -18},
	{ {11, token::op_idiv}, -18},
	{ {11, token::op_mod}, -18},
	{ {11, token::op_pow}, -18},
	{ {11, token::close_paren}, -18},
	{ {11, token::newline}, -18},
	{ {12, token::op_plus}, -22},
	{ {12, token::op_minus}, -22},
	{ {12, token::op_mult}, -22},
	{ {12, token::op_div}, -22},
	{ {12, token::op_idiv}, -22},
	{ {12, token::op_mod}, -22},
	{ {12, token::op_pow}, -22},
	{ {12, token::close_paren}, -22},
	{ {12, token::newline}, -22},
	{ {13, token::op_plus}, -20},
	{ {13, token::op_minus}, -20},
	{ {13, token::op_mult}, -20},
	{ {13, token::op_div}, -20},
	{ {13, token::op_idiv}, -20},
	{ {13, token::op_mod}, -20},
	{ {13, token::op_pow}, -20},
	{ {13, token::close_paren}, -20},
	{ {13, token::newline}, -20},
	{ {14, token::op_plus}, -21},
	{ {14, token::op_minus}, -21},
	{ {14, token::op_mult}, -21},
	{ {14, token::op_div}, -21},
	{ {14, token::op_idiv}, -21},
	{ {14, token::op_mod}, -21},
	{ {14, token::op_pow}, -21},
	{ {14, token::close_paren}, -21},
	{ {14, token::newline}, -21},
	{ {15, token::kw_print}, 9},
	{ {15, token::open_paren}, 10},
	{ {15, token::identifier}, 11},
	{ {15, token::type_string}, 12},
	{ {15, token::type_double}, 13},
	{ {15, token::type_int}, 14},
	{ {15, token::eof}, -4},
	{ {16, token::open_paren}, 10},
	{ {16, token::identifier}, 11},
	{ {16, token::type_string}, 12},
	{ {16, token::type_double}, 13},
	{ {16, token::type_int}, 14},
	{ {17, token::open_paren}, 10},
	{ {17, token::identifier}, 11},
	{ {17, token::type_string}, 12},
	{ {17, token::type_double}, 13},
	{ {17, token::type_int}, 14},
	{ {18, token::open_paren}, 10},
	{ {18, token::identifier}, 11},
	{ {18, token::type_string}, 12},
	{ {18, token::type_double}, 13},
	{ {18, token::type_int}, 14},
	{ {19, token::open_paren}, 10},
	{ {19, token::identifier}, 11},
	{ {19, token::type_string}, 12},
	{ {19, token::type_double}, 13},
	{ {19, token::type_int}, 14},
	{ {20, token::open_paren}, 10},
	{ {20, token::identifier}, 11},
	{ {20, token::type_string}, 12},
	{ {20, token::type_double}, 13},
	{ {20, token::type_int}, 14},
	{ {21, token::open_paren}, 10},
	{ {21, token::identifier}, 11},
	{ {21, token::type_string}, 12},
	{ {21, token::type_double}, 13},
	{ {21, token::type_int}, 14},
	{ {22, token::open_paren}, 10},
	{ {22, token::identifier}, 11},
	{ {22, token::type_string}, 12},
	{ {22, token::type_double}, 13},
	{ {22, token::type_int}, 14},
	{ {23, token::open_paren}, 28},
	{ {23, token::identifier}, 29},
	{ {23, token::type_string}, 30},
	{ {23, token::type_double}, 31},
	{ {23, token::type_int}, 32},
	{ {24, token::close_paren}, 42},
	{ {25, token::op_plus}, 43},
	{ {25, token::op_minus}, 44},
	{ {25, token::close_paren}, -10},
	{ {25, token::newline}, -10},
	{ {26, token::op_plus}, -16},
	{ {26, token::op_minus}, -16},
	{ {26, token::op_mult}, 45},
	{ {26, token::op_div}, 46},
	{ {26, token::op_idiv}, 47},
	{ {26, token::op_mod}, 48},
	{ {26, token::op_pow}, 49},
	{ {26, token::close_paren}, -16},
	{ {26, token::newline}, -16},
	{ {27, token::op_plus}, -19},
	{ {27, token::op_minus}, -19},
	{ {27, token::op_mult}, -19},
	{ {27, token::op_div}, -19},
	{ {27, token::op_idiv}, -19},
	{ {27, token::op_mod}, -19},
	{ {27, token::op_pow}, -19},
	{ {27, token::close_paren}, -19},
	{ {27, token::newline}, -19},
	{ {28, token::open_paren}, 28},
	{ {28, token::identifier}, 29},
	{ {28, token::type_string}, 30},
	{ {28, token::type_double}, 31},
	{ {28, token::type_int}, 32},
	{ {29, token::op_plus}, -18},
	{ {29, token::op_minus}, -18},
	{ {29, token::op_mult}, -18},
	{ {29, token::op_div}, -18},
	{ {29, token::op_idiv}, -18},
	{ {29, token::op_mod}, -18},
	{ {29, token::op_pow}, -18},
	{ {29, token::close_paren}, -18},
	{ {29, token::newline}, -18},
	{ {30, token::op_plus}, -22},
	{ {30, token::op_minus}, -22},
	{ {30, token::op_mult}, -22},
	{ {30, token::op_div}, -22},
	{ {30, token::op_idiv}, -22},
	{ {30, token::op_mod}, -22},
	{ {30, token::op_pow}, -22},
	{ {30, token::close_paren}, -22},
	{ {30, token::newline}, -22},
	{ {31, token::op_plus}, -20},
	{ {31, token::op_minus}, -20},
	{ {31, token::op_mult}, -20},
	{ {31, token::op_div}, -20},
	{ {31, token::op_idiv}, -20},
	{ {31, token::op_mod}, -20},
	{ {31, token::op_pow}, -20},
	{ {31, token::close_paren}, -20},
	{ {31, token::newline}, -20},
	{ {32, token::op_plus}, -21},
	{ {32, token::op_minus}, -21},
	{ {32, token::op_mult}, -21},
	{ {32, token::op_div}, -21},
	{ {32, token::op_idiv}, -21},
	{ {32, token::op_mod}, -21},
	{ {32, token::op_pow}, -21},
	{ {32, token::close_paren}, -21},
	{ {32, token::newline}, -21},
	{ {33, token::eof}, -3},
	{ {34, token::close_paren}, -8},
	{ {34, token::newline}, -8},
	{ {35, token::close_paren}, -9},
	{ {35, token::newline}, -9},
	{ {36, token::op_plus}, -11},
	{ {36, token::op_minus}, -11},
	{ {36, token::close_paren}, -11},
	{ {36, token::newline}, -11},
	{ {37, token::op_plus}, -12},
	{ {37, token::op_minus}, -12},
	{ {37, token::close_paren}, -12},
	{ {37, token::newline}, -12},
	{ {38, token::op_plus}, -13},
	{ {38, token::op_minus}, -13},
	{ {38, token::close_paren}, -13},
	{ {38, token::newline}, -13},
	{ {39, token::op_plus}, -14},
	{ {39, token::op_minus}, -14},
	{ {39, token::close_paren}, -14},
	{ {39, token::newline}, -14},
	{ {40, token::op_plus}, -15},
	{ {40, token::op_minus}, -15},
	{ {40, token::close_paren}, -15},
	{ {40, token::newline}, -15},
	{ {41, token::close_paren}, 51},
	{ {42, token::op_plus}, -17},
	{ {42, token::op_minus}, -17},
	{ {42, token::op_mult}, -17},
	{ {42, token::op_div}, -17},
	{ {42, token::op_idiv}, -17},
	{ {42, token::op_mod}, -17},
	{ {42, token::op_pow}, -17},
	{ {42, token::close_paren}, -17},
	{ {42, token::newline}, -17},
	{ {43, token::open_paren}, 28},
	{ {43, token::identifier}, 29},
	{ {43, token::type_string}, 30},
	{ {43, token::type_double}, 31},
	{ {43, token::type_int}, 32},
	{ {44, token::open_paren}, 28},
	{ {44, token::identifier}, 29},
	{ {44, token::type_string}, 30},
	{ {44, token::type_double}, 31},
	{ {44, token::type_int}, 32},
	{ {45, token::open_paren}, 28},
	{ {45, token::identifier}, 29},
	{ {45, token::type_string}, 30},
	{ {45, token::type_double}, 31},
	{ {45, token::type_int}, 32},
	{ {46, token::open_paren}, 28},
	{ {46, token::identifier}, 29},
	{ {46, token::type_string}, 30},
	{ {46, token::type_double}, 31},
	{ {46, token::type_int}, 32},
	{ {47, token::open_paren}, 28},
	{ {47, token::identifier}, 29},
	{ {47, token::type_string}, 30},
	{ {47, token::type_double}, 31},
	{ {47, token::type_int}, 32},
	{ {48, token::open_paren}, 28},
	{ {48, token::identifier}, 29},
	{ {48, token::type_string}, 30},
	{ {48, token::type_double}, 31},
	{ {48, token::type_int}, 32},
	{ {49, token::open_paren}, 28},
	{ {49, token::identifier}, 29},
	{ {49, token::type_string}, 30},
	{ {49, token::type_double}, 31},
	{ {49, token::type_int}, 32},
	{ {50, token::close_paren}, 59},
	{ {51, token::newline}, -7},
	{ {52, token::close_paren}, -8},
	{ {52, token::newline}, -8},
	{ {53, token::close_paren}, -9},
	{ {53, token::newline}, -9},
	{ {54, token::op_plus}, -11},
	{ {54, token::op_minus}, -11},
	{ {54, token::close_paren}, -11},
	{ {54, token::newline}, -11},
	{ {55, token::op_plus}, -12},
	{ {55, token::op_minus}, -12},
	{ {55, token::close_paren}, -12},
	{ {55, token::newline}, -12},
	{ {56, token::op_plus}, -13},
	{ {56, token::op_minus}, -13},
	{ {56, token::close_paren}, -13},
	{ {56, token::newline}, -13},
	{ {57, token::op_plus}, -14},
	{ {57, token::op_minus}, -14},
	{ {57, token::close_paren}, -14},
	{ {57, token::newline}, -14},
	{ {58, token::op_plus}, -15},
	{ {58, token::op_minus}, -15},
	{ {58, token::close_paren}, -15},
	{ {58, token::newline}, -15},
	{ {59, token::op_plus}, -17},
	{ {59, token::op_minus}, -17},
	{ {59, token::op_mult}, -17},
	{ {59, token::op_div}, -17},
	{ {59, token::op_idiv}, -17},
	{ {59, token::op_mod}, -17},
	{ {59, token::op_pow}, -17},
	{ {59, token::close_paren}, -17},
	{ {59, token::newline}, -17},
};
static map<pair<int, token>, int> tgoto = {
	{ {0, token::StatementSuite}, 1},
	{ {0, token::StatementList}, 2},
	{ {0, token::Statement}, 3},
	{ {0, token::Print}, 4},
	{ {0, token::Expression}, 5},
	{ {0, token::MultiplicativeExpression}, 6},
	{ {0, token::BaseExpression}, 7},
	{ {0, token::Literal}, 8},
	{ {10, token::Expression}, 24},
	{ {10, token::MultiplicativeExpression}, 25},
	{ {10, token::BaseExpression}, 26},
	{ {10, token::Literal}, 27},
	{ {15, token::StatementList}, 33},
	{ {15, token::Statement}, 3},
	{ {15, token::Print}, 4},
	{ {15, token::Expression}, 5},
	{ {15, token::MultiplicativeExpression}, 6},
	{ {15, token::BaseExpression}, 7},
	{ {15, token::Literal}, 8},
	{ {16, token::Expression}, 34},
	{ {16, token::MultiplicativeExpression}, 6},
	{ {16, token::BaseExpression}, 7},
	{ {16, token::Literal}, 8},
	{ {17, token::Expression}, 35},
	{ {17, token::MultiplicativeExpression}, 6},
	{ {17, token::BaseExpression}, 7},
	{ {17, token::Literal}, 8},
	{ {18, token::MultiplicativeExpression}, 36},
	{ {18, token::BaseExpression}, 7},
	{ {18, token::Literal}, 8},
	{ {19, token::MultiplicativeExpression}, 37},
	{ {19, token::BaseExpression}, 7},
	{ {19, token::Literal}, 8},
	{ {20, token::MultiplicativeExpression}, 38},
	{ {20, token::BaseExpression}, 7},
	{ {20, token::Literal}, 8},
	{ {21, token::MultiplicativeExpression}, 39},
	{ {21, token::BaseExpression}, 7},
	{ {21, token::Literal}, 8},
	{ {22, token::MultiplicativeExpression}, 40},
	{ {22, token::BaseExpression}, 7},
	{ {22, token::Literal}, 8},
	{ {23, token::Expression}, 41},
	{ {23, token::MultiplicativeExpression}, 25},
	{ {23, token::BaseExpression}, 26},
	{ {23, token::Literal}, 27},
	{ {28, token::Expression}, 50},
	{ {28, token::MultiplicativeExpression}, 25},
	{ {28, token::BaseExpression}, 26},
	{ {28, token::Literal}, 27},
	{ {43, token::Expression}, 52},
	{ {43, token::MultiplicativeExpression}, 25},
	{ {43, token::BaseExpression}, 26},
	{ {43, token::Literal}, 27},
	{ {44, token::Expression}, 53},
	{ {44, token::MultiplicativeExpression}, 25},
	{ {44, token::BaseExpression}, 26},
	{ {44, token::Literal}, 27},
	{ {45, token::MultiplicativeExpression}, 54},
	{ {45, token::BaseExpression}, 26},
	{ {45, token::Literal}, 27},
	{ {46, token::MultiplicativeExpression}, 55},
	{ {46, token::BaseExpression}, 26},
	{ {46, token::Literal}, 27},
	{ {47, token::MultiplicativeExpression}, 56},
	{ {47, token::BaseExpression}, 26},
	{ {47, token::Literal}, 27},
	{ {48, token::MultiplicativeExpression}, 57},
	{ {48, token::BaseExpression}, 26},
	{ {48, token::Literal}, 27},
	{ {49, token::MultiplicativeExpression}, 58},
	{ {49, token::BaseExpression}, 26},
	{ {49, token::Literal}, 27},
};

static void reduce(int decision, stack<Node*> *value_stack) {
	switch(decision) {
		case 1: {
			auto* StatementSuite = value_stack->top();
			value_stack->pop();
			auto* node = new Mango1 {StatementSuite};
			value_stack->push(node);
			break;
		}
		case 2: {
			auto* StatementList = value_stack->top();
			value_stack->pop();
			auto* node = new StatementSuite1 {StatementList};
			value_stack->push(node);
			break;
		}
		case 3: {
			auto* StatementList = value_stack->top();
			value_stack->pop();
			auto* Statement = value_stack->top();
			value_stack->pop();
			auto* node = new StatementList1 {Statement, StatementList};
			value_stack->push(node);
			break;
		}
		case 4: {
			auto* Statement = value_stack->top();
			value_stack->pop();
			auto* node = new StatementList2 {Statement};
			value_stack->push(node);
			break;
		}
		case 5: {
			auto* Print = value_stack->top();
			value_stack->pop();
			auto* node = new Statement1 {Print};
			value_stack->push(node);
			break;
		}
		case 6: {
			auto* Expression = value_stack->top();
			value_stack->pop();
			auto* node = new Statement2 {Expression};
			value_stack->push(node);
			break;
		}
		case 7: {
			auto* Expression = value_stack->top();
			value_stack->pop();
			auto* node = new Print1 {Expression};
			value_stack->push(node);
			break;
		}
		case 8: {
			auto* Expression = value_stack->top();
			value_stack->pop();
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* node = new Expression1 {MultiplicativeExpression, Expression};
			value_stack->push(node);
			break;
		}
		case 9: {
			auto* Expression = value_stack->top();
			value_stack->pop();
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* node = new Expression2 {MultiplicativeExpression, Expression};
			value_stack->push(node);
			break;
		}
		case 10: {
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* node = new Expression3 {MultiplicativeExpression};
			value_stack->push(node);
			break;
		}
		case 11: {
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* BaseExpression = value_stack->top();
			value_stack->pop();
			auto* node = new MultiplicativeExpression1 {BaseExpression, MultiplicativeExpression};
			value_stack->push(node);
			break;
		}
		case 12: {
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* BaseExpression = value_stack->top();
			value_stack->pop();
			auto* node = new MultiplicativeExpression2 {BaseExpression, MultiplicativeExpression};
			value_stack->push(node);
			break;
		}
		case 13: {
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* BaseExpression = value_stack->top();
			value_stack->pop();
			auto* node = new MultiplicativeExpression3 {BaseExpression, MultiplicativeExpression};
			value_stack->push(node);
			break;
		}
		case 14: {
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* BaseExpression = value_stack->top();
			value_stack->pop();
			auto* node = new MultiplicativeExpression4 {BaseExpression, MultiplicativeExpression};
			value_stack->push(node);
			break;
		}
		case 15: {
			auto* MultiplicativeExpression = value_stack->top();
			value_stack->pop();
			auto* BaseExpression = value_stack->top();
			value_stack->pop();
			auto* node = new MultiplicativeExpression5 {BaseExpression, MultiplicativeExpression};
			value_stack->push(node);
			break;
		}
		case 16: {
			auto* BaseExpression = value_stack->top();
			value_stack->pop();
			auto* node = new MultiplicativeExpression6 {BaseExpression};
			value_stack->push(node);
			break;
		}
		case 17: {
			auto* Expression = value_stack->top();
			value_stack->pop();
			auto* node = new BaseExpression1 {Expression};
			value_stack->push(node);
			break;
		}
		case 18: {
			auto* identifier = value_stack->top();
			value_stack->pop();
			auto* node = new BaseExpression2 {identifier};
			value_stack->push(node);
			break;
		}
		case 19: {
			auto* Literal = value_stack->top();
			value_stack->pop();
			auto* node = new BaseExpression3 {Literal};
			value_stack->push(node);
			break;
		}
		case 20: {
			auto* type_double = value_stack->top();
			value_stack->pop();
			auto* node = new Literal1 {type_double};
			value_stack->push(node);
			break;
		}
		case 21: {
			auto* type_int = value_stack->top();
			value_stack->pop();
			auto* node = new Literal2 {type_int};
			value_stack->push(node);
			break;
		}
		case 22: {
			auto* type_string = value_stack->top();
			value_stack->pop();
			auto* node = new Literal3 {type_string};
			value_stack->push(node);
			break;
		}
		default:
			exit(1);
	}
}

#endif