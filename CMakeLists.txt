cmake_minimum_required(VERSION 3.13)
project(clionlang)

set(CMAKE_CXX_STANDARD 14)

add_executable(clionlang main.cpp lex/mglex.cpp lex/mglex.h sys/keywords.cpp sys/keywords.h parser/parser.cpp parser/parser.h)