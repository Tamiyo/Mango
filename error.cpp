#include "error.hpp"

namespace mango
{
	void error(int line, const string& message)
	{
		report(line, "", message);
	}

	void error(const token& token, const string& message)
	{
		if (token.type == END_OF_FILE)
		{
			report(token.line, " at end", message);
		}
		else
		{
			report(token.line, " at '" + token.lexeme + "'", message);
		}
	}

	void report(int line, const string& where, const string& message)
	{
		cout << "[line " << line << "] Error " << where << ": " << message << endl;
		hadError = true;
	}
}
