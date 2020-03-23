#include "lexer.hpp"

namespace mango {
    lexer::lexer(string source) : source(move(source)) {
    }

    vector<token> lexer::scan_tokens() {
        while (!is_at_end()) {
            start = current;
            scan_token();
        }

        tokens.emplace_back(token(END_OF_FILE, "", 0, line));
        return tokens;
    }

    void lexer::scan_token() {
        char c = advance();
        switch (c) {
            case '(':
                add_token(OPEN_PAREN);
                break;
            case ')':
                add_token(CLOSE_PAREN);
                break;
            case '{':
                add_token(OPEN_CURLY);
                break;
            case '}':
                add_token(CLOSE_CURLY);
                break;
            case '[':
                add_token(OPEN_SQUARE);
                break;
            case ']':
                add_token(CLOSE_SQUARE);
                break;
            case '+':
                add_token(ADD);
                break;
            case '-':
                add_token(SUB);
                break;
            case '*':
                add_token(MUL);
                break;
            case '/':
                add_token(match('/') ? IDIV : DIV);
                break;
            case '%':
                add_token(MOD);
                break;
            case '^':
                add_token(POW);
                break;
            case '=':
                add_token(match('=') ? EQUAL_EQUAL : EQUAL);
                break;
            case '!':
                add_token(match('=') ? BANG_EQUAL : BANG);
            case '<':
                add_token(match('=') ? LTE : LT);
                break;
            case '>':
                add_token(match('=') ? GTE : GT);
                break;
            case '.':
                add_token(DOT);
                break;
            case ',':
                add_token(COMMA);
                break;
            case ':':
                add_token(COLON);
                break;
            case ';':
                add_token(SEMICOLON);
                break;
            case '_':
                add_token(UNDERSCORE);
                break;
            case '#':
                while (peek() != '\n' && !is_at_end()) advance();
                break;
            case '@':
                add_token(DEFINITION);
                break;
            case '\n':
                line++;
                break;
            case ' ':
            case '\r':
            case '\t':
                break;
            case '\'':
            case '\"':
                process_string(c);
                break;
            default:
                if (isdigit(c)) {
                    process_number();
                } else if (isalpha(c)) {
                    process_identifier();
                } else {
                    error(line, "Unexpected character " + to_string(c) + ".");
                }
                break;
        }
    }

    char lexer::advance() {
        current++;
        return source.at(current - 1);
    }

    void lexer::add_token(token_type type) {
        add_token(type, {-1});
    }

    void lexer::add_token(token_type type, const variable &literal) {
        string text = source.substr(start, (current - start));
        tokens.emplace_back(token(type, text, literal, line));
    }

    bool lexer::is_at_end() {
        return current >= source.size();
    }

    bool lexer::match(char expected) {
        if (is_at_end()) return false;
        if (source.at(current) != expected) return false;

        current++;
        return true;
    }

    char lexer::peek() {
        if (is_at_end()) return '\0';
        return source.at(current);
    }


    char lexer::peek_next() {
        if (current + 1 >= source.length()) return '\0';
        return source.at(current + 1);
    }

    void lexer::process_string(char delim) {
        while (peek() != delim && !is_at_end()) {
            if (peek() == '\n') line++;
            advance();
        }

        // Unterminated string
        if (is_at_end()) {
            error(line, "Unterminated string.");
            return;
        }

        // Remove closing delim
        if (peek() != delim) {
            error(line, "Incorrectly terminated string.");
            return;
        }

        advance();

        // Trim the quotes
        string value = source.substr(start + 1, (current - start - 2));
        add_token(STRING, value);
    }

    void lexer::process_number() {
        bool is_float = false;
        while (isdigit(peek())) advance();

        if (peek() == '.' && isdigit(peek_next())) {
            is_float = true;

            // Consume the '.'
            advance();

            while (isdigit(peek())) advance();
        }

        string sub = source.substr(start, (current - start));

        if (is_float) {
            double d = stod(sub);
            add_token(NUMBER, d);
        } else {
            int i = stoi(sub);
            add_token(NUMBER, i);
        }
    }

    void lexer::process_identifier() {
        while (isalnum(peek())) advance();

        string text = source.substr(start, (current - start));
        token_type type = keywords.find(text) != keywords.end() ? keywords[text] : IDENTIFIER;

        add_token(type);
    }
}
