#include "token.hpp"

namespace mango
{
	token::token(token_type type, const string& lexeme, const variable& literal, int line) : type(type), lexeme(lexeme),
	                                                                                         literal(literal),
	                                                                                         line(line)
	{
	}

	string stringify(const variable& v)
	{
		string o;

		int t0;
		double t1;
		string t2;
		bool t3;

		switch (v.index())
		{
		case 0:
			t0 = get<0>(v);
			o = to_string(t0);
			return o;
		case 1:
			t1 = get<1>(v);
			o = to_string(t1);
			return o;
		case 2:
			t2 = get<2>(v);
			o = t2;
			return o;
		case 3:
			t3 = get<3>(v);
			o = to_string(t3);
			return o;
		default:
			cout << "Internal Error : Failed to convert std::variant to string" << endl;
			return nullptr;
		}
	}
}
