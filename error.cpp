#include "error.hpp"

namespace mango {
    void error(int line, string message) {
        report(line, "", move(message));
    }

    void report(int line, const string &where, const string &message) {
        cout << "[line " << line << "] Error " << where << ": " << message << endl;
        hadError = true;
    }
}