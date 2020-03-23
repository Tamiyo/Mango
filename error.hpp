#ifndef MANGO_LOX_ERROR_HPP
#define MANGO_LOX_ERROR_HPP

#include <iostream>
#include <string>
#include <utility>

#include "token.hpp"

using namespace std;

namespace mango
{
	void error(int, const string&);

	void error(const token&, const string&);

	void report(int, const string&, const string&);

	static bool hadError = false;
}

#endif //MANGO_LOX_ERROR_HPP
