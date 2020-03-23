#ifndef MANGOREVISITEDCPPCLION_PARSE_TABLE_H
#define MANGOREVISITEDCPPCLION_PARSE_TABLE_H

#include <map>
#include <stack>
#include <memory>

#include "grammar.h"
#include "tree.h"

using namespace std;

namespace mango {static map<pair<int, token>, int> taction = {
		{ {0, kw_print}, 11},
		{ {0, open_paren}, 12},
		{ {0, identifier}, 13},
		{ {0, type_string}, 14},
		{ {0, type_double}, 15},
		{ {0, type_int}, 16},
		{ {0, type_boolean}, 17},
		{ {1, eof}, 0},
		{ {2, eof}, -2},
		{ {3, semicolon}, 18},
		{ {4, semicolon}, -5},
		{ {5, semicolon}, -6},
		{ {6, semicolon}, -8},
		{ {7, semicolon}, -9},
		{ {8, semicolon}, -7},
		{ {9, op_plus}, 19},
		{ {9, op_minus}, 20},
		{ {9, close_paren}, -14},
		{ {9, semicolon}, -14},
		{ {10, op_plus}, -20},
		{ {10, op_minus}, -20},
		{ {10, op_mult}, 21},
		{ {10, op_div}, 22},
		{ {10, op_idiv}, 23},
		{ {10, op_mod}, 24},
		{ {10, op_pow}, 25},
		{ {10, close_paren}, -20},
		{ {10, semicolon}, -20},
		{ {11, open_paren}, 26},
		{ {12, open_paren}, 30},
		{ {12, identifier}, 31},
		{ {12, type_string}, 32},
		{ {12, type_double}, 33},
		{ {12, type_int}, 34},
		{ {12, type_boolean}, 35},
		{ {13, op_plus}, -22},
		{ {13, op_minus}, -22},
		{ {13, op_mult}, -22},
		{ {13, op_div}, -22},
		{ {13, op_idiv}, -22},
		{ {13, op_mod}, -22},
		{ {13, op_pow}, -22},
		{ {13, op_equals}, 36},
		{ {13, close_paren}, -22},
		{ {13, semicolon}, -22},
		{ {14, op_plus}, -25},
		{ {14, op_minus}, -25},
		{ {14, op_mult}, -25},
		{ {14, op_div}, -25},
		{ {14, op_idiv}, -25},
		{ {14, op_mod}, -25},
		{ {14, op_pow}, -25},
		{ {14, close_paren}, -25},
		{ {14, semicolon}, -25},
		{ {15, op_plus}, -23},
		{ {15, op_minus}, -23},
		{ {15, op_mult}, -23},
		{ {15, op_div}, -23},
		{ {15, op_idiv}, -23},
		{ {15, op_mod}, -23},
		{ {15, op_pow}, -23},
		{ {15, close_paren}, -23},
		{ {15, semicolon}, -23},
		{ {16, op_plus}, -24},
		{ {16, op_minus}, -24},
		{ {16, op_mult}, -24},
		{ {16, op_div}, -24},
		{ {16, op_idiv}, -24},
		{ {16, op_mod}, -24},
		{ {16, op_pow}, -24},
		{ {16, close_paren}, -24},
		{ {16, semicolon}, -24},
		{ {17, op_plus}, -26},
		{ {17, op_minus}, -26},
		{ {17, op_mult}, -26},
		{ {17, op_div}, -26},
		{ {17, op_idiv}, -26},
		{ {17, op_mod}, -26},
		{ {17, op_pow}, -26},
		{ {17, close_paren}, -26},
		{ {17, semicolon}, -26},
		{ {18, kw_print}, 11},
		{ {18, open_paren}, 12},
		{ {18, identifier}, 13},
		{ {18, type_string}, 14},
		{ {18, type_double}, 15},
		{ {18, type_int}, 16},
		{ {18, type_boolean}, 17},
		{ {18, eof}, -4},
		{ {19, open_paren}, 12},
		{ {19, identifier}, 39},
		{ {19, type_string}, 14},
		{ {19, type_double}, 15},
		{ {19, type_int}, 16},
		{ {19, type_boolean}, 17},
		{ {20, open_paren}, 12},
		{ {20, identifier}, 39},
		{ {20, type_string}, 14},
		{ {20, type_double}, 15},
		{ {20, type_int}, 16},
		{ {20, type_boolean}, 17},
		{ {21, open_paren}, 12},
		{ {21, identifier}, 39},
		{ {21, type_string}, 14},
		{ {21, type_double}, 15},
		{ {21, type_int}, 16},
		{ {21, type_boolean}, 17},
		{ {22, open_paren}, 12},
		{ {22, identifier}, 39},
		{ {22, type_string}, 14},
		{ {22, type_double}, 15},
		{ {22, type_int}, 16},
		{ {22, type_boolean}, 17},
		{ {23, open_paren}, 12},
		{ {23, identifier}, 39},
		{ {23, type_string}, 14},
		{ {23, type_double}, 15},
		{ {23, type_int}, 16},
		{ {23, type_boolean}, 17},
		{ {24, open_paren}, 12},
		{ {24, identifier}, 39},
		{ {24, type_string}, 14},
		{ {24, type_double}, 15},
		{ {24, type_int}, 16},
		{ {24, type_boolean}, 17},
		{ {25, open_paren}, 12},
		{ {25, identifier}, 39},
		{ {25, type_string}, 14},
		{ {25, type_double}, 15},
		{ {25, type_int}, 16},
		{ {25, type_boolean}, 17},
		{ {26, open_paren}, 30},
		{ {26, identifier}, 31},
		{ {26, type_string}, 32},
		{ {26, type_double}, 33},
		{ {26, type_int}, 34},
		{ {26, type_boolean}, 35},
		{ {27, close_paren}, 47},
		{ {28, op_plus}, 48},
		{ {28, op_minus}, 49},
		{ {28, close_paren}, -14},
		{ {28, semicolon}, -14},
		{ {29, op_plus}, -20},
		{ {29, op_minus}, -20},
		{ {29, op_mult}, 50},
		{ {29, op_div}, 51},
		{ {29, op_idiv}, 52},
		{ {29, op_mod}, 53},
		{ {29, op_pow}, 54},
		{ {29, close_paren}, -20},
		{ {29, semicolon}, -20},
		{ {30, open_paren}, 30},
		{ {30, identifier}, 31},
		{ {30, type_string}, 32},
		{ {30, type_double}, 33},
		{ {30, type_int}, 34},
		{ {30, type_boolean}, 35},
		{ {31, op_plus}, -22},
		{ {31, op_minus}, -22},
		{ {31, op_mult}, -22},
		{ {31, op_div}, -22},
		{ {31, op_idiv}, -22},
		{ {31, op_mod}, -22},
		{ {31, op_pow}, -22},
		{ {31, close_paren}, -22},
		{ {31, semicolon}, -22},
		{ {32, op_plus}, -25},
		{ {32, op_minus}, -25},
		{ {32, op_mult}, -25},
		{ {32, op_div}, -25},
		{ {32, op_idiv}, -25},
		{ {32, op_mod}, -25},
		{ {32, op_pow}, -25},
		{ {32, close_paren}, -25},
		{ {32, semicolon}, -25},
		{ {33, op_plus}, -23},
		{ {33, op_minus}, -23},
		{ {33, op_mult}, -23},
		{ {33, op_div}, -23},
		{ {33, op_idiv}, -23},
		{ {33, op_mod}, -23},
		{ {33, op_pow}, -23},
		{ {33, close_paren}, -23},
		{ {33, semicolon}, -23},
		{ {34, op_plus}, -24},
		{ {34, op_minus}, -24},
		{ {34, op_mult}, -24},
		{ {34, op_div}, -24},
		{ {34, op_idiv}, -24},
		{ {34, op_mod}, -24},
		{ {34, op_pow}, -24},
		{ {34, close_paren}, -24},
		{ {34, semicolon}, -24},
		{ {35, op_plus}, -26},
		{ {35, op_minus}, -26},
		{ {35, op_mult}, -26},
		{ {35, op_div}, -26},
		{ {35, op_idiv}, -26},
		{ {35, op_mod}, -26},
		{ {35, op_pow}, -26},
		{ {35, close_paren}, -26},
		{ {35, semicolon}, -26},
		{ {36, open_paren}, 12},
		{ {36, identifier}, 39},
		{ {36, type_string}, 14},
		{ {36, type_double}, 15},
		{ {36, type_int}, 16},
		{ {36, type_boolean}, 17},
		{ {37, eof}, -3},
		{ {38, close_paren}, -12},
		{ {38, semicolon}, -12},
		{ {39, op_plus}, -22},
		{ {39, op_minus}, -22},
		{ {39, op_mult}, -22},
		{ {39, op_div}, -22},
		{ {39, op_idiv}, -22},
		{ {39, op_mod}, -22},
		{ {39, op_pow}, -22},
		{ {39, close_paren}, -22},
		{ {39, semicolon}, -22},
		{ {40, close_paren}, -13},
		{ {40, semicolon}, -13},
		{ {41, op_plus}, -15},
		{ {41, op_minus}, -15},
		{ {41, close_paren}, -15},
		{ {41, semicolon}, -15},
		{ {42, op_plus}, -16},
		{ {42, op_minus}, -16},
		{ {42, close_paren}, -16},
		{ {42, semicolon}, -16},
		{ {43, op_plus}, -17},
		{ {43, op_minus}, -17},
		{ {43, close_paren}, -17},
		{ {43, semicolon}, -17},
		{ {44, op_plus}, -18},
		{ {44, op_minus}, -18},
		{ {44, close_paren}, -18},
		{ {44, semicolon}, -18},
		{ {45, op_plus}, -19},
		{ {45, op_minus}, -19},
		{ {45, close_paren}, -19},
		{ {45, semicolon}, -19},
		{ {46, close_paren}, 57},
		{ {47, op_plus}, -21},
		{ {47, op_minus}, -21},
		{ {47, op_mult}, -21},
		{ {47, op_div}, -21},
		{ {47, op_idiv}, -21},
		{ {47, op_mod}, -21},
		{ {47, op_pow}, -21},
		{ {47, close_paren}, -21},
		{ {47, semicolon}, -21},
		{ {48, open_paren}, 30},
		{ {48, identifier}, 31},
		{ {48, type_string}, 32},
		{ {48, type_double}, 33},
		{ {48, type_int}, 34},
		{ {48, type_boolean}, 35},
		{ {49, open_paren}, 30},
		{ {49, identifier}, 31},
		{ {49, type_string}, 32},
		{ {49, type_double}, 33},
		{ {49, type_int}, 34},
		{ {49, type_boolean}, 35},
		{ {50, open_paren}, 30},
		{ {50, identifier}, 31},
		{ {50, type_string}, 32},
		{ {50, type_double}, 33},
		{ {50, type_int}, 34},
		{ {50, type_boolean}, 35},
		{ {51, open_paren}, 30},
		{ {51, identifier}, 31},
		{ {51, type_string}, 32},
		{ {51, type_double}, 33},
		{ {51, type_int}, 34},
		{ {51, type_boolean}, 35},
		{ {52, open_paren}, 30},
		{ {52, identifier}, 31},
		{ {52, type_string}, 32},
		{ {52, type_double}, 33},
		{ {52, type_int}, 34},
		{ {52, type_boolean}, 35},
		{ {53, open_paren}, 30},
		{ {53, identifier}, 31},
		{ {53, type_string}, 32},
		{ {53, type_double}, 33},
		{ {53, type_int}, 34},
		{ {53, type_boolean}, 35},
		{ {54, open_paren}, 30},
		{ {54, identifier}, 31},
		{ {54, type_string}, 32},
		{ {54, type_double}, 33},
		{ {54, type_int}, 34},
		{ {54, type_boolean}, 35},
		{ {55, close_paren}, 65},
		{ {56, semicolon}, -11},
		{ {57, semicolon}, -10},
		{ {58, close_paren}, -12},
		{ {58, semicolon}, -12},
		{ {59, close_paren}, -13},
		{ {59, semicolon}, -13},
		{ {60, op_plus}, -15},
		{ {60, op_minus}, -15},
		{ {60, close_paren}, -15},
		{ {60, semicolon}, -15},
		{ {61, op_plus}, -16},
		{ {61, op_minus}, -16},
		{ {61, close_paren}, -16},
		{ {61, semicolon}, -16},
		{ {62, op_plus}, -17},
		{ {62, op_minus}, -17},
		{ {62, close_paren}, -17},
		{ {62, semicolon}, -17},
		{ {63, op_plus}, -18},
		{ {63, op_minus}, -18},
		{ {63, close_paren}, -18},
		{ {63, semicolon}, -18},
		{ {64, op_plus}, -19},
		{ {64, op_minus}, -19},
		{ {64, close_paren}, -19},
		{ {64, semicolon}, -19},
		{ {65, op_plus}, -21},
		{ {65, op_minus}, -21},
		{ {65, op_mult}, -21},
		{ {65, op_div}, -21},
		{ {65, op_idiv}, -21},
		{ {65, op_mod}, -21},
		{ {65, op_pow}, -21},
		{ {65, close_paren}, -21},
		{ {65, semicolon}, -21},
};
static map<pair<int, token>, int> tgoto = {
		{ {0, StatementSuite}, 1},
		{ {0, StatementList}, 2},
		{ {0, Statement}, 3},
		{ {0, SimpleStatement}, 4},
		{ {0, ComplexStatement}, 5},
		{ {0, Print}, 6},
		{ {0, Assignment}, 7},
		{ {0, Expression}, 8},
		{ {0, MultiplicativeExpression}, 9},
		{ {0, BaseExpression}, 10},
		{ {12, Expression}, 27},
		{ {12, MultiplicativeExpression}, 28},
		{ {12, BaseExpression}, 29},
		{ {18, StatementList}, 37},
		{ {18, Statement}, 3},
		{ {18, SimpleStatement}, 4},
		{ {18, ComplexStatement}, 5},
		{ {18, Print}, 6},
		{ {18, Assignment}, 7},
		{ {18, Expression}, 8},
		{ {18, MultiplicativeExpression}, 9},
		{ {18, BaseExpression}, 10},
		{ {19, Expression}, 38},
		{ {19, MultiplicativeExpression}, 9},
		{ {19, BaseExpression}, 10},
		{ {20, Expression}, 40},
		{ {20, MultiplicativeExpression}, 9},
		{ {20, BaseExpression}, 10},
		{ {21, MultiplicativeExpression}, 41},
		{ {21, BaseExpression}, 10},
		{ {22, MultiplicativeExpression}, 42},
		{ {22, BaseExpression}, 10},
		{ {23, MultiplicativeExpression}, 43},
		{ {23, BaseExpression}, 10},
		{ {24, MultiplicativeExpression}, 44},
		{ {24, BaseExpression}, 10},
		{ {25, MultiplicativeExpression}, 45},
		{ {25, BaseExpression}, 10},
		{ {26, Expression}, 46},
		{ {26, MultiplicativeExpression}, 28},
		{ {26, BaseExpression}, 29},
		{ {30, Expression}, 55},
		{ {30, MultiplicativeExpression}, 28},
		{ {30, BaseExpression}, 29},
		{ {36, Expression}, 56},
		{ {36, MultiplicativeExpression}, 9},
		{ {36, BaseExpression}, 10},
		{ {48, Expression}, 58},
		{ {48, MultiplicativeExpression}, 28},
		{ {48, BaseExpression}, 29},
		{ {49, Expression}, 59},
		{ {49, MultiplicativeExpression}, 28},
		{ {49, BaseExpression}, 29},
		{ {50, MultiplicativeExpression}, 60},
		{ {50, BaseExpression}, 29},
		{ {51, MultiplicativeExpression}, 61},
		{ {51, BaseExpression}, 29},
		{ {52, MultiplicativeExpression}, 62},
		{ {52, BaseExpression}, 29},
		{ {53, MultiplicativeExpression}, 63},
		{ {53, BaseExpression}, 29},
		{ {54, MultiplicativeExpression}, 64},
		{ {54, BaseExpression}, 29},
};

static void reduce(int decision, stack<Node*>& value_stack) {
		switch(decision) {
			case 1: {
				auto StatementSuite_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Mango1 {StatementSuite_1};
				value_stack.push(node);
				break;
			}
			case 2: {
				auto StatementList_1 = value_stack.top();
				value_stack.pop();
				auto* node = new StatementSuite1 {StatementList_1};
				value_stack.push(node);
				break;
			}
			case 3: {
				auto StatementList_1 = value_stack.top();
				value_stack.pop();
				auto Statement_2 = value_stack.top();
				value_stack.pop();
				auto* node = new StatementList1 {Statement_2, StatementList_1};
				value_stack.push(node);
				break;
			}
			case 4: {
				auto Statement_1 = value_stack.top();
				value_stack.pop();
				auto* node = new StatementList2 {Statement_1};
				value_stack.push(node);
				break;
			}
			case 5: {
				auto SimpleStatement_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Statement1 {SimpleStatement_1};
				value_stack.push(node);
				break;
			}
			case 6: {
				auto ComplexStatement_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Statement2 {ComplexStatement_1};
				value_stack.push(node);
				break;
			}
			case 7: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new SimpleStatement1 {Expression_1};
				value_stack.push(node);
				break;
			}
			case 8: {
				auto Print_1 = value_stack.top();
				value_stack.pop();
				auto* node = new ComplexStatement1 {Print_1};
				value_stack.push(node);
				break;
			}
			case 9: {
				auto Assignment_1 = value_stack.top();
				value_stack.pop();
				auto* node = new ComplexStatement2 {Assignment_1};
				value_stack.push(node);
				break;
			}
			case 10: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Print1 {Expression_1};
				value_stack.push(node);
				break;
			}
			case 11: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto identifier_2 = value_stack.top();
				value_stack.pop();
				auto* node = new Assignment1 {identifier_2, Expression_1};
				value_stack.push(node);
				break;
			}
			case 12: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto MultiplicativeExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new Expression1 {MultiplicativeExpression_2, Expression_1};
				value_stack.push(node);
				break;
			}
			case 13: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto MultiplicativeExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new Expression2 {MultiplicativeExpression_2, Expression_1};
				value_stack.push(node);
				break;
			}
			case 14: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Expression3 {MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 15: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression1 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 16: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression2 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 17: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression3 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 18: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression4 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 19: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression5 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 20: {
				auto BaseExpression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression6 {BaseExpression_1};
				value_stack.push(node);
				break;
			}
			case 21: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression1 {Expression_1};
				value_stack.push(node);
				break;
			}
			case 22: {
				auto identifier_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression2 {identifier_1};
				value_stack.push(node);
				break;
			}
			case 23: {
				auto type_double_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression3 {type_double_1};
				value_stack.push(node);
				break;
			}
			case 24: {
				auto type_int_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression4 {type_int_1};
				value_stack.push(node);
				break;
			}
			case 25: {
				auto type_string_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression5 {type_string_1};
				value_stack.push(node);
				break;
			}
			case 26: {
				auto* node = new BaseExpression6 {};
				value_stack.push(node);
				break;
			}
			default:
				exit(1);
		}
	}

}
#endif