#include "lexer.h"

#include "iostream"

namespace mango {
    lexer::lexer() {
        ifstream ifs(R"(D:\Documents\MangoRevisitedCppCLion\example.mg)");
        string content((istreambuf_iterator<char>(ifs)),
                       (istreambuf_iterator<char>()));
        contents = content + "$";
    }

    void lexer::lex() {
        int index = 0;
        int maxSize = contents.size();

        while (index < maxSize) {
            int start_index = index;

            // Identifier
            if (isalpha(contents[index])) {
                index++;
                while (isalnum(contents[index]) && index < maxSize) index++;

                string slice = contents.substr(start_index, index - start_index);
                if (keyword_token_map.count(slice) > 0) {
                    token tok = keyword_token_map[slice];
                    tokens.push({tok, slice});
                } else {
                    token tok = identifier;
                    tokens.push({tok, slice});
                }
            }
                // Numeric [Int / Double]
            else if (isdigit(contents[index])) {
                bool is_decimal = false;

                while (index < maxSize) {
                    index++;
                    if (contents[index] == '.' && !is_decimal) is_decimal = true;
                    else if (contents[index] == '.' && is_decimal) break;
                    else if (!isdigit(contents[index])) break;
                }
                string slice = contents.substr(start_index, index - start_index);
                tokens.push({is_decimal ? type_double : type_int, slice});
            }
                // String
            else if (contents[index] == '"') {
                index++;
                while (contents[index] != '"') index++;

                string slice = contents.substr(start_index + 1, index - start_index - 1);
                tokens.push({type_string, slice});
                index++;
            }
                // Operator
            else if (operator_token_map.count(string(1, contents[index])) > 0 ||
                     operator_token_map.count(string(2, contents[index])) > 0) {
                if (index < maxSize && operator_token_map.count(contents.substr(index, 2)) > 0) {
                    string slice = contents.substr(index, 2);
                    tokens.push({operator_token_map[slice], slice});
                    index += 2;
                } else {
                    tokens.push({operator_token_map[string(1, contents[index])],
                                 string(1, contents[index])});
                    index++;
                }
            } else index++;
        }
    }

}