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
    production_set = [x.strip() for x in open(r'C:\Users\Matt\Documents\Mango\generator\grammar.mg').readlines()]
    for production in production_set:
        if len(production) > 2:
            lhs = production[:production.find('->') - 1]
            rhs = production[production.find('->') + 3:]

            if index == 0:
                INDEXED_GRAMMAR[lhs, rhs] = ('', 0)
                index += 1
            else:
                INDEXED_GRAMMAR[lhs, rhs] = (index, len(rhs.split(' ')) * 2)
                index += 1

            # Add to Grammar
            if lhs in GRAMMAR.keys():
                GRAMMAR[lhs] += [rhs]
            else:
                GRAMMAR[lhs] = [rhs]

            # Add to Nonterminals
            if lhs not in NONTERMINALS:
                NONTERMINALS += [lhs]
                GRAMMAR_SYMBOLS += [lhs]

            # Add to terminals
            rhs_tokens = rhs.split(' ')
            for token in rhs_tokens:
                if token not in TERMINALS:
                    TERMINALS += [token]
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
            else:
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
            'A': 'NTS_SP',
            'a': 'NTS_S',
            'B': '',
            'b': '',
            't': 'TS_EOF'
        }
        # Creating the table matrix
        ACTION = {}
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
        for _, item in INDEXED_GRAMMAR.items():
            print(_, item)
            if item[0] != '':
                out += '\tGOTO[{0}] = {1};\n'.format(item[0] + 1, item[1])
            else:
                out += '\tGOTO[1] = 2;\n'
        for keys, item in ACTION.items():
            out += '\tACTION[{0}][tokens::{1}] = \"{2}\";\n'.format(keys[0], keys[1], item)
        out += '}\n\n' \
               'void mgparser::ppeval() {\n' \
               '\ttokens::Symbols lexeme = tokens::TS_EMPTY;\n' \
               '\tbool reduced = false;\n' \
               '\twhile(lexeme != tokens::TS_EOF) {\n' \
               '\t\tif (!reduced) {\n' \
               '\t\t\tlexeme = lexer->lltoken().second;\n' \
               '\t\t}' \
               '\t\tint state = ss.top().state;\n' \
               '\t\tss.pop();\n' \
               '\t\tif (ACTION[state][lexeme].substr(0,1) == \"S\") {\n' \
               '\t\t\tss.push(stack_symbol{lexeme});\n' \
               '\t\t\tss.push(stack_symbol{state});\n' \
               '\t\t}\n' \
               '\t\telse if(ACTION[state][lexeme].substr(0,1) == \"R\") {\n' \
               '\t\t\tint pop_amnt = GOTO[atoi(ACTION[state][lexeme].substr(1))];\n' \
               '\t\t\tint pop_cur = 0;\n' \
               '\t\t\twhile (pop_cur++ < pop_amnt) {\n' \
               '\t\t\t\tss.pop();\n' \
               '\t\t\t}\n' \
               '\t\t\tss.push(stack_symbol{atoi(ACTION[state][lexeme].substr(1))});\n' \
               '\t\telse if (ACTION[state][lexeme] == \"ACCEPT\") {\n' \
               '\t\t\tcout << \"ACCEPTED BY PARSER\" << endl;\n' \
               '\t\t}\n' \
               '\t}\n' \
               '}\n'
        print(out)

        # f = open('../../parser/mgparser.cpp')
        # f.write(out)
        # f.close()

    C = [CLOSURE([{
        'A': 'NTS_SP',
        'a': '',
        'B': 'NTS_S',
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
