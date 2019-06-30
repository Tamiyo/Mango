import copy
from collections import OrderedDict

NONTERMINALS = []
TERMINALS = []
GRAMMAR = {}
INDEXED_GRAMMAR = {}
FIRST_SET = {}
FOLLOW_SET = {}
GRAMMAR_SYMBOLS = []
C = {}


def main():
    global TERMINALS, NONTERMINALS, GRAMMAR, GRAMMAR_SYMBOLS, INDEXED_GRAMMAR
    TERMINALS += ['TS_END_OF_FILE']

    index = 0
    production_set = [x.strip() for x in open(r'../grammar.mg').readlines()]
    for production in production_set:
        if len(production) > 2 and production[0] != "#":
            lhs = production[:production.find('->') - 1]
            rhs = production[production.find('->') + 3:]

            if index == 0:
                INDEXED_GRAMMAR[lhs, rhs] = ('', 0)
                index += 1
            else:
                INDEXED_GRAMMAR[lhs, rhs] = (index, len(rhs.split(' ') * 2))
                index += 1

            # Add to Grammar
            if lhs in GRAMMAR.keys():
                GRAMMAR[lhs] += [rhs]
            else:
                GRAMMAR[lhs] = [rhs]

            # Add to Nonterminals
            if lhs not in NONTERMINALS:
                NONTERMINALS += [lhs]
                if lhs not in GRAMMAR_SYMBOLS:
                    GRAMMAR_SYMBOLS += [lhs]

            # Add to terminals
            rhs_tokens = rhs.split(' ')
            for token in rhs_tokens:
                if token not in TERMINALS:
                    TERMINALS += [token]
                    if token not in GRAMMAR_SYMBOLS:
                        GRAMMAR_SYMBOLS += [token]

    for key, item in INDEXED_GRAMMAR.items():
        print(key)
    tGrammar = copy.deepcopy(GRAMMAR)
    FIRST_SET = FIRST(tGrammar)

    tGrammar = copy.deepcopy(GRAMMAR)
    tFirst = copy.deepcopy(FIRST_SET)
    FOLLOW_SET = FOLLOW(tGrammar, tFirst)

    # RULES = TRANSITIONS(INDEXED_GRAMMAR)
    ITEMS(GRAMMAR, FIRST_SET, FOLLOW_SET, RULES=None)
    return


def FIRST(G):
    print('Calculating FIRST_SET(G)')
    first_productions = {}

    for key, item in G.items():
        rhs = item
        while len(rhs) > 0:
            back = rhs.pop()
            first = back[:back.find(' ')] if back.find(' ') != -1 else back

            if first[0:2] == 'TS':
                if key in first_productions.keys():
                    first_productions[key] += [first]
                else:
                    first_productions[key] = [first]
            elif first != '':
                for production in G[first]:
                    rhs.append(production)
    print('first_productions:', first_productions, end='\n\n')
    return first_productions


def FIRST_S(rhs, G):
    first_productions = []
    while len(rhs) > 0:
        back = rhs.pop()
        first = back[:back.find(' ')] if back.find(' ') != -1 else back

        if first[0:2] == 'TS':
            first_productions += [first]
        elif first != '':
            for production in G[first]:
                rhs.append(production)
    return first_productions


def FOLLOW(G, FIRST_SET):
    global NONTERMINALS

    print('Calculating FOLLOW(G)')
    follow_productions = {}

    follow_productions[NONTERMINALS[0]] = ['TS_END_OF_FILE']
    changed = True

    while changed:
        changed = False
        for non_terminal in NONTERMINALS:
            for key, production in G.items():
                for prod in production:
                    tokens = prod.split(' ')
                    index = 0
                    for token in tokens:
                        if token == non_terminal:
                            if index == len(tokens) - 1:
                                for fprod in (follow_productions[key] if key in follow_productions.keys() else []):
                                    if fprod not in (follow_productions[
                                        non_terminal] if non_terminal in follow_productions.keys() else []):
                                        if non_terminal in follow_productions.keys():
                                            follow_productions[non_terminal] += [fprod]
                                        else:
                                            follow_productions[non_terminal] = [fprod]
                                        changed = True
                            elif token[0:2] == "NT":
                                for fprod in (FIRST_SET[token] if token in FIRST_SET.keys() else []):
                                    if fprod not in (follow_productions[
                                        non_terminal] if non_terminal in follow_productions.keys() else []):
                                        if non_terminal in follow_productions.keys():
                                            follow_productions[non_terminal] += [fprod]
                                        else:
                                            follow_productions[non_terminal] = [fprod]
                                        changed = True
                            elif token[0:2] == "TS" and token not in (
                                    follow_productions[
                                        non_terminal] if non_terminal in follow_productions.keys() else []):
                                if non_terminal in follow_productions.keys():
                                    follow_productions[non_terminal] += [token]
                                else:
                                    follow_productions[non_terminal] = [token]
                                changed = True
                    index += 1
    print('follow_productions', follow_productions, end='\n\n')
    return follow_productions


def ITEMS(GRAMMAR, FIRST_SET, FOLLOW_SET, RULES):
    print('Calculating ITEMS(G, F, FL)')

    def __createitemset__(A, a, B, b, t):
        return {
            'A': A.strip(),
            'a': a.strip(),
            'B': B.strip(),
            'b': b.strip(),
            't': t.strip(),
        }

    def combine_item(I):
        return I['A'], I['a'].strip() + I['B'].strip() + I['b'].strip()

    def print_itemset(C):
        iter = 0
        for set in C:
            print('I{0}'.format(iter))
            for i in set:
                print('\t[{0} -> {1} * {2} {3}, {4}]'.format(i['A'], i['a'], i['B'], i['b'], i['t']))
            iter += 1

    def CLOSURE(I):
        itemAdded = True
        while itemAdded:
            itemAdded = False
            for item in I:
                if item['B'] in NONTERMINALS:
                    for production in GRAMMAR[item['B']]:
                        for b in FIRST_S([item['b'] + ' ' + item['t'] if item['b'] != '' else item['t']], GRAMMAR):
                            if b in TERMINALS:
                                titem = __createitemset__(item['B'],
                                                          '',
                                                          production[0:production.find(' ')] if production.find(
                                                              ' ') != -1 else production,
                                                          production[production.find(' ') + 1:] if production.find(
                                                              ' ') != -1 else '',
                                                          b)
                                if titem not in I:
                                    I.append(titem)
                                    itemAdded = True
        return I

    def GOTO(I, X):
        J = []
        for item in I:
            if item['B'] == X:
                J.append(__createitemset__(item['A'],
                                           item['a'] + ' ' + X,
                                           item['b'][:item['b'].find(' ')] if item['b'].find(' ') != -1 else item['b'],
                                           item['b'][item['b'].find(' ') + 1:] if item['b'].find(' ') != -1 else '',
                                           item['t']))
        cJ = CLOSURE(J)
        # print_itemset(cJ)
        return cJ

    def create_CLR_parsing_table(C):
        print('\nCreating CLR Parsing Table')
        # [A -> a*Bb, t]
        IDENTITY = {
            'A': 'NTS_MANGO',
            'a': 'NTS_STATEMENT_SUITE',
            'B': '',
            'b': '',
            't': 'TS_END_OF_FILE'
        }
        # Creating the table matrix
        ACTION = {}
        print('len(C):', len(C))
        for state in range(0, len(C)):
            for item in C[state]:
                GTC = GOTO(C[state], item['B'])
                if GTC in C and item['B'] != '':
                    j = C.index(GTC)
                    if item['B'][0:2] == 'TS':
                        ACTION[state, item['B']] = 'S{0}'.format(j)
                    else:
                        ACTION[state, item['B']] = '{0}'.format(j)
                if item['B'] == '' and item['b'] == '' and item['a'] != '':
                    print('Searching for rule', combine_item(item), 'R', INDEXED_GRAMMAR[combine_item(item)][0])
                    ACTION[state, item['t']] = 'R{0}'.format(INDEXED_GRAMMAR[combine_item(item)][0])
                if IDENTITY in C[state]:
                    ACTION[state, 'TS_END_OF_FILE'] = 'ACCEPT'
        for key, item in ACTION.items():
            print(key, item)
        # Creating the C file
        out = '#include \"mgparser.h\"\n' \
              '#include <string>\n' \
              '#include <iostream>\n\n' \
              'mgparser::mgparser(const char* file) {\n' \
              '\tlexer = new mglexer(file);\n' \
              '\tss.push(stack_symbol{0});\n'
        # Note for efficiency we could ignore the symbols (add later)
        # Note also that we need to handle GOTO better (there is an error in the implemnetation rn)
        for n in NONTERMINALS:
            print(''.join([str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in n.split("_")]) + ",")
        for key, item in INDEXED_GRAMMAR.items():
            if item[0] != '':
                out += '\tGOTO[{0}] = '.format(item[0])
                out += '{'
                out += 'tokens::{0}, {1}'.format(key[0], item[1])
                out += '};\n'
                ttype = ''.join([str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in key[0].split("_")])
                print("self.goto.insert({}, GotoNode".format(item[0]) + "{" + "token_type: TokenType::{}, value: {}".format(ttype, str(int(int(item[1])/2))) + "});")
            else:
                out += '\tGOTO[1] = {tokens::' + key[0] + ', 2};\n'
        for keys, item in ACTION.items():
            out += '\tACTION[{0}][tokens::{1}] = \"{2}\";\n'.format(keys[0], keys[1], item)
            ttype = ''.join([str(x).replace("NTS", "").replace("TS", "").lower().capitalize() for x in keys[1].split("_")])
            if item[0] == "S":
                print("self.action.insert(({}, &TokenType::{}), ActionNode".format(keys[0], ttype) + "{" + "action: ParserAction::{}, value: {}".format("Shift", item[1:]) + "});")
            elif item[0] == "R":
                print("self.action.insert(({}, &TokenType::{}), ActionNode".format(keys[0], ttype) + "{" + "action: ParserAction::{}, value: {}".format("Reduce", item[1:]) + "});")
            elif item == "ACCEPT":
                print("self.action.insert(({}, &TokenType::{}), ActionNode".format(keys[0], ttype) + "{" + "action: ParserAction::{}, value: {}".format("Accept", -1) + "});")
            else:
                print("self.action.insert(({}, &TokenType::{}), ActionNode".format(keys[0], ttype) + "{" + "action: ParserAction::{}, value: {}".format("Goto", item) + "});")
        out += '}\n\n' \
               'void mgparser::ppeval() {\n' \
               '\tauto token = lexer->lltoken();\n' \
               '\twhile (true) {\n' \
               '\t\tauto s = ss.top();\n' \
               '\t\tif (token.second >= tokens::TS_STRING && token.second <= tokens::MYSBL_END) {\n' \
               '\t\t\tif (ACTION[s.state][token.second].substr(0, 1) == "S") {\n' \
               '\t\t\t\tif (token.second == tokens::TS_TERM) { strs.push(token.first); }\n' \
               '\t\t\t\tss.push({token.second});\n' \
               '\t\t\t\tss.push({atoi(ACTION[s.state][token.second].substr(1).c_str())});\n' \
               '\t\t\t\ttoken = lexer->lltoken();\n' \
               '\t\t\t} else if (ACTION[s.state][token.second] == "ACCEPT") {\n' \
               '\t\t\t\tnode NTS_STATEMENT_SUITE = ns.top();\n' \
               '\t\t\t\tns.pop();\n' \
               '\t\t\t\tns.push(NTS_MANGO_NTS_STATEMENT_SUITE(NTS_STATEMENT_SUITE));\n' \
               '\t\t\t\tcout << "Parse Accepted" << endl;\n' \
               '\t\t\t\tbreak;\n' \
               '\t\t\t} else if (ACTION[s.state][token.second].substr(0, 1) == "R") {\n' \
               '\t\t\t\tauto g = GOTO[atoi(ACTION[s.state][token.second].substr(1).c_str())];\n' \
               '\t\t\t\tfor (int i = 0; i < g.second; i++) { ss.pop(); }\n' \
               '' + '' + '' \
                         '\t\t\t\ts = ss.top();\n' \
                         '\t\t\t\tss.push(stack_symbol{g.first});\n' \
                         '\t\t\t\tss.push(stack_symbol{atoi(ACTION[s.state][g.first].c_str())});\n' \
                         '\t\t\t} else {\n' \
                         '\t\t\t\tcout << "Parse Error" << endl;\n' \
                         '\t\t\t\tbreak;\n' \
                         '\t\t\t}\n' \
                         '\t\t} else {\n' \
                         '\t\t\ttoken = lexer->lltoken();\n' \
                         '\t\t}\n' \
                         '\t}\n' \
                         '}\n'

        f = open('mgparser.cpp', 'w')
        f.write(out)
        f.close()

    hardcoded_symbols = ['TS_STRING',
                         'TS_IDENTIFIER',
                         'TS_FLOAT',
                         'TS_INT',
                         'TS_TERM',

                         'TS_IF',
                         'TS_ELIF',
                         'TS_ELSE',

                         'TS_LCB',
                         'TS_RCB',
                         'TS_LPAREN',
                         'TS_RPAREN',

                         'TS_OPERATOR_EQUALS',
                         'TS_OPERATOR_ADD',
                         'TS_OPERATOR_SUB',
                         'TS_OPERATOR_MUL',
                         'TS_OPERATOR_DIV',
                         'TS_OPERATOR_EXP',

                         'TS_COLON',
                         'TS_COMMA',

                         'TS_FOR',
                         'TS_WHILE',
                         'TS_DEFINE',

                         'TS_OPERATOR_LT',
                         'TS_OPERATOR_LTE',
                         'TS_OPERATOR_GT',
                         'TS_OPERATOR_GTE',
                         'TS_OPERATOR_DOUBLE_EQUALS',
                         'TS_OPERATOR_TRIPLE_EQUALS',

                         'TS_OPERATOR_NEG',
                         'TS_OPERATOR_NONNULL',
                         'TS_SPACE',
                         'TS_SYMBOL_NEWLINE',
                         'TS_EMPTY',
                         'TS_END_OF_FILE',

                         'NTS_OPERATOR']
    out2 = '#ifndef MANGO_CL_TOKENS_H\n' \
           '#define MANGO_CL_TOKENS_H\n' \
           '#include<map>\n' \
           '#include<string>\n\n' \
           'class tokens {\n' \
           'public:\n' \
           '\t\ttokens();\n\n' \
           '\t\tenum Symbols {\n'

    for SYMBOL in hardcoded_symbols:
        out2 += '\t\t\t{0},\n'.format(SYMBOL)
    out2 += '\n'
    for SYMBOL in GRAMMAR_SYMBOLS:
        if SYMBOL not in hardcoded_symbols:
            out2 += '\t\t\t{},\n'.format(SYMBOL)

    out2 += '\n\t\t\tMYSBL_END\n\t};\n\n' \
            '\tstd::map<std::string, Symbols> TOKENS;\n' \
            '\tstd::map<int, Symbols> TYPES;\n' \
            '};\n\n' \
            '#endif //MANGO_CL_TOKENS_H'
    # f2 = open('../../tokens/tokens.h', 'w')
    # f2.write(out2)
    # f2.close()

    C = [CLOSURE([{
        'A': 'NTS_MANGO',
        'a': '',
        'B': 'NTS_STATEMENT_SUITE',
        'b': '',
        't': 'TS_END_OF_FILE'
    }])]
    # Note that this should change when the Mango language is implemented

    added = True
    while added:
        added = False
        for I in C:
            for X in GRAMMAR_SYMBOLS:
                GTX = GOTO(I, X)
                if len(GTX) > 0 and GTX not in C:
                    C.append(GTX)
                    added = True

    print_itemset(C)
    create_CLR_parsing_table(C)


def TRANSITIONS(INDEXED_GRAMMAR):
    def create_struct_from_production(key, production):
        header = ""
        out = "\nclass {0}_{1} : public node ".format(key, production.replace(' ', '_'))
        out += "{\npublic:\n"

        tokens = production.split(' ')
        constructor = "\n\texplicit {0}_{1} (".format(key, production.replace(' ', '_'))
        constructor_body = ""
        for token in tokens:
            if token[0:2] == "NT":
                out += "\tnode {0};\n".format(token)
                constructor += "node {0}, ".format(token)
                constructor_body += "\t\tthis->{0} = {1};\n".format(token, token)
            elif token == "TS_TERM" or token == "TS_IDENTIFIER":
                out += "\tnode {0};\n".format(token)
                constructor += "node {0}, ".format(token)
                constructor_body += "\t\tthis->{0} = {1};\n".format(token, token)
        constructor = constructor[:len(constructor) - 2]
        constructor += ") {\n"
        constructor += "\t\tcout << \"{0}\" << endl;\n".format(key)
        constructor += constructor_body
        constructor += '\t}\n'

        out += constructor
        out += "};\n"
        return out

    def create_rule_from_production(index, key, production):
        rule_no = str(index)
        out = "\t\t\t\t\tcase {}: ".format(rule_no)
        out += "{\n"

        tokens = production.split(' ')

        prod = []
        for p in production:
            if p == "TS_TERM" or p == "TS_IDENTIFIER" or p[0:2] == "NT":
                prod.append(p)

        node_creation = "\t\t\t\t\t\tns.push({}_{}(".format(key, production.replace(' ', '_'))
        for token in tokens:
            if token == "TS_TERM":
                out += "\t\t\t\t\t\tnode _{} = TS_IDENTIFIER(strs.top());\n\t\t\t\t\t\tstrs.pop();\n".format(token)
                node_creation += '_{}, '.format(token)
            elif token == "TS_IDENTIFIER":
                out += "\t\t\t\t\t\tnode _{} = TS_TERM(token.first, token.second);\n".format(token)
                node_creation += '_{}, '.format(token)
            elif token[0:2] == "NT":
                out += "\t\t\t\t\t\tnode {} = ns.top();\n\t\t\t\t\t\tns.pop();\n".format(token)
                node_creation += '{}, '.format(token)
        node_creation = node_creation[:len(node_creation) - 2]
        node_creation += "));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\t}\n"
        out += node_creation
        return out

    struct_output = "class node {};\n\n"
    rule_output = ""

    i = 0
    structs = []
    rules = []
    for k, _ in INDEXED_GRAMMAR.items():
        key = k[0]
        production = k[1]
        struct = create_struct_from_production(key, production)
        structs.append(struct)
        rule = create_rule_from_production(i, key, production)
        rules.append(rule)
        i += 1

    myfile = open('../../parse_tree/node.h', 'w')
    myfile.write(
        "#ifndef MANGO_CL_NODE_H\n#define MANGO_CL_NODE_H\n#include<string>\n#include<iostream>\nusing std::string; "
        "using std::cout; using std::endl;\nclass node {};\nclass TS_IDENTIFIER : public node {public:    string "
        "identifier;    explicit TS_IDENTIFIER (string identifier) {        this->identifier = std::move(identifier); "
        "   }};class TS_TERM : public node {public:    string term;    int infered_type;    explicit TS_TERM (string "
        "term, int infered_type) {        this->term = std::move(term);        this->infered_type = infered_type;    "
        "}};\n")
    myfile.write('\n'.join(structs))
    myfile.write('\n#endif //MANGO_CL_NODE_H')
    myfile.close()
    x = '\n\t\t\t\tswitch (atoi(ACTION[s.state][token.second].substr(1).c_str())) {\n'
    # x += '\n'.join(rules)
    x += "\t\t\t\t}\n"
    return x


if __name__ == '__main__':
    main()
