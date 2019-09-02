#ifndef MANGO_V2_CPP_SEMANTIC_ANALYZER_H
#define MANGO_V2_CPP_SEMANTIC_ANALYZER_H

#include <map>
#include <vector>
#include <string>
#include <stdlib.h>

#include "core.h"
#include "ast.h"

using std::map;
using std::vector;
using std::strtod;
using std::string;
using std::to_string;

map<PrimitiveType, vector<PrimitiveType>> primitive_type_conversions = {
        {PrimitiveType::Float,    {PrimitiveType::Integer, PrimitiveType::String}},
        {PrimitiveType::Integer,  {PrimitiveType::Float,   PrimitiveType::String}},
        {PrimitiveType::String,   {PrimitiveType::Float,   PrimitiveType::Integer}},
        {PrimitiveType::Boolean,  {}},
        {PrimitiveType::Object,   {}},
        {PrimitiveType::Function, {}},
};

string convert(const string &a, const string &b, PrimitiveType typeA, PrimitiveType typeB) {
    if (typeA == PrimitiveType::String) {
        return a;
    } else if (typeA == PrimitiveType::Integer && typeB == PrimitiveType::Float) {
        return b.substr(0, a.find('.'));
    } else if (typeA == PrimitiveType::Float && typeB == PrimitiveType::Integer) {
        return b + ".0";
    }
}


string floatArithmetic(const string &a, const string &b, const string &op) {
    float float_a = strtod(a.c_str(), nullptr);
    float float_b = strtod(b.c_str(), nullptr);

    if (op == "+") {
        return to_string(float_a + float_b);
    } else if (op == "-") {
        return to_string(float_a - float_b);
    } else if (op == "*") {
        return to_string(float_a * float_b);
    } else if (op == "/") {
        return to_string(float_a / float_b);
    } else if (op == "%") {
        return "";
    }
}

string integerArithmetic(const string &a, const string &b, const string &op) {
    int integer_a = strtol(a.c_str(), nullptr, 10);
    int integer_b = strtol(b.c_str(), nullptr, 10);

    if (op == "+") {
        return to_string(integer_a + integer_b);
    } else if (op == "-") {
        return to_string(integer_a - integer_b);
    } else if (op == "*") {
        return to_string(integer_a * integer_b);
    } else if (op == "/") {
        return to_string(integer_a / integer_b);
    } else if (op == "%") {
        return to_string(integer_a % integer_b);
    }
}

string stringArithmetic(const string &a, const string &b, const string &op) {
    if (op == "+") {
        return a + b;
    } else if (op == "-") {
        return "";
    } else if (op == "*") {
        return "";
    } else if (op == "/") {
        return "";
    } else if (op == "%") {
        return "";
    }
}

string doArithmetic(const string &a, const string &b, PrimitiveType type, const string &op) {
    if (type == PrimitiveType::Float) {
        return floatArithmetic(a, b, op);
    } else if (type == PrimitiveType::Integer) {
        return integerArithmetic(a, b, op);
    } else if (type == PrimitiveType::String) {
        return stringArithmetic(a, b, op);
    }
}

#endif //MANGO_V2_CPP_SEMANTIC_ANALYZER_H
