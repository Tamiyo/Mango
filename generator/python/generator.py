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
    TERMINALS += ['TS_EOF']

    index = 0
    production_set = [x.strip() for x in open(r'D:\Documents\mango_cl\generator\grammar.mg').readlines()]
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

    tGrammar = copy.deepcopy(GRAMMAR)
    FIRST_SET = FIRST(tGrammar)

    tGrammar = copy.deepcopy(GRAMMAR)
    tFirst = copy.deepcopy(FIRST_SET)
    FOLLOW_SET = FOLLOW(tGrammar, tFirst)

    ITEMS(GRAMMAR, FIRST_SET, FOLLOW_SET)
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

    follow_productions[NONTERMINALS[0]] = ['TS_EOF']
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


def ITEMS(GRAMMAR, FIRST_SET, FOLLOW_SET):
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
            'a': 'NTS_STATEMENTS',
            'B': '',
            'b': '',
            't': 'TS_EOF'
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
                    ACTION[state, 'TS_EOF'] = 'ACCEPT'
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
        for key, item in INDEXED_GRAMMAR.items():
            if item[0] != '':
                out += '\tGOTO[{0}] = '.format(item[0])
                out += '{'
                out += 'tokens::{0}, {1}'.format(key[0], item[1])
                out += '};\n'
            else:
                out += '\tGOTO[1] = {tokens::' + key[0] + ', 2};\n'
        for keys, item in ACTION.items():
            out += '\tACTION[{0}][tokens::{1}] = \"{2}\";\n'.format(keys[0], keys[1], item)
        out += '}\n\n' \
               'void mgparser::ppeval() {\n' \
               '\tauto token = lexer->lltoken();\n' \
               '\twhile (true) {\n' \
               '\t\tauto s = ss.top();\n' \
               '\t\tif (token.second >= tokens::TS_STRING && token.second <= tokens::MYSBL_END) {\n' \
               '\t\t\tif (ACTION[s.state][token.second].substr(0, 1) == "S") {\n' \
               '\t\t\t\tss.push({token.second});\n' \
               '\t\t\t\tss.push({atoi(ACTION[s.state][token.second].substr(1).c_str())});\n' \
               '\t\t\t\ttoken = lexer->lltoken();\n' \
               '\t\t\t} else if (ACTION[s.state][token.second] == "ACCEPT") {\n' \
               '\t\t\t\tcout << "Parse Accepted" << endl;\n' \
               '\t\t\t\tbreak;\n' \
               '\t\t\t} else if (ACTION[s.state][token.second].substr(0, 1) == "R") {\n' \
               '\t\t\t\tauto g = GOTO[atoi(ACTION[s.state][token.second].substr(1).c_str())];\n' \
               '\t\t\t\tfor (int i = 0; i < g.second; i++) { ss.pop(); }\n' \
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

        print(out)

        f = open('../../parser/mgparser.cpp', 'w+')
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
                         'TS_NEWLINE',
                         'TS_EMPTY',
                         'TS_EOF',

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
    f2 = open('../../tokens/tokens.h', 'w+')
    f2.write(out2)
    f2.close()

    C = [CLOSURE([{
        'A': 'NTS_MANGO',
        'a': '',
        'B': 'NTS_STATEMENTS',
        'b': '',
        't': 'TS_EOF'
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


if __name__ == '__main__':
    main()
