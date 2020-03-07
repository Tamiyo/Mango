#include "lexer.h"

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

        if (isalpha(contents[index])) {
            index++;
            while (isalnum(contents[index]) && index < maxSize) index++;

            string slice = contents.substr(start_index, index - start_index);
            if (keyword_token_map.count(slice) > 0) {
                token tok = keyword_token_map[slice];
                tokens.push({tok, slice});
            } else {
                token tok = token::identifier;
                tokens.push({tok, slice});
            }
        } else if (isdigit(contents[index])) {
            bool is_decimal = false;

            while (index < maxSize) {
                index++;
                if (contents[index] == '.' && !is_decimal) is_decimal = true;
                else if (contents[index] == '.' && is_decimal) break;
                else if (!isdigit(contents[index])) break;
            }
            string slice = contents.substr(start_index, index - start_index);
            tokens.push({token::literal, slice});
        } else if (contents[index] == '"') {
            index++;
            while (contents[index] != '"' && isalnum(contents[index])) index++;

            string slice = contents.substr(start_index, index - start_index);
            tokens.push({token::literal, slice});

        } else if (operator_token_map.count(string(1, contents[index])) > 0) {
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
