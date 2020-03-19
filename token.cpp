#include "token.hpp"

namespace mango {
    token::token(token_type type, string lexeme, variable literal, int line) : type(type), lexeme(move(lexeme)),
                                                                               literal(move(literal)),
                                                                               line(line) {}
}