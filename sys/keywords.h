//
// Created by Matt on 4/15/2019.
//

#ifndef CLIONLANG_KEYWORDS_H
#define CLIONLANG_KEYWORDS_H

#include<map>
#include<string>

using std::map;
using std::string;

class keywords {
public:
    keywords();

    map<int, const char *> TYPES;
    map<string, int> KEYWORDS;
};


#endif //CLIONLANG_KEYWORDS_H
