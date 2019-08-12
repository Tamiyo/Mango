# Contains functions that are involved in the CLR algorithm

from algorithm_helpers import create_itemset, print_itemset, combine_item
from algorithm_steps import FIRST_S
from settings import GRAMMAR, TERMINALS, NONTERMINALS, INDEXED_GRAMMAR, GRAMMAR_SYMBOLS


def CLOSURE(I):
    itemAdded = True
    while itemAdded:
        itemAdded = False
        for item in I:
            if item['B'] in NONTERMINALS:
                for production in GRAMMAR[item['B']]:
                    for b in FIRST_S([item['b'] + ' ' + item['t'] if item['b'] != '' else item['t']], GRAMMAR):
                        if b in TERMINALS:
                            titem = create_itemset(item['B'],
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
            J.append(create_itemset(item['A'],
                                    item['a'] + ' ' + X,
                                    item['b'][:item['b'].find(' ')] if item['b'].find(
                                        ' ') != -1 else item['b'],
                                    item['b'][item['b'].find(
                                        ' ') + 1:] if item['b'].find(' ') != -1 else '',
                                    item['t']))
    cJ = CLOSURE(J)
    return cJ


ACTION = {}


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
    global ACTION
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
                print('Searching for rule', combine_item(item),
                      'R', INDEXED_GRAMMAR[combine_item(item)][0])
                ACTION[state, item['t']] = 'R{0}'.format(
                    INDEXED_GRAMMAR[combine_item(item)][0])
            if IDENTITY in C[state]:
                ACTION[state, 'TS_END_OF_FILE'] = 'ACCEPT'
    pass


def generate_parse_table():
    print('Calculating ITEMS(G, F, FL)')

    C = [CLOSURE([{
        'A': 'NTS_MANGO',
        'a': '',
        'B': 'NTS_STATEMENT_SUITE',
        'b': '',
        't': 'TS_END_OF_FILE'
    }])]

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
