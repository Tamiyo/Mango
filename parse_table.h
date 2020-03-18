#ifndef MANGOREVISITEDCPPCLION_PARSE_TABLE_H
#define MANGOREVISITEDCPPCLION_PARSE_TABLE_H

#include <map>
#include <stack>
#include <memory>

#include "grammar.h"
#include "tree.h"

using namespace std;

namespace mango {static map<pair<int, token>, int> taction = {
		{ {0, kw_print}, 12},
		{ {0, kw_for}, 13},
		{ {0, open_paren}, 14},
		{ {0, identifier}, 15},
		{ {0, type_string}, 16},
		{ {0, type_double}, 17},
		{ {0, type_int}, 18},
		{ {0, type_boolean}, 19},
		{ {1, eof}, 0},
		{ {2, close_curly}, -2},
		{ {2, eof}, -2},
		{ {3, semicolon}, 20},
		{ {4, semicolon}, -5},
		{ {5, semicolon}, -6},
		{ {6, semicolon}, -8},
		{ {7, semicolon}, -9},
		{ {8, semicolon}, -10},
		{ {9, semicolon}, -7},
		{ {10, op_plus}, 21},
		{ {10, op_minus}, 22},
		{ {10, close_paren}, -16},
		{ {10, semicolon}, -16},
		{ {11, op_plus}, -22},
		{ {11, op_minus}, -22},
		{ {11, op_mult}, 23},
		{ {11, op_div}, 24},
		{ {11, op_idiv}, 25},
		{ {11, op_mod}, 26},
		{ {11, op_pow}, 27},
		{ {11, close_paren}, -22},
		{ {11, semicolon}, -22},
		{ {12, open_paren}, 28},
		{ {13, identifier}, 29},
		{ {14, open_paren}, 33},
		{ {14, identifier}, 34},
		{ {14, type_string}, 35},
		{ {14, type_double}, 36},
		{ {14, type_int}, 37},
		{ {14, type_boolean}, 38},
		{ {15, op_plus}, -24},
		{ {15, op_minus}, -24},
		{ {15, op_mult}, -24},
		{ {15, op_div}, -24},
		{ {15, op_idiv}, -24},
		{ {15, op_mod}, -24},
		{ {15, op_pow}, -24},
		{ {15, op_equals}, 39},
		{ {15, close_paren}, -24},
		{ {15, semicolon}, -24},
		{ {16, op_plus}, -27},
		{ {16, op_minus}, -27},
		{ {16, op_mult}, -27},
		{ {16, op_div}, -27},
		{ {16, op_idiv}, -27},
		{ {16, op_mod}, -27},
		{ {16, op_pow}, -27},
		{ {16, close_paren}, -27},
		{ {16, semicolon}, -27},
		{ {17, op_plus}, -25},
		{ {17, op_minus}, -25},
		{ {17, op_mult}, -25},
		{ {17, op_div}, -25},
		{ {17, op_idiv}, -25},
		{ {17, op_mod}, -25},
		{ {17, op_pow}, -25},
		{ {17, close_paren}, -25},
		{ {17, semicolon}, -25},
		{ {18, op_plus}, -26},
		{ {18, op_minus}, -26},
		{ {18, op_mult}, -26},
		{ {18, op_div}, -26},
		{ {18, op_idiv}, -26},
		{ {18, op_mod}, -26},
		{ {18, op_pow}, -26},
		{ {18, close_paren}, -26},
		{ {18, semicolon}, -26},
		{ {19, op_plus}, -28},
		{ {19, op_minus}, -28},
		{ {19, op_mult}, -28},
		{ {19, op_div}, -28},
		{ {19, op_idiv}, -28},
		{ {19, op_mod}, -28},
		{ {19, op_pow}, -28},
		{ {19, close_paren}, -28},
		{ {19, semicolon}, -28},
		{ {20, kw_print}, 12},
		{ {20, kw_for}, 13},
		{ {20, open_paren}, 14},
		{ {20, close_curly}, -4},
		{ {20, identifier}, 15},
		{ {20, type_string}, 16},
		{ {20, type_double}, 17},
		{ {20, type_int}, 18},
		{ {20, type_boolean}, 19},
		{ {20, eof}, -4},
		{ {21, open_paren}, 14},
		{ {21, identifier}, 42},
		{ {21, type_string}, 16},
		{ {21, type_double}, 17},
		{ {21, type_int}, 18},
		{ {21, type_boolean}, 19},
		{ {22, open_paren}, 14},
		{ {22, identifier}, 42},
		{ {22, type_string}, 16},
		{ {22, type_double}, 17},
		{ {22, type_int}, 18},
		{ {22, type_boolean}, 19},
		{ {23, open_paren}, 14},
		{ {23, identifier}, 42},
		{ {23, type_string}, 16},
		{ {23, type_double}, 17},
		{ {23, type_int}, 18},
		{ {23, type_boolean}, 19},
		{ {24, open_paren}, 14},
		{ {24, identifier}, 42},
		{ {24, type_string}, 16},
		{ {24, type_double}, 17},
		{ {24, type_int}, 18},
		{ {24, type_boolean}, 19},
		{ {25, open_paren}, 14},
		{ {25, identifier}, 42},
		{ {25, type_string}, 16},
		{ {25, type_double}, 17},
		{ {25, type_int}, 18},
		{ {25, type_boolean}, 19},
		{ {26, open_paren}, 14},
		{ {26, identifier}, 42},
		{ {26, type_string}, 16},
		{ {26, type_double}, 17},
		{ {26, type_int}, 18},
		{ {26, type_boolean}, 19},
		{ {27, open_paren}, 14},
		{ {27, identifier}, 42},
		{ {27, type_string}, 16},
		{ {27, type_double}, 17},
		{ {27, type_int}, 18},
		{ {27, type_boolean}, 19},
		{ {28, open_paren}, 33},
		{ {28, identifier}, 34},
		{ {28, type_string}, 35},
		{ {28, type_double}, 36},
		{ {28, type_int}, 37},
		{ {28, type_boolean}, 38},
		{ {29, colon}, 50},
		{ {30, close_paren}, 51},
		{ {31, op_plus}, 52},
		{ {31, op_minus}, 53},
		{ {31, close_paren}, -16},
		{ {31, semicolon}, -16},
		{ {32, op_plus}, -22},
		{ {32, op_minus}, -22},
		{ {32, op_mult}, 54},
		{ {32, op_div}, 55},
		{ {32, op_idiv}, 56},
		{ {32, op_mod}, 57},
		{ {32, op_pow}, 58},
		{ {32, close_paren}, -22},
		{ {32, semicolon}, -22},
		{ {33, open_paren}, 33},
		{ {33, identifier}, 34},
		{ {33, type_string}, 35},
		{ {33, type_double}, 36},
		{ {33, type_int}, 37},
		{ {33, type_boolean}, 38},
		{ {34, op_plus}, -24},
		{ {34, op_minus}, -24},
		{ {34, op_mult}, -24},
		{ {34, op_div}, -24},
		{ {34, op_idiv}, -24},
		{ {34, op_mod}, -24},
		{ {34, op_pow}, -24},
		{ {34, close_paren}, -24},
		{ {34, semicolon}, -24},
		{ {35, op_plus}, -27},
		{ {35, op_minus}, -27},
		{ {35, op_mult}, -27},
		{ {35, op_div}, -27},
		{ {35, op_idiv}, -27},
		{ {35, op_mod}, -27},
		{ {35, op_pow}, -27},
		{ {35, close_paren}, -27},
		{ {35, semicolon}, -27},
		{ {36, op_plus}, -25},
		{ {36, op_minus}, -25},
		{ {36, op_mult}, -25},
		{ {36, op_div}, -25},
		{ {36, op_idiv}, -25},
		{ {36, op_mod}, -25},
		{ {36, op_pow}, -25},
		{ {36, close_paren}, -25},
		{ {36, semicolon}, -25},
		{ {37, op_plus}, -26},
		{ {37, op_minus}, -26},
		{ {37, op_mult}, -26},
		{ {37, op_div}, -26},
		{ {37, op_idiv}, -26},
		{ {37, op_mod}, -26},
		{ {37, op_pow}, -26},
		{ {37, close_paren}, -26},
		{ {37, semicolon}, -26},
		{ {38, op_plus}, -28},
		{ {38, op_minus}, -28},
		{ {38, op_mult}, -28},
		{ {38, op_div}, -28},
		{ {38, op_idiv}, -28},
		{ {38, op_mod}, -28},
		{ {38, op_pow}, -28},
		{ {38, close_paren}, -28},
		{ {38, semicolon}, -28},
		{ {39, open_paren}, 14},
		{ {39, identifier}, 42},
		{ {39, type_string}, 16},
		{ {39, type_double}, 17},
		{ {39, type_int}, 18},
		{ {39, type_boolean}, 19},
		{ {40, close_curly}, -3},
		{ {40, eof}, -3},
		{ {41, close_paren}, -14},
		{ {41, semicolon}, -14},
		{ {42, op_plus}, -24},
		{ {42, op_minus}, -24},
		{ {42, op_mult}, -24},
		{ {42, op_div}, -24},
		{ {42, op_idiv}, -24},
		{ {42, op_mod}, -24},
		{ {42, op_pow}, -24},
		{ {42, close_paren}, -24},
		{ {42, semicolon}, -24},
		{ {43, close_paren}, -15},
		{ {43, semicolon}, -15},
		{ {44, op_plus}, -17},
		{ {44, op_minus}, -17},
		{ {44, close_paren}, -17},
		{ {44, semicolon}, -17},
		{ {45, op_plus}, -18},
		{ {45, op_minus}, -18},
		{ {45, close_paren}, -18},
		{ {45, semicolon}, -18},
		{ {46, op_plus}, -19},
		{ {46, op_minus}, -19},
		{ {46, close_paren}, -19},
		{ {46, semicolon}, -19},
		{ {47, op_plus}, -20},
		{ {47, op_minus}, -20},
		{ {47, close_paren}, -20},
		{ {47, semicolon}, -20},
		{ {48, op_plus}, -21},
		{ {48, op_minus}, -21},
		{ {48, close_paren}, -21},
		{ {48, semicolon}, -21},
		{ {49, close_paren}, 61},
		{ {50, type_int}, 62},
		{ {51, op_plus}, -23},
		{ {51, op_minus}, -23},
		{ {51, op_mult}, -23},
		{ {51, op_div}, -23},
		{ {51, op_idiv}, -23},
		{ {51, op_mod}, -23},
		{ {51, op_pow}, -23},
		{ {51, close_paren}, -23},
		{ {51, semicolon}, -23},
		{ {52, open_paren}, 33},
		{ {52, identifier}, 34},
		{ {52, type_string}, 35},
		{ {52, type_double}, 36},
		{ {52, type_int}, 37},
		{ {52, type_boolean}, 38},
		{ {53, open_paren}, 33},
		{ {53, identifier}, 34},
		{ {53, type_string}, 35},
		{ {53, type_double}, 36},
		{ {53, type_int}, 37},
		{ {53, type_boolean}, 38},
		{ {54, open_paren}, 33},
		{ {54, identifier}, 34},
		{ {54, type_string}, 35},
		{ {54, type_double}, 36},
		{ {54, type_int}, 37},
		{ {54, type_boolean}, 38},
		{ {55, open_paren}, 33},
		{ {55, identifier}, 34},
		{ {55, type_string}, 35},
		{ {55, type_double}, 36},
		{ {55, type_int}, 37},
		{ {55, type_boolean}, 38},
		{ {56, open_paren}, 33},
		{ {56, identifier}, 34},
		{ {56, type_string}, 35},
		{ {56, type_double}, 36},
		{ {56, type_int}, 37},
		{ {56, type_boolean}, 38},
		{ {57, open_paren}, 33},
		{ {57, identifier}, 34},
		{ {57, type_string}, 35},
		{ {57, type_double}, 36},
		{ {57, type_int}, 37},
		{ {57, type_boolean}, 38},
		{ {58, open_paren}, 33},
		{ {58, identifier}, 34},
		{ {58, type_string}, 35},
		{ {58, type_double}, 36},
		{ {58, type_int}, 37},
		{ {58, type_boolean}, 38},
		{ {59, close_paren}, 70},
		{ {60, semicolon}, -12},
		{ {61, semicolon}, -11},
		{ {62, comma}, 71},
		{ {63, close_paren}, -14},
		{ {63, semicolon}, -14},
		{ {64, close_paren}, -15},
		{ {64, semicolon}, -15},
		{ {65, op_plus}, -17},
		{ {65, op_minus}, -17},
		{ {65, close_paren}, -17},
		{ {65, semicolon}, -17},
		{ {66, op_plus}, -18},
		{ {66, op_minus}, -18},
		{ {66, close_paren}, -18},
		{ {66, semicolon}, -18},
		{ {67, op_plus}, -19},
		{ {67, op_minus}, -19},
		{ {67, close_paren}, -19},
		{ {67, semicolon}, -19},
		{ {68, op_plus}, -20},
		{ {68, op_minus}, -20},
		{ {68, close_paren}, -20},
		{ {68, semicolon}, -20},
		{ {69, op_plus}, -21},
		{ {69, op_minus}, -21},
		{ {69, close_paren}, -21},
		{ {69, semicolon}, -21},
		{ {70, op_plus}, -23},
		{ {70, op_minus}, -23},
		{ {70, op_mult}, -23},
		{ {70, op_div}, -23},
		{ {70, op_idiv}, -23},
		{ {70, op_mod}, -23},
		{ {70, op_pow}, -23},
		{ {70, close_paren}, -23},
		{ {70, semicolon}, -23},
		{ {71, type_int}, 72},
		{ {72, open_curly}, 73},
		{ {73, kw_print}, 12},
		{ {73, kw_for}, 13},
		{ {73, open_paren}, 14},
		{ {73, identifier}, 15},
		{ {73, type_string}, 16},
		{ {73, type_double}, 17},
		{ {73, type_int}, 18},
		{ {73, type_boolean}, 19},
		{ {74, close_curly}, 77},
		{ {75, close_curly}, -2},
		{ {75, eof}, -2},
		{ {76, semicolon}, 78},
		{ {77, semicolon}, -13},
		{ {78, kw_print}, 12},
		{ {78, kw_for}, 13},
		{ {78, open_paren}, 14},
		{ {78, close_curly}, -4},
		{ {78, identifier}, 15},
		{ {78, type_string}, 16},
		{ {78, type_double}, 17},
		{ {78, type_int}, 18},
		{ {78, type_boolean}, 19},
		{ {78, eof}, -4},
		{ {79, close_curly}, -3},
		{ {79, eof}, -3},
};
static map<pair<int, token>, int> tgoto = {
		{ {0, StatementSuite}, 1},
		{ {0, StatementList}, 2},
		{ {0, Statement}, 3},
		{ {0, SimpleStatement}, 4},
		{ {0, ComplexStatement}, 5},
		{ {0, Print}, 6},
		{ {0, Assignment}, 7},
		{ {0, ForLoop}, 8},
		{ {0, Expression}, 9},
		{ {0, MultiplicativeExpression}, 10},
		{ {0, BaseExpression}, 11},
		{ {14, Expression}, 30},
		{ {14, MultiplicativeExpression}, 31},
		{ {14, BaseExpression}, 32},
		{ {20, StatementList}, 40},
		{ {20, Statement}, 3},
		{ {20, SimpleStatement}, 4},
		{ {20, ComplexStatement}, 5},
		{ {20, Print}, 6},
		{ {20, Assignment}, 7},
		{ {20, ForLoop}, 8},
		{ {20, Expression}, 9},
		{ {20, MultiplicativeExpression}, 10},
		{ {20, BaseExpression}, 11},
		{ {21, Expression}, 41},
		{ {21, MultiplicativeExpression}, 10},
		{ {21, BaseExpression}, 11},
		{ {22, Expression}, 43},
		{ {22, MultiplicativeExpression}, 10},
		{ {22, BaseExpression}, 11},
		{ {23, MultiplicativeExpression}, 44},
		{ {23, BaseExpression}, 11},
		{ {24, MultiplicativeExpression}, 45},
		{ {24, BaseExpression}, 11},
		{ {25, MultiplicativeExpression}, 46},
		{ {25, BaseExpression}, 11},
		{ {26, MultiplicativeExpression}, 47},
		{ {26, BaseExpression}, 11},
		{ {27, MultiplicativeExpression}, 48},
		{ {27, BaseExpression}, 11},
		{ {28, Expression}, 49},
		{ {28, MultiplicativeExpression}, 31},
		{ {28, BaseExpression}, 32},
		{ {33, Expression}, 59},
		{ {33, MultiplicativeExpression}, 31},
		{ {33, BaseExpression}, 32},
		{ {39, Expression}, 60},
		{ {39, MultiplicativeExpression}, 10},
		{ {39, BaseExpression}, 11},
		{ {52, Expression}, 63},
		{ {52, MultiplicativeExpression}, 31},
		{ {52, BaseExpression}, 32},
		{ {53, Expression}, 64},
		{ {53, MultiplicativeExpression}, 31},
		{ {53, BaseExpression}, 32},
		{ {54, MultiplicativeExpression}, 65},
		{ {54, BaseExpression}, 32},
		{ {55, MultiplicativeExpression}, 66},
		{ {55, BaseExpression}, 32},
		{ {56, MultiplicativeExpression}, 67},
		{ {56, BaseExpression}, 32},
		{ {57, MultiplicativeExpression}, 68},
		{ {57, BaseExpression}, 32},
		{ {58, MultiplicativeExpression}, 69},
		{ {58, BaseExpression}, 32},
		{ {73, StatementSuite}, 74},
		{ {73, StatementList}, 75},
		{ {73, Statement}, 76},
		{ {73, SimpleStatement}, 4},
		{ {73, ComplexStatement}, 5},
		{ {73, Print}, 6},
		{ {73, Assignment}, 7},
		{ {73, ForLoop}, 8},
		{ {73, Expression}, 9},
		{ {73, MultiplicativeExpression}, 10},
		{ {73, BaseExpression}, 11},
		{ {78, StatementList}, 79},
		{ {78, Statement}, 76},
		{ {78, SimpleStatement}, 4},
		{ {78, ComplexStatement}, 5},
		{ {78, Print}, 6},
		{ {78, Assignment}, 7},
		{ {78, ForLoop}, 8},
		{ {78, Expression}, 9},
		{ {78, MultiplicativeExpression}, 10},
		{ {78, BaseExpression}, 11},
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
				auto ForLoop_1 = value_stack.top();
				value_stack.pop();
				auto* node = new ComplexStatement3 {ForLoop_1};
				value_stack.push(node);
				break;
			}
			case 11: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Print1 {Expression_1};
				value_stack.push(node);
				break;
			}
			case 12: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto identifier_2 = value_stack.top();
				value_stack.pop();
				auto* node = new Assignment1 {identifier_2, Expression_1};
				value_stack.push(node);
				break;
			}
			case 13: {
				auto StatementSuite_1 = value_stack.top();
				value_stack.pop();
				auto type_int_2 = value_stack.top();
				value_stack.pop();
				auto type_int_3 = value_stack.top();
				value_stack.pop();
				auto identifier_4 = value_stack.top();
				value_stack.pop();
				auto* node = new ForLoop1 {identifier_4, type_int_3, type_int_2, StatementSuite_1};
				value_stack.push(node);
				break;
			}
			case 14: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto MultiplicativeExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new Expression1 {MultiplicativeExpression_2, Expression_1};
				value_stack.push(node);
				break;
			}
			case 15: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto MultiplicativeExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new Expression2 {MultiplicativeExpression_2, Expression_1};
				value_stack.push(node);
				break;
			}
			case 16: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new Expression3 {MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 17: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression1 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 18: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression2 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 19: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression3 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 20: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression4 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 21: {
				auto MultiplicativeExpression_1 = value_stack.top();
				value_stack.pop();
				auto BaseExpression_2 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression5 {BaseExpression_2, MultiplicativeExpression_1};
				value_stack.push(node);
				break;
			}
			case 22: {
				auto BaseExpression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new MultiplicativeExpression6 {BaseExpression_1};
				value_stack.push(node);
				break;
			}
			case 23: {
				auto Expression_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression1 {Expression_1};
				value_stack.push(node);
				break;
			}
			case 24: {
				auto identifier_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression2 {identifier_1};
				value_stack.push(node);
				break;
			}
			case 25: {
				auto type_double_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression3 {type_double_1};
				value_stack.push(node);
				break;
			}
			case 26: {
				auto type_int_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression4 {type_int_1};
				value_stack.push(node);
				break;
			}
			case 27: {
				auto type_string_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression5 {type_string_1};
				value_stack.push(node);
				break;
			}
			case 28: {
				auto type_boolean_1 = value_stack.top();
				value_stack.pop();
				auto* node = new BaseExpression6 {type_boolean_1};
				value_stack.push(node);
				break;
			}
			default:
				exit(1);
		}
	}

}
#endif