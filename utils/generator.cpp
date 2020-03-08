#include "generator.h"

generator::generator() {
    int index = 1;
    for (const auto &p : grammar::grammar) {
        for (const auto &production : p.second) {
            grammar_indexed[{p.first, production}] = index++;
        }
    }
}

void generator::generate() {
    gfirst();
    gfollow();
    gitems();
    gtable_action();
    gtable_goto();
    gtable_files();
}

void generator::gfirst() {
    for (const token &t : terminals) first[t] = {t};
    for (const token &t : nonterminals) first[t] = {};

    while (true) {
        map<token, set<token>> previous(first);
        for (const auto &p : grammar::grammar) {
            token A = p.first;
            vector<vector<token>> productions = p.second;
            for (const auto &X : productions) {
                first[A].insert(first[X[0]].begin(), first[X[0]].end());
            }
        }
        if (previous == first) break;
    }
}

void generator::gfollow() {
    for (const token &t : nonterminals) follow[t] = {};
    follow[start_symbol].insert(token::eof);

    while (true) {
        map<token, set<token>> previous = follow;
        for (const auto &p : grammar::grammar) {
            token A = p.first;
            vector<vector<token>> productions = p.second;
            for (const auto &X : productions) {
                int n = static_cast<int>(X.size());
                for (int i = 0; i < n; i++) {
                    token Xi = X[i];
                    if (find(nonterminals.begin(), nonterminals.end(), Xi) != nonterminals.end() && i < n - 1) {
                        follow[Xi].insert(first[X[i + 1]].begin(), first[X[i + 1]].end());
                    } else if (find(nonterminals.begin(), nonterminals.end(), Xi) != nonterminals.end() &&
                               X[X.size() - 1] == Xi) {
                        follow[Xi].insert(follow[A].begin(), follow[A].end());
                    }
                }
            }
        }
        if (previous == follow) break;
    }
}

generator::CustomItemSet generator::gclosure(CustomItemSet I) {
    CustomItemSet original(I);

    while (true) {
        CustomItemSet previous(I);
        CustomItemSet temporary(I);

        for (const auto &i : temporary) {
            token X = i.gX();
            token b = i.gBeta();
            token z = i.gZ();

            if (find(nonterminals.begin(), nonterminals.end(), X) != nonterminals.end()) {
                for (const auto &production : grammar::grammar[X]) {
                    token seek_token = (b != token::epsilon) ? b : z;
                    for (const auto &w : first[seek_token]) {
                        if (find(terminals.begin(), terminals.end(), w) != terminals.end()) {
                            I.insert(item(X, production, 0, w));
                        }
                    }
                }
            }
        }

        if (previous == I)return I;
    }
}

generator::CustomItemSet generator::ggoto(CustomItemSet I, token X) {
    CustomItemSet J = {};

    for (const auto &i : I) {
        if (i.gLoc() < i.gSize() && i.gX() == X) {
            J.insert(item(i.gA(), i.gTokens(), i.gLoc() + 1, i.gZ()));
        }
    }
    return gclosure(J);
}

void generator::gitems() {
    CustomItemSet init_item(item(token::Mango, {token::StatementSuite}, 0, token::eof));
    c = {gclosure(init_item)};

    bool added = true;
    while (added) {
        added = false;
        vector<CustomItemSet> temporary(c);
        for (const auto &I : temporary) {
            for (const auto &X : token()) {
                CustomItemSet goto_result = ggoto(I, X);
                if (find(c.begin(), c.end(), goto_result) == c.end() && !goto_result.empty()) {
                    added = true;
                    c.push_back(goto_result);
                }
            }
        }
    }
}

void generator::gtable_action() {
    int i = 0;
    for (const auto &Ii : c) {
        for (const auto &it : Ii) {
            token a = it.gX();
            if (it.isAtEnd() && it.gA() == start_symbol) {
                taction[{i, token::eof}] = 0;
            }
            if (it.isAtEnd() && it.gA() != start_symbol) {
                token A = it.gA();
                int j = grammar_indexed[{A, it.gTokens()}];
                for (const auto &X : follow[A]) {
                    taction[{i, X}] = -j;
                }
            }
            if (it.isNotAtEnd() && find(terminals.begin(), terminals.end(), a) != terminals.end()) {
                CustomItemSet goto_result = ggoto(Ii, a);
                if (find(c.begin(), c.end(), goto_result) != c.end()) {
                    auto iter = find(c.begin(), c.end(), goto_result);
                    int j = distance(c.begin(), iter);
                    taction[{i, a}] = j;
                }
            }
        }
        i++;
    }
}

void generator::gtable_goto() {
    int i = 0;
    for (const auto &Ii : c) {
        for (const auto &A : nonterminals) {
            CustomItemSet goto_result = ggoto(Ii, A);
            if (find(c.begin(), c.end(), goto_result) != c.end()) {
                auto iter = find(c.begin(), c.end(), goto_result);
                int j = distance(c.begin(), iter);
                tgoto[{i, A}] = j;
            }
        }
        i++;
    }
}

void generator::gtable_files() {
    ofstream ofile("../parsing/parse_table.h");
    ofile.clear();

    ostringstream ss;

    ss << "#ifndef MANGOREVISITEDCPPCLION_PARSE_TABLE_H\n"
          "#define MANGOREVISITEDCPPCLION_PARSE_TABLE_H\n"
          "\n"
          "#include \"map\"\n"
          "#include \"stack\"\n"
          "#include \"../core/grammar.h\"\n"
          "#include \"../tree/tree.h\"\n"
          "\n"
          "using std::map;\n"
          "using std::pair;\n"
          "using std::stack;\n\n"
          "using namespace grammar;\n"
          "\n";

    ss << "static map<pair<int, token>, int> taction = {\n";
    for (const auto &p : taction) {
        ss << "\t{ {" << p.first.first << ", token::" << token_map[p.first.second] << "}, " << p.second
           << "},\n";
    }
    ss << "};\n";

    ss << "static map<pair<int, token>, int> tgoto = {\n";

    for (const auto &p : tgoto) {
        ss << "\t{ {" << p.first.first << ", token::" << token_map[p.first.second] << "}, " << p.second
           << "},\n";
    }

    ss << "};\n\n";

    ss << "static void reduce(int decision, stack<Node*> *value_stack) {\n"
          "\tswitch(decision) {\n";
    int gindex = 1;
    vector<string> node_names = {};
    map<string, vector<string>> node_pointer_map = {};
    for (const auto &p : grammar::grammar) {
        token key = p.first;
        vector<vector<token>> productions = p.second;

        int i = 0;
        for (const auto &production : productions) {
            ss << "\t\tcase " << gindex << ": {\n";
            gindex++;
            i++;
            std::vector<string> tempvars = {};
            vector<token> production_reversed = production;
            std::reverse(production_reversed.begin(), production_reversed.end());
            for (const auto &sym : production_reversed) {
                if (find(nonterminals.begin(), nonterminals.end(), sym) != nonterminals.end() ||
                    sym == token::type_string || sym == token::type_int || sym == token::type_double ||
                    sym == token::identifier) {
                    tempvars.emplace_back(grammar::token_map[sym]);
                    ss << "\t\t\t" << "auto* " << grammar::token_map[sym] << " = value_stack->top();\n";
                    ss << "\t\t\t" << "value_stack->pop();\n";
                }
            }
            ss << "\t\t\t" << "auto* node = new " << grammar::token_map[key] << i << " {";
            std::reverse(tempvars.begin(), tempvars.end());
            node_pointer_map[grammar::token_map[key] + std::to_string(i)] = tempvars;
            for (int q = 0; q < (int) tempvars.size(); q++) {
                if (q == tempvars.size() - 1) {
                    ss << tempvars[q];
                } else {
                    ss << tempvars[q] << ", ";
                }
            }
            ss << "};\n";
            ss << "\t\t\tvalue_stack->push(node);\n";
            ss << "\t\t\tbreak;\n";
            ss << "\t\t}\n";

            ostringstream ss2;
            ss2 << grammar::token_map[key] << i;
            node_names.emplace_back(ss2.str());
        }
    }
    ss << "\t\tdefault:\n";
    ss << "\t\t\texit(1);\n";
    ss << "\t}\n";
    ss << "}\n\n";
    ss << "#endif";

    ofile << ss.str();
    ofile.close();

    gtable_tree(node_pointer_map);
    gtable_interpreter(node_pointer_map);
}

void generator::gtable_tree(const map<string, vector<string>> &node_pointer_map) {
    ofstream ofile("../tree/tree.h");
    ofile.clear();

    ostringstream ss;

    ss << "#ifndef MANGOREVISITEDCPPCLION_TREE_H\n"
          "#define MANGOREVISITEDCPPCLION_TREE_H\n"
          "\n"
          "#include \"string\"\n"
          "\n"
          "using std::string;\n\n"
          "class Identifier;\n"
          "class StringLiteral;\n"
          "class IntegerLiteral;\n"
          "class DoubleLiteral;\n";


    for (const auto &p : node_pointer_map) {
        ss << "class " << p.first << ";\n";
    }

    ss << "\nclass Visitor {\n"
          "public:\n"
          "\tvirtual void visit(Identifier *n) { return; };\n"
          "\tvirtual void visit(StringLiteral *n) { return; };\n"
          "\tvirtual void visit(DoubleLiteral *n) { return; };\n"
          "\tvirtual void visit(IntegerLiteral *n) { return; };\n";

    for (const auto &p : node_pointer_map) {
        ss << "\tvirtual void visit(" << p.first << "* n) { return; };" << "\n";
    }

    ss << "};\n\n";

    ss << "class Node {\n"
          "public:\n"
          "    virtual void accept(Visitor *v) = 0;\n"
          "};\n\n"
          "class Identifier : public Node {\n"
          "public:\n"
          "    string f0;\n"
          "\n"
          "    explicit Identifier(string n0) {\n"
          "        f0 = std::move(n0);\n"
          "    }\n"
          "\n"
          "    void accept(Visitor *v) override {\n"
          "        v->visit(this);\n"
          "    }\n"
          "};\n"
          "class StringLiteral : public Node {\n"
          "public:\n"
          "    string f0;\n"
          "\n"
          "    explicit StringLiteral(string n0) {\n"
          "        f0 = std::move(n0);\n"
          "    }\n"
          "\n"
          "    void accept(Visitor *v) override {\n"
          "        v->visit(this);\n"
          "    }\n"
          "};\n"
          "\n"
          "class DoubleLiteral : public Node {\n"
          "public:\n"
          "    double f0;\n"
          "\n"
          "    explicit DoubleLiteral(string n0) {\n"
          "        f0 = std::stod(n0);\n"
          "    }\n"
          "\n"
          "    void accept(Visitor *v) override {\n"
          "        v->visit(this);\n"
          "    }\n"
          "};\n"
          "\n"
          "class IntegerLiteral : public Node {\n"
          "public:\n"
          "    int f0;\n"
          "\n"
          "    explicit IntegerLiteral(string n0) {\n"
          "        f0 = std::stoi(n0);\n"
          "    }\n"
          "\n"
          "    void accept(Visitor *v) override {\n"
          "        v->visit(this);\n"
          "    }\n"
          "};\n\n";


    for (const auto &p : node_pointer_map) {
        ss << "class " << p.first << " : public Node {\n";
        ss << "public:\n";

        for (int i = 1; i < p.second.size() + 1; i++) {
            ss << "\tNode* n" << i << ";\n";
        }

        ss << "\texplicit " << p.first << "(";
        for (int i = 1; i < p.second.size() + 1; i++) {
            if (i == p.second.size()) {
                ss << "Node* a" << i;
            } else {
                ss << "Node* a" << i << ", ";
            }
        }
        ss << ") {\n";
        for (int i = 1; i < p.second.size() + 1; i++) {
            ss << "\t\tn" << i << " = a" << i << ";\n";
        }
        ss << "\t}\n";

        ss << "\tvoid accept(Visitor *v) override {\n"
              "\t\tv->visit(this);\n"
              "\t}\n";
        ss << "};\n\n";
    }

    ss << "#endif //MANGOREVISITEDCPPCLION_TREE_H";

    ofile << ss.str();
    ofile.close();
}

void generator::gtable_interpreter(const map<string, vector<string>> &node_pointer_map) {
    ofstream ofile("../interpreter/interpreter.h");
    ofile.clear();

    ostringstream ss;

    ss << "#ifndef MANGOREVISITEDCPPCLION_INTERPRETER_H\n"
          "#define MANGOREVISITEDCPPCLION_INTERPRETER_H\n"
          "\n"
          "#include \"iostream\"\n"
          "#include \"cmath\"\n\n"
          "#include \"../tree/tree.h\"\n"
          "#include \"state.h\"\n\n"
          "using std::cout;\n"
          "using std::endl;\n\n";

    ss << "class Interpreter : public Visitor {\n"
          "public:\n"
          "\tvoid visit(Identifier *n) override;\n"
          "\tvoid visit(StringLiteral *n) override;\n"
          "\tvoid visit(DoubleLiteral *n) override;\n"
          "\tvoid visit(IntegerLiteral *n) override;\n";

    for (const auto &p : node_pointer_map) {
        ss << "\tvoid visit(" << p.first << "* n) override;\n";
    }

    ss << "private:\n";
    ss << "\tstate current_state;\n"
          "\tstatic string get_type_debug(const variant<int, string, double>&);\n";

    ss << "};\n\n";

    ss << "#endif //MANGOREVISITEDCPPCLION_INTERPRETER_H";

    ofile << ss.str();
    ofile.close();
}
