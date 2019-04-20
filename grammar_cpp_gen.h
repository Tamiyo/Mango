#pragma once

#include<fstream>
#include<iostream>
#include<string>
#include<vector>
#include<algorithm>

#include "sys/keywords.h"

class grammar_cpp_gen
{
public:
	grammar_cpp_gen();
	virtual ~grammar_cpp_gen();

	void gen_string_to_enum_array();
	void gen_parse_table();

private:
	keywords* keys;
	std::vector<std::string> split_string(std::string);
	void create_ll1_parser_files(std::map<keywords::Symbols, std::map<keywords::Symbols, int>>);
};

