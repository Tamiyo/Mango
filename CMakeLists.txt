cmake_minimum_required(VERSION 3.15)
project(mango_lox)

set(CMAKE_CXX_STANDARD 20)

add_executable(mango_lox main.cpp token.cpp token.hpp lexer.cpp lexer.hpp error.cpp error.hpp syntax_tree.cpp syntax_tree.hpp)