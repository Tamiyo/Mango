cmake_minimum_required(VERSION 3.13)
project(mango_cl)

set(CMAKE_CXX_STANDARD 14)

add_executable(mango_cl main.cpp generator/generator_parser.cpp generator/generator_parser.h tokens/tokens.cpp tokens/tokens.h lexer/mglexer.cpp lexer/mglexer.h parser/mgparser.cpp parser/mgparser.h parse_tree/expr.cpp parse_tree/expr.h)