//
// Created by Matt on 8/11/2019.
//

#ifndef MANGO_V2_CPP_LEXER_H
#define MANGO_V2_CPP_LEXER_H

#include <iostream>
#include <string>
#include <vector>

#include "core.h"

using std::string;
using std::vector;
using std::cout;
using std::endl;

struct Lexer {
public:
    string input;

    vector<LexerResult> lex() {
        printf("Being Lex!\n");

        vector<LexerResult> tokens;
        auto input_iter = input.begin();

        int token_char_value;
        while (*input_iter) {
            token_char_value = *input_iter;

            // 0 ... 9
            if (token_char_value >= 48 && token_char_value <= 57) {
                string token;
                bool has_decimal = false;

                while (*input_iter) {
                    token_char_value = *input_iter;

                    // if the token is 0 ... 9
                    if (token_char_value >= 48 && token_char_value <= 57) {
                        token.push_back(*input_iter);
                        input_iter++;
                    }
                        // if the token is '.'
                    else if (token_char_value == 46) {
                        if (!has_decimal) {
                            has_decimal = true;
                            token.push_back(*input_iter);
                            input_iter++;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                LexerResult result;
                if (has_decimal) {
                    result = {token, PrimitiveType::Float, TokenType::Term};
                } else {
                    result = {token, PrimitiveType::Integer, TokenType::Term};
                }
                tokens.push_back(result);
                cout << "Found Number!: " << token << endl;
            }

            // a ... z | A ... Z
            if ((token_char_value >= 65 && token_char_value <= 90) ||
                (token_char_value >= 97 && token_char_value <= 122)) {
                string token;
                bool past_first = false;

                while (*input_iter) {
                    token_char_value = *input_iter;

                    // if the token is a ... z | A ... Z
                    if ((token_char_value >= 65 && token_char_value <= 90) ||
                        (token_char_value >= 97 && token_char_value <= 122)) {
                        past_first = true;
                        token.push_back(*input_iter);
                        input_iter++;
                    }
                        // if the token is 0 ... 9 || _
                    else if ((token_char_value >= 48 && token_char_value <= 57) || token_char_value == 95) {
                        if (past_first) {
                            token.push_back(*input_iter);
                            input_iter++;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                LexerResult result;
                TokenType token_type = identifier_to_enum(token);
                if (token_type == TokenType::None) {
                    result = {token, PrimitiveType::Null, TokenType::Identifier};
                } else {
                    result = {token, PrimitiveType::Null, token_type};
                }
                tokens.push_back(result);
                cout << "Found Identifier!: " << token << endl;
            }

            // "
            if (token_char_value == 34) {
                string token;
                bool inside_quotes = false;

                while (*input_iter) {
                    token_char_value = *input_iter;

                    // if the token is "
                    if (token_char_value == 34) {
                        if (!inside_quotes) {
                            inside_quotes = true;
                            input_iter++;
                        } else {
                            input_iter++;
                            break;
                        }
                    } else {
                        token.push_back(*input_iter);
                        input_iter++;
                    }
                }

                LexerResult result = {token, PrimitiveType::String, TokenType::Term};
                tokens.push_back(result);
                cout << "Found String!: " << token << endl;
            }

                // \t \s
            else if (token_char_value == 9 || token_char_value == 32) {
                input_iter++;
            }
                // _
            else {
                string token;
                int length = 0;
                bool previous = false;

                while (*input_iter) {
                    token_char_value = *input_iter;

                    // if the token is  '+' | '-' | '*' | '/' | '%' | '^' | '!' | '{' | '}' | '(' | ')' | ',' | ':' | ';' | '\n' | '$' | '=' | '>' | '<' | '.'
                    if ((token_char_value >= 36 && token_char_value <= 47) ||
                        (token_char_value >= 58 && token_char_value <= 62) || token_char_value == 10 ||
                        token_char_value == 94 || token_char_value == 123 || token_char_value == 125) {
                        string temp = token;
                        temp.push_back(*input_iter);
                        if (symbol_to_enum(temp) == TokenType::None) {
                            break;
                        } else {
                            token.push_back(*input_iter);
                        }
                        input_iter++;
                    } else {
                        break;
                    }
                }

                TokenType token_type = symbol_to_enum(token);
                LexerResult result = {token, PrimitiveType::Null, token_type};

                cout << "Found Symbol!: " << token << endl;

                tokens.push_back(result);
            }
        }

        return tokens;
    }
};

#endif //MANGO_V2_CPP_LEXER_H
