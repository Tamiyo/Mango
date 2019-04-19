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

	void gen();

private:
	keywords::Symbols** create_grammar_map();
	std::vector<std::string> split_string(std::string);
};

