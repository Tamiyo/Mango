#include <utility>

#include <utility>

#include "generator_parser.h"

generator_parser::generator_parser() {
    keys = new tokens();
    non_terminals = vector<string>();
    terminals = vector<string>();
    first = map<string, vector<string>>();
    follow = map<string, vector<string>>();
    grammar = map<string, vector<string>>();
    C = vector<vector<generator_parser::item *>>();
}

generator_parser::~generator_parser() = default;

void generator_parser::gen_parse_table() {

    /// Setup Pre-requsisite Variables
    string();
    terminals.emplace_back("$");

    ifstream grammarFile;
    grammarFile.open(R"(D:\Documents\mango_cl\generator\grammar.mg)");

    /// Generate Prereq grammar from File
    int rule_no = 1;
    string r;
    if (grammarFile.is_open()) {
        while (getline(grammarFile, r)) {
            if (r.length() > 2) {
                string lhs = r.substr(0, r.find("->") - 1);
                string rhs = r.substr(r.find("->") + 3);
                grammar[lhs].push_back(rhs);

                cout << lhs << " -> " << rhs << " = " << rule_no << endl;

                if (find(non_terminals.begin(), non_terminals.end(), lhs) == non_terminals.end()) {
                    non_terminals.push_back(lhs);
                    grammar_symbols.push_back(lhs);
                    //cout << "NTS: " << lhs << endl;;
                }

                vector<string> spl_rhs = split_string(rhs);
                for (auto &spl_rh : spl_rhs) {
                    if (find(terminals.begin(), terminals.end(), spl_rh) == terminals.end() &&
                        spl_rh.substr(0, 2) == "TS" && spl_rh != "TS_EMPTY") {
                        terminals.push_back(spl_rh);
                        grammar_symbols.push_back(spl_rh);
                        //cout << "TS: " << spl_rhs.at(i) << endl;
                    }
                }

                rule_no++;
            }
        }
        grammarFile.close();
    } else {
        cout << "Unable to open file" << endl;
    }


    /// Generate FIRST Transitions
    FIRST();

    /// Generate FOLLOW Transitions
    FOLLOW();

    /// Check for FIRST/FIRST and FIRST/FOLLOW Warnings
    CHECK_WARNINGS();

    /// Generate Itemsets
    ITEMS();

}

vector<string> generator_parser::split_string(string str) {
    vector<string> out;

    while (str.find(' ') != -1) {
        out.push_back(str.substr(0, str.find(' ')));
        str = str.substr(str.find(' ') + 1);
    }
    out.push_back(str);
    return out;
}

void generator_parser::FIRST() {
    map<string, vector<string>> augmented_grammar = grammar;

    /// Generate FIRST() transitions
    cout << endl
         << "Starting FIRST()" << endl;

    auto it = augmented_grammar.begin();
    while (it != augmented_grammar.end()) {
        vector<string> rhs = it->second;
        while (!rhs.empty()) {
            string first_production = rhs.back().substr(0, rhs.back().find(' '));

            rhs.pop_back();
            if (first_production.substr(0, 2) == "TS") {
                first[it->first].push_back(first_production);
            } else {
                for (const auto &additions : augmented_grammar[first_production]) {
                    rhs.push_back(additions);
                }
            }
        }
        it++;
    }

    /// Print out FIRST for debugging
    for (const auto &NTS_F : first) {
        cout << "FIRST(" << NTS_F.first << ") = { ";
        for (const auto &p : NTS_F.second) {
            cout << p << ", ";
        }
        cout << "}" << endl;
    }
}

void generator_parser::FOLLOW() {
    map<string, vector<string>> augmented_grammar = grammar;

    /// Generate FOLLOW() transitions
    cout << endl
         << endl
         << "Starting FOLLOW()" << endl;

    bool has_changed = true;

    follow[non_terminals.at(0)].push_back("TS_EOF");

    while (has_changed) {
        has_changed = false;
        for (const auto &non_terminal : non_terminals) {
            for (const auto &rule : augmented_grammar) {
                for (const auto &procedure : rule.second) {
                    vector<string> tokens = split_string(procedure);
                    auto t_it = tokens.begin();

                    while (t_it != tokens.end()) {
                        // If the current string is the non-terminal
                        if (*t_it == non_terminal) {
                            // If the next string is the end,
                            if ((t_it + 1) == tokens.end()) {
                                for (const auto &f : follow[rule.first]) {
                                    if (find(follow[non_terminal].begin(), follow[non_terminal].end(), f) ==
                                        follow[non_terminal].end()) {
                                        follow[non_terminal].push_back(f);
                                        has_changed = true;
                                    }
                                }
                            } else if ((*(t_it + 1)).substr(0, 2) == "NT") {
                                for (const auto &f : first[(*(t_it + 1))]) {
                                    if (find(follow[non_terminal].begin(), follow[non_terminal].end(), f) ==
                                        follow[non_terminal].end()) {
                                        follow[non_terminal].push_back(f);
                                        has_changed = true;
                                    }
                                }
                            } else if ((*(t_it + 1)).substr(0, 2) == "TS" &&
                                       find(follow[non_terminal].begin(), follow[non_terminal].end(),
                                            *(t_it + 1)) == follow[non_terminal].end()) {
                                follow[non_terminal].push_back((*(t_it + 1)));
                                has_changed = true;
                            }
                        }
                        t_it++;
                    }
                }
            }
        }
    }

    /// Print out FOLLOW()
    for (const auto &NTS_F : follow) {
        cout << "FOLLOW(" << NTS_F.first << ") = { ";
        for (const auto &p : NTS_F.second) {
            cout << p << ", ";
        }
        cout << "}" << endl;
    }
}

void generator_parser::CHECK_WARNINGS() {

/// Warnings for FIRST/FIRST conflicts
    cout << endl;
    for (const auto &rule : grammar) {
        vector<string> productions = rule.second;
        if (productions.size() > 1) {
            vector<string> v;
            for (const auto &prod : productions) {
                v.push_back(split_string(prod).at(0));
            }
            auto it = v.begin();
            while ((it + 1) != v.end()) {
                vector<string> item1 = first[*it];
                vector<string> item2 = first[*(it + 1)];

                for (const auto &pair1 : item1) {
                    for (const auto &pair2 : item2) {
                        if (pair1 == pair2) {
                            cout << "ERROR: " << *it << " & " << *(it + 1) << "\t\t\tFIRST/FIRST conflict"
                                 << endl;
                        }
                    }
                }
                it++;
            }
        }
    }

    /// Warnings for FIRST/FOLLOW conflicts
    cout << endl;
    for (const auto &rule : grammar) {
        vector<string> productions = rule.second;
        if (productions.size() > 1) {
            vector<string> v;
            for (const auto &prod : productions) {
                v.push_back(split_string(prod).at(0));
            }
            auto it = v.begin();
            while ((it + 1) != v.end()) {
                vector<string> prodvec = follow[rule.first];
                vector<string> item1 = first[*it];
                vector<string> item2 = first[*(it + 1)];

                for (const auto &pair1 : item1) {
                    if (pair1 == "TS_EMPTY") {
                        for (const auto &pair3 : prodvec) {
                            for (const auto &pair2 : item2) {
                                if (pair3 == pair2) {
                                    cout << "ERROR: " << *it << " & " << *(it + 1) << "\t\t\tFIRST/FOLLOW conflict"
                                         << endl;
                                }
                            }
                        }
                    }
                }
                it++;
            }
        }
    }
}


void generator_parser::ITEMS() {

    vector<vector<generator_parser::item *>> C;

    C.emplace_back(vector<generator_parser::item *>());
    C.at(0).emplace_back(new generator_parser::item{"NTS_SP", "", "NTS_S", "", "TS_EOF"});

//    itemset.at(0).emplace_back(new generator_parser::item{"NTS_MANGO", "", "NTS_STMTS", "", "TS_EOF"});
    vector<generator_parser::item *> temp = CLOSURE(C.at(0));
    for (auto t : temp) {
        C.at(0).push_back(t);
    }

    bool changed = true;
    int i = 0;
    while (changed) {
        changed = false;
        for (const auto &I : C) {
            for (const auto &X : grammar_symbols) {
                auto G = GOTO(I, X);
                cout << "Comparing" << endl;
                for (const auto &ti : G) {
                    cout << "\t" << ti->toStringSimple() << endl;
                }
                cout << "to" << endl;
                for (const auto &ti : I) {
                    cout << "\t" << ti->toStringSimple() << endl;
                }

                if (!G.empty() && deepVectorCheck(C, G)) {
                    C.emplace_back(G);
                }
            }
        }
    }

    i = 0;
    for (const auto &itemset : C) {
        cout << "\nitemset: " << i << ":" << endl;
        for (const auto &item : itemset) {
            cout << "\titem: " << item->toStringSimple() << endl;
        }
        i++;
    }

}

/// Computes the GOTO transition for a given itemset and a grammar symbol
vector<generator_parser::item *> generator_parser::GOTO(const vector<generator_parser::item *> &I, const string &X) {
    vector<generator_parser::item *> J;

    for (const auto &item : I) {
        if (item->B == X) {
            string _b;
            if (item->B.find(' ') == -1) { _b = ""; }
            else { _b = item->B.substr(item->B.find(' ') + 1); }
            generator_parser::item *tItem = new generator_parser::item{item->A, item->a + X,
                                                                       item->b.substr(0, item->b.find(' ')), _b,
                                                                       item->t};

            if(find(J.begin(), J.end(), tItem) == J.end()) {
                cout << "GOTO ADDED: " << tItem->toStringSimple() << ", " << X << endl;
                J.emplace_back(tItem);
            }
        }
    }

    return CLOSURE(J);
}

/// Computes the CLOSURE of a given itemset
vector<generator_parser::item *> generator_parser::CLOSURE(const vector<generator_parser::item *> &itemset) {
    map<string, vector<string>> augmented_grammar = grammar;

    vector<generator_parser::item *> out_itemset;

    stack<generator_parser::item *> stack;
    for (const auto &item : itemset) {
        out_itemset.push_back(item);
        stack.push(item);
    }

    while (!stack.empty()) {
        auto item = stack.top();
        stack.pop();
//        cout << "Closing:: " << item->toStringSimple() << endl;

        vector<string> first_prod;
        vector<string> rhs = augmented_grammar[item->b];

        /// Create the rule FIRST(B + t)
        for (auto &rh : rhs) {
            rh += " " + item->t;
//            cout << "\t" << rh;
        }

        if (rhs.empty()) {
            rhs.emplace_back("TS_EOF");
        } else {
            cout << endl;
        }

        /// Find the FIRST of each production in 'B"
        while (!rhs.empty()) {
            string first_production = rhs.back().substr(0, rhs.back().find(' '));
            rhs.pop_back();
            if (first_production.substr(0, 2) == "TS") {
                first_prod.push_back(first_production);
//                cout << "\tFirstProduction: " << first_production << endl;
            } else {
                for (const auto &additions : augmented_grammar[first_production]) {
                    rhs.push_back(additions);
                }
            }
        }

        for (const auto &production : augmented_grammar[item->B]) {
            for (const auto &terminal : first_prod) {
                string _b;
                if (production.find(' ') == -1) {
                    _b = "";
                } else {
                    _b = production.substr(production.find(' ') + 1);
                }
                auto testItem = new generator_parser::item{item->B, "", production.substr(0, production.find(' ')),
                                                           _b, terminal,};
//                cout << "\t Added to close: " << testItem->toStringSimple() << endl;
                out_itemset.push_back(testItem);
                stack.push(testItem);
            }
        }
    }
    return out_itemset;
}

bool generator_parser::deepVectorCheck(vector<vector<generator_parser::item *>> a, vector<generator_parser::item *> b) {

    for (const auto &subvector : a) {
        int check = 0;
        for (const auto &elem : subvector) {
            for (const auto &elem2 : b) {
                if (elem == elem2) {
                    check++;
                    break;
                }
            }
        }
        if (subvector.size() - 1 == check) {
            return true;
        }
    }
    return false;
}
