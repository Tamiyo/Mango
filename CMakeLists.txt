cmake_minimum_required(VERSION 3.14)
project(mango_v2_cpp)

set(CMAKE_CXX_STANDARD 14)

add_executable(mango_v2_cpp main.cpp core.h parser_tree.h lexer.h parser.h common.h)