#ifndef MANGO_LOX_LEXER_HPP
#define MANGO_LOX_LEXER_HPP

#include <string>
#include <vector>
#include <utility>

#include "error.hpp"
#include "token.hpp"

using namespace std;

namespace mango
{
	class lexer
	{
	public:
		explicit lexer(string);

		vector<token> scan_tokens();

	private:

		void scan_token();

		char advance();

		void add_token(token_type);

		void add_token(token_type, const variable&);

		bool is_at_end();

		bool match(char);

		char peek();

		char peek_next();

		void process_string(char);

		void process_number();

		void process_identifier();

		string source;
		vector<token> tokens;
		int start = 0;
		int current = 0;
		int line = 1;
	};
}

#endif //MANGO_LOX_LEXER_HPP
