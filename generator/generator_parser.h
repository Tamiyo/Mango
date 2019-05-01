//
// Created by Matt on 4/25/2019.
//

#ifndef MANGO_CL_GENERATOR_PARSER_H
#define MANGO_CL_GENERATOR_PARSER_H

#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <stack>
#include <algorithm>
#include "../tokens/tokens.h"

using namespace std;

class generator_parser {
public:
    generator_parser();

    virtual ~generator_parser();

    void gen_parse_table();

private:
    struct item {
        string A = "";
        string a = "";
        string B = "";
        string b = "";
        string t = "";

        string toStringSimple() {
            return "[ " + A + " -> " + a + " * " + B + " ; " + b + ", " + t + "]";
        }

        string toString() {
            return "[ " + A + " -> " + a + " * " + B + " ; " + b + ", " + t + "]\n" +
                   "\tA: " + A + "\n" +
                   "\ta: " + a + "\n" +
                   "\tB: " + B + "\n" +
                   "\tb: " + b + "\n" +
                   "\tt: " + t + "";
        }

        inline bool operator==(const item &rhs) {
            return this->A == rhs.A &&
                   this->a == rhs.a &&
                   this->B == rhs.B &&
                   this->b == rhs.b &&
                   this->t == rhs.t;
        }
    };


    vector<string> non_terminals;
    vector<string> terminals;
    vector<string> grammar_symbols;

    map<string, vector<string>> first;
    map<string, vector<string>> follow;

    map<string, vector<string>> grammar;
    vector<vector<generator_parser::item *>> C;

    tokens *keys;

    vector<string> split_string(string);

    void FIRST();

    void FOLLOW();

    void CHECK_WARNINGS();

    vector<generator_parser::item *> CLOSURE(const vector<generator_parser::item *> &);

    vector<generator_parser::item *> GOTO(const vector<generator_parser::item *>&, const string&);

    void ITEMS();
};


#endif //MANGO&_CL_GENERATOR&_PARSER_H
