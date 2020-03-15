#ifndef MANGOREVISITEDCPPCLION_GENERATOR_H
#define MANGOREVISITEDCPPCLION_GENERATOR_H

#include <utility>
#include <iostream>

#include <fstream>
#include <algorithm>
#include <vector>
#include <set>
#include <string>
#include <sstream>
#include <iterator>
#include <map>

#include "grammar.h"

using namespace std;

namespace mango {
    class generator {
    public:
        generator();

        void generate();

    public:
        class item {
        public:
            item(token A, vector<token> tokens, int loc, token z) {
                this->A = A;
                this->tokens = move(tokens);
                this->loc = loc;
                this->z = z;
            }

            inline bool operator==(const item &i2) const {
                return A == i2.A &&
                       tokens == i2.tokens &&
                       loc == i2.loc &&
                       z == i2.z;
            }


            friend ostream &operator<<(ostream &os, const item &it) {
                os << token_map[it.A] << " -> ";
                int i = 0;
                for (const auto &tok : it.tokens) {
                    if (i == it.loc) {
                        os << ". ";
                    }
                    os << token_map[tok] << " ";
                    i++;
                }
                os << "| " << token_map[it.z];
                return os;
            }

            [[nodiscard]] bool isNotAtEnd() const {
                return loc < tokens.size();
            }

            [[nodiscard]] bool isAtEnd() const {
                return loc == tokens.size();
            }

            [[nodiscard]] int gLoc() const {
                return this->loc;
            }

            [[nodiscard]] int gSize() const {
                return this->tokens.size();
            }

            [[nodiscard]] token gA() const {
                return this->A;
            }

            token gAlpha() {
                if (loc < static_cast<int>(tokens.size()) && loc > 0) {
                    return tokens[loc - 1];
                }
                return epsilon;
            }

            [[nodiscard]] token gX() const {
                if (loc >= 0 && loc < static_cast<int>(tokens.size())) {
                    return tokens[loc];
                }
                return epsilon;
            }

            [[nodiscard]] token gBeta() const {
                if (loc + 1 < static_cast<int>(tokens.size())) {
                    return tokens[loc + 1];
                }
                return epsilon;
            }

            [[nodiscard]] token gZ() const {
                return this->z;
            }

            [[nodiscard]] vector<token> gTokens() const {
                return this->tokens;
            }

        private:
            token A;
            vector<token> tokens;
            int loc;
            token z;
        };

        class CustomItemSet {
        public:
            CustomItemSet() {
                items = {};
            }

            explicit CustomItemSet(item it) {
                items = {move(it)};
            }

            CustomItemSet(CustomItemSet const &c) {
                items = c.items;
            }

            friend ostream &operator<<(ostream &os, const CustomItemSet &itemSet) {
                for (const auto &it : itemSet.items) {
                    os << it << "\n";
                }
                return os;
            }

            inline item &operator[](int i) {
                return items[i];
            }

            [[nodiscard]] inline auto begin() const {
                return items.begin();
            }

            [[nodiscard]] inline auto end() const {
                return items.end();
            }

            bool empty() {
                return items.empty();
            }

            void insert(const item &i) {
                if (find(items.begin(), items.end(), i) == items.end()) {
                    items.push_back(i);
                }
            }

            inline bool operator==(const CustomItemSet &cis2) const {
                return items == cis2.items;
            }

            vector<item> items;
        };

        void gfirst();

        void gfollow();

        CustomItemSet gclosure(CustomItemSet);

        CustomItemSet ggoto(CustomItemSet, token);

        void gitems();

        void gtable_action();

        void gtable_goto();

        void gtable_files();

        void gtable_tree(const map<string, vector<string>> &);

        void gtable_interpreter(const map<string, vector<string>> &);

        map<pair<token, vector<token>>, int> grammar_indexed;

        map<token, set<token>> first;
        map<token, set<token>> follow;
        vector<CustomItemSet> c;
        map<pair<int, token>, int> taction;
        map<pair<int, token>, int> tgoto;

        map<CustomItemSet, CustomItemSet> closure_cache;
        map<pair<CustomItemSet, token>, CustomItemSet> goto_cache;
    };
}


#endif //MANGOREVISITEDCPPCLION_GENERATOR_H
